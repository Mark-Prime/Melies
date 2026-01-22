use std::{
  fs::{ self, OpenOptions },
  io::Write,
  path::{ Path, PathBuf },
  process::Command,
  thread,
};
use fs_extra::{ copy_items, dir };
use serde_json::{ json, Value };
use vdm::VDM;

fn use_64bit(settings: &Value, tab: i64) -> bool {
  let mut use_64bit = settings["hlae"]["use_64bit"].as_bool().unwrap();

  if tab > 0 {
    let alt_install = settings["alt_installs"][(tab - 1) as usize].clone();

    if alt_install["use_64bit"].as_bool().is_some() {
      use_64bit = alt_install["use_64bit"].as_bool().unwrap();
    }
  }

  return use_64bit;
}

fn get_tf2_path(settings: &Value, tab: i64) -> PathBuf {
  let mut tf2_path = PathBuf::from(settings["tf_folder"].as_str().unwrap())
    .parent()
    .unwrap()
    .to_path_buf();

  if use_64bit(settings, tab) == true {
    tf2_path.push("tf_win64.exe");
  } else {
    tf2_path.push("tf.exe");
  }

  return tf2_path;
}

fn build_launch_options(
  settings: &Value,
  demo_name: &str,
  tab: &str
) -> String {
  let mut launch_options = settings["hlae"]["launch_options"].to_string();
  let tab = tab.parse::<i64>().unwrap();
  let use_64bit = use_64bit(settings, tab);

  if tab > 0 {
    let alt_install = settings["alt_installs"][(tab - 1) as usize].clone();

    let alt_launch_options =
      &alt_install["launch_options"].as_str();

    match alt_launch_options {
      Some(options) => {
        let json_options = json!(options);
        launch_options = json_options.to_string();
      }
      None => {}
    }
  }

  let mut launch_options = launch_options.replace("\"", "").replace("-game tf", "");

  if !launch_options.contains("-insecure") {
    launch_options = format!("{} -insecure", launch_options);
  }

  if demo_name != "" && settings["hlae"]["playdemo"] == true {
    let mut trimmed_name = demo_name.to_string().replace(".dem", "");

    if trimmed_name.chars().nth(0).unwrap() == '\\' {
      trimmed_name.remove(0);
    }

    println!("Playing demo: {}", trimmed_name);
    launch_options = format!("{} +playdemo {}", launch_options, trimmed_name);
  }

  let install = get_install(settings, tab);

  if install != settings["tf_folder"].as_str().unwrap() {
    let tf2_path = PathBuf::from(settings["tf_folder"].as_str().unwrap())
      .parent()
      .unwrap()
      .to_path_buf();
    let tf2_str = tf2_path.to_str().unwrap().to_owned() + "\\";

    let game = install.replace(&tf2_str, "");
    launch_options = format!("{} -game {}", launch_options, game);
  } else if use_64bit == true {
    launch_options = format!("{} -game tf", launch_options);
  }

  if use_64bit != true && !launch_options.contains("-force32bit") {
    launch_options = format!("{} -force32bit", launch_options);
  }

  if settings["hlae"]["novid"] == true && !launch_options.contains("-novid") {
    launch_options = format!("{} -novid", launch_options);
  }

  if settings["hlae"]["borderless"] == true {
    launch_options = launch_options.replace("-fullscreen", "");
    launch_options = format!("{} -windowed -noborder", launch_options);
  }

  launch_options = format!("{} -dxlevel {}", launch_options, settings["hlae"]["dxlevel"]);

  launch_options = format!(
    "{} -w {} -h {}",
    launch_options,
    settings["hlae"]["width"],
    settings["hlae"]["height"]
  );

  launch_options = launch_options.replace("  ", " ");

  return launch_options;
}

