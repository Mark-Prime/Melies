use std::{path::{Path, PathBuf}, process::Command, thread};
use serde_json::{json, Value};

fn get_tf2_path(settings: &Value) -> PathBuf {
  let mut tf2_path = PathBuf::from(settings["tf_folder"].as_str().unwrap()).parent().unwrap().to_path_buf();

  if settings["hlae"]["use_64bit"] == true {
    tf2_path.push("tf_win64.exe");
  } else {
    tf2_path.push("tf.exe");
  }

  return tf2_path;
}

fn build_launch_options(settings: &Value, demo_name: &str, install: &str) -> String {
  let mut launch_options = settings["hlae"]["launch_options"].to_string().replace("\"", "").replace("-game tf", "");

  if demo_name != "" && settings["hlae"]["playdemo"] == true {
    let mut trimmed_name = demo_name.to_string().replace(".dem", "");

    if trimmed_name.chars().nth(0).unwrap() == '\\' {
      trimmed_name.remove(0);
    }

    println!("Playing demo: {}", trimmed_name);
    launch_options = format!("{} +playdemo {}", launch_options, trimmed_name);
  }

  if install != settings["tf_folder"].as_str().unwrap() {
    let tf2_path = PathBuf::from(settings["tf_folder"].as_str().unwrap()).parent().unwrap().to_path_buf();
    let tf2_str = tf2_path.to_str().unwrap().to_owned() + "\\";

    let game = install.replace(&tf2_str, "");
    launch_options = format!("{} -game {}", launch_options, game);
  } else if settings["hlae"]["use_64bit"] == true {
    launch_options = format!("{} -game tf", launch_options);
  }

  return launch_options;
}

pub(crate) fn run_tf2(demo_name: &str, settings: &Value, install: &str) {
  println!("Running TF2");

  let launch_options = build_launch_options(settings, demo_name, install);

  println!("Launch options: {}", launch_options);

  let tf2_path = get_tf2_path(settings);

  let sparkly_path = match settings["hlae"]["use_64bit"].as_bool().unwrap() {
    true => settings["hlae"]["sparklyfx_path"].as_str().unwrap().replace("xsdk-base.dll", "xsdk-base64.dll"),
    false => settings["hlae"]["sparklyfx_path"].as_str().unwrap().replace("xsdk-base64.dll", "xsdk-base.dll"),
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

      if !settings["hlae"]["use_64bit"].as_bool().unwrap() && Path::new(&hlae_dll).exists() {
        args.push("-hookDllPath");
        args.push(&hlae_dll);
      }

      Command::new(hlae)
        .args(args)
        .spawn()
        .expect("failed to execute process");
    }
    "svr" => {
      return;
    }
    "svr.mov" => {
      return;
    }
    "svr.mp4" => {
      return;
    }
    _ => {
      Command::new(tf2_path)
        .args(launch_options.as_str().split(" ").collect::<Vec<_>>())
        .spawn()
        .expect("failed to execute process");
    }
  }

  wait_for_tf2(settings);
}

fn wait_for_tf2(settings: &Value) -> bool {
  use sysinfo::System;
  use std::{thread, time};

  loop {
    let wait_time = time::Duration::from_secs(1);

    thread::sleep(wait_time);

    let s = System::new_all();

    let mut process_found = false;

    for _process in s.processes_by_name("tf_win64".as_ref()) {
      process_found = true;
      break;
    }

    if settings["hlae"]["use_64bit"] == true {
      for _process in s.processes_by_name("tf_win64".as_ref()) {
        process_found = true;
        break;
      }
    } else {
      for _process in s.processes_by_name("tf".as_ref()) {
        process_found = true;
        break;
      }
    }

    if !process_found {
      break;
    }
  }

  true
}

fn last_modified_folder(videos_folder: &str) -> Option<std::fs::DirEntry> {
    std::fs::read_dir(videos_folder)
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

fn get_demo_name(path: &PathBuf) -> String {
    use regex::Regex;
    
    let name = path.file_name().unwrap().to_str().unwrap();

    let re = Regex::new(r"(?<name>.*)_\d*-\d*(.*)").unwrap();

    let Some(caps) = re.captures(name) else {
        println!("no match!");
        return String::new();
    };

    return caps["name"].to_string();
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

pub(crate) fn batch_record(demo_name: &str, settings: &Value, install: &str) -> Value {
  use vdm::VDM;

  run_tf2(demo_name, settings, install);

  let output_folder = settings["output"]["folder"].as_str().unwrap();
  let tf_folder = settings["tf_folder"].as_str().unwrap();

  let mut last_modified = last_modified_folder(output_folder).unwrap().path();

  let mut clip_name = format!("{}", last_modified.file_name().unwrap().to_str().unwrap());
  let mut demo_name = get_demo_name(&last_modified);

  if !has_audio(&last_modified) {
    println!("Audio not in recording, deleting");

    delete_folder(&last_modified, 0);

    last_modified = last_modified_folder(output_folder).unwrap().path();
    clip_name = format!("{}", last_modified.file_name().unwrap().to_str().unwrap());
    println!("New clip name: {}", clip_name);
    demo_name = get_demo_name(&last_modified);
  }

  let vdm_path = format!("{}\\{}.vdm", tf_folder, demo_name);

  let mut vdm = VDM::open(&vdm_path).unwrap();

  let actions = &vdm.actions;

  let mut i = 0;
  for (index, action) in actions.iter().enumerate() {
      match action {
          vdm::action::Action::PlayCommands(props) => {
              if props.commands.contains(&clip_name) {
                  println!("Found the clip name");
                  println!("index of: {}", index);
                  i = index + 1;
                  break;
              }
          }
          _ => {}
      }
  }

  if i == 0 {
    return json!({
      "complete": false,
      "next_demo": demo_name
    })
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