pub(crate) fn run_tf2(
  demo_name: &str,
  settings: &Value,
  tab: &str
) -> Value {
  println!("Running TF2");

  let launch_options = build_launch_options(settings, demo_name, tab);

  println!("Launch options: {}", launch_options);

  let tab = tab.parse::<i64>().unwrap();
  
  let tf2_path = get_tf2_path(settings, tab);
  let use_64bit = use_64bit(settings, tab);

  let sparkly_path = settings["hlae"]["sparklyfx_path"].as_str().unwrap();

  let sparkly_path = match use_64bit {
    true =>
      sparkly_path
        .replace("xsdk-base.dll", "xsdk-base64.dll"),
    false =>
      sparkly_path
        .replace("xsdk-base64.dll", "xsdk-base.dll"),
  };

  let hlae = settings["hlae"]["hlae_path"].as_str().unwrap();
  let hlae_dll = hlae.to_owned().replace("HLAE.exe", "AfxHookSource.dll");

  match settings["output"]["method"].as_str().unwrap() {
    "sparklyfx" => {
      let mut args = vec![
        "-customLoader",
        "-noGui",
        "-autoStart",
        "-hookDllPath",
        &sparkly_path,
        "-programPath",
        tf2_path.to_str().unwrap(),
        "-cmdLine",
        launch_options.as_str()
      ];

      if !use_64bit && Path::new(&hlae_dll).exists() {
        args.push("-hookDllPath");
        args.push(&hlae_dll);
      }

      let hlae_cmd = Command::new(hlae).args(args).spawn();

      match hlae_cmd {
        Ok(mut hlae) => {
          hlae.wait().unwrap();
        }
        Err(e) => {
          println!("error: {}", e);
          return json!({
            "status": "error",
            "message": format!("Failed to run HLAE: {}", e)
          });
        }
      }
    }
    "svr" | "svr.mov" | "svr.mp4" => {
      return json!({
            "status": "error",
            "message": format!("Can not run SVR")
          });
    }
    _ => {
      let cmd_res = Command::new(tf2_path)
        .args(launch_options.as_str().split(" ").collect::<Vec<_>>())
        .spawn();

      match cmd_res {
        Ok(mut cmd) => {
          cmd.wait().unwrap();
        }
        Err(e) => {
          println!("error: {}", e);
          return json!({
            "status": "error",
            "message": format!("Failed to run TF2: {}", e)
          });
        }
      }
    }
  }

  return json!({"status": "success"});
}

pub fn is_tf2_running() -> bool {
  let system = sysinfo::System::new_all();

  let old_bits = system.processes_by_name("tf.exe".as_ref()).next().is_some();

  let new_bits = system.processes_by_name("tf_win64.exe".as_ref()).next().is_some();

  old_bits || new_bits
}

fn last_modified_folder(videos_folder: &str) -> Option<std::fs::DirEntry> {
  std::fs
    ::read_dir(videos_folder)
    .expect("Couldn't access local directory")
    .flatten()
    .filter(|f| f.metadata().unwrap().is_dir())
    .max_by_key(|x| x.metadata().unwrap().modified().unwrap())
}

fn has_audio(path: &PathBuf) -> bool {
  let take_path = format!("{}/take0000", path.to_str().unwrap());

  if !std::fs::metadata(take_path.clone()).is_ok() {
    return false;
  }

  for entry in std::fs::read_dir(take_path).unwrap() {
    let entry = entry.unwrap();
    if entry.path().extension().unwrap() == "wav" {
      return true;
    }
  }

  false
}

fn get_demo_name(path: &PathBuf, demo_name: &str) -> String {
  use regex::Regex;

  let name = path.file_name().unwrap().to_str().unwrap();

  let re = Regex::new(r"(?<name>.*)_\d*-\d*(.*)").unwrap();

  let Some(caps) = re.captures(name) else {
    println!("no match!");
    return String::from(demo_name);
  };

  return caps["name"].to_string();
}

pub fn build_new_install(folder_name: &str, settings: &Value) -> Value {
  let new_folder_name = folder_name.to_lowercase().replace(" ", "_");
  let tf_folder = settings["tf_folder"].as_str().unwrap();
  let parent_folder = PathBuf::from(tf_folder).parent().unwrap().to_path_buf();

  let new_tf_folder = format!("{}\\{}\\tf", parent_folder.to_str().unwrap(), new_folder_name);
  let new_custom_folder = format!("{}\\custom", new_tf_folder);

  if !Path::new(&new_custom_folder).exists() {
    std::fs::create_dir_all(&new_custom_folder).unwrap();
  }

  let tf_scripts = &format!("{}\\scripts", tf_folder);
  let tf_cfg = &format!("{}\\cfg", tf_folder);
  let tf_gameinfo = &format!("{}\\gameinfo.txt", tf_folder);

  let mut options = dir::CopyOptions::new();

  options.overwrite = true;

  let mut from_paths = Vec::new();

  from_paths.push(tf_scripts.as_str());
  from_paths.push(tf_cfg.as_str());
  from_paths.push(tf_gameinfo.as_str());

  copy_items(&from_paths, &Path::new(&new_tf_folder), &options).unwrap();

  let new_gameinfo = &format!("{}\\gameinfo.txt", new_tf_folder);

  let contents = fs::read_to_string(new_gameinfo).unwrap();

  let new = contents
    .replace("game+mod+custom_mod\ttf/custom/*", "game+mod+custom_mod\t|gameinfo_path|custom/*")
    .replace("game+game_write\t\ttf", "game+game_write\t\t|gameinfo_path|\n\t\t\tgame\t\t\ttf");

  let mut file = OpenOptions::new().write(true).truncate(true).open(new_gameinfo).unwrap();
  file.write(new.as_bytes()).unwrap();

  return json!({ 
    "name": folder_name,
    "tf_folder": format!("{}\\tf", new_folder_name)
  });
}

fn delete_folder(path: &PathBuf, try_count: i32) {
  if std::fs::remove_dir_all(path).is_err() {
    println!("Failed to delete folder");

    if try_count > 5 {
      return;
    }

    println!("Trying to delete folder again");
    thread::sleep(std::time::Duration::from_secs(1));
    delete_folder(path, try_count + 1);
  }
}

fn load_vdm(tf_folder: &str, demo_name: &str) -> Option<VDM> {
  let vdm_path = format!("{}\\{}.vdm", tf_folder, demo_name);

  let vdm_result = VDM::open(&vdm_path);

  println!("VDM path: {}", vdm_path);

  let vdm = match vdm_result {
    Ok(vdm) => vdm,
    Err(e) => {
      println!("Failed to open {}.vdm: {}", demo_name, e);
      return None;
    }
  };

  return Some(vdm);
}

pub(crate) fn get_next_demo(settings: &Value, demo_name: &str) -> Value {
  use vdm::VDM;

  let output_folder = settings["output"]["folder"].as_str().unwrap();
  let tf_folder = settings["tf_folder"].as_str().unwrap();

  println!("Output folder: {}", output_folder);

  let mut last_modified;

  match last_modified_folder(output_folder) {
    Some(entry) => last_modified = entry.path(),
    None => {
      let vdm = load_vdm(tf_folder, &demo_name);

      if vdm.is_none() {
        println!("Failed to load vdm");
        return json!({
          "complete": false,
          "next_demo": demo_name
        });
      }

      let vdm = vdm.unwrap();

      let props = vdm.first().props();

      if props.commands.contains("quit;") {
        if props.name.contains("mls_load_vdm") {
          let new_vdm = props.name.replace("mls_load_vdm ", "");

          let new_vdm_path = format!("{}\\{}.vdm", tf_folder, new_vdm);

          let new_vdm = VDM::open(&new_vdm_path).unwrap();

          new_vdm.export(&format!("{}\\{}.vdm", tf_folder, demo_name));

          return json!({
            "complete": false,
            "next_demo": demo_name
          });
        }

        return json!({
          "complete": true
        });
      }

      return json!({
        "complete": false,
        "next_demo": demo_name
      })
    },
  };

  println!("Last modified: {}", last_modified.display());

  let mut clip_name = format!("{}", last_modified.file_name().unwrap().to_str().unwrap());
  let mut demo_name = get_demo_name(&last_modified, demo_name);

  if !has_audio(&last_modified) {
    println!("Audio not in recording, deleting");

    delete_folder(&last_modified, 0);

    let last_folder = last_modified_folder(output_folder);

    if last_folder.is_some() {
      last_modified = last_folder.unwrap().path();
      clip_name = format!("{}", last_modified.file_name().unwrap().to_str().unwrap());
      println!("New clip name: {}", clip_name);
      demo_name = get_demo_name(&last_modified, &demo_name);
    }
  }

  if demo_name.contains("~") {
    let mut demo_split = demo_name.split("~");
    demo_name = demo_split.next().unwrap().to_string();
  }

  let vdm_path = format!("{}\\{}.vdm", tf_folder, demo_name);
  let vdm = load_vdm(tf_folder, &demo_name);

  if vdm.is_none() {
    println!("Failed to load vdm");
    return json!({
      "complete": false,
      "next_demo": demo_name
    });
  }

  let mut vdm = vdm.unwrap();

  let actions = &vdm.actions;

  let mut i: usize = 0;
  for (index, action) in actions.iter().enumerate() {
    match action {
      vdm::action::Action::PlayCommands(props) => {
        if props.commands.contains(&clip_name) {
          println!("Found the clip name");
          println!("index of: {}", index);
          i = index + 1;
          break;
        }

        if props.commands.contains("quit;") {
          if props.name.contains("mls_load_vdm") {
            let new_vdm = props.name.replace("mls_load_vdm ", "");

            let new_vdm_path = format!("{}\\{}.vdm", tf_folder, new_vdm);

            let new_vdm = VDM::open(&new_vdm_path).unwrap();

            new_vdm.export(&vdm_path);

            return json!({
              "complete": false,
              "next_demo": demo_name
            });
          }

          return json!({
            "complete": true
          });
        }
      }
      _ => {}
    }
  }

  if i == 0 {
    return json!({
      "complete": false,
      "next_demo": demo_name
    });
  }

  while i > 0 {
    vdm.remove(i);
    i -= 1;
  }

  let second_skip = vdm.nth_mut(1);
  let mut skip_to = 0;

  match second_skip {
    vdm::action::Action::SkipAhead(props) => {
      if props.skip_to_tick.is_some() {
        skip_to = props.skip_to_tick.unwrap();
      }
    }
    vdm::action::Action::PlayCommands(props) => {
      if props.commands.contains("playdemo") {
        let commands = props.commands.as_str();
        let playdemo = commands.replace("playdemo ", "");

        return json!({
          "complete": false,
          "next_demo": &playdemo.replace(";", "")
        });
      }

      if props.commands.contains("quit;") {
        if props.name.contains("mls_load_vdm") {
          let new_vdm = props.name.replace("mls_load_vdm ", "");

          let new_vdm_path = format!("{}\\{}.vdm", tf_folder, new_vdm);

          let new_vdm = VDM::open(&new_vdm_path).unwrap();

          new_vdm.export(&vdm_path);

          return json!({
            "complete": false,
            "next_demo": demo_name
          });
        }

        return json!({
          "complete": true
        });
      }

      return json!({
        "complete": true
      });
    }
    _ => {}
  }

  let first = vdm.first_mut();

  first.props_mut().skip_to_tick = Some(skip_to);

  vdm.remove(1);

  vdm.export(&vdm_path);

  return json!({
    "complete": false,
    "next_demo": demo_name
  });
}

pub fn get_install(settings: &Value, tab: i64) -> String {
  if tab == 0 {
    return settings["tf_folder"].as_str().unwrap().to_string();
  }
  
  let alt_installs = settings["alt_installs"].as_array().unwrap();
  
  return alt_installs[(tab - 1) as usize]["tf_folder"].as_str().unwrap().to_string();
}