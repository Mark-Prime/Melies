use std::{ env, fs::{ self, File }, path::Path };
use chrono::{ DateTime, Local };
use regex::Regex;
use serde_json::{ self, json, Value };
use vdm::{ VDM, action::ActionType };

use crate::{
  cfg::write_cfg,
  clip::Clip,
  command::{Command, CommandType},
  demos::load_demo,
  event::{ Event, EventStyle::Bookmark },
  macros::ifelse,
  settings::{ load_settings, setting_as_bool },
  util::find_dir,
};

enum Action {
  Clip(Clip),
  Command(Command),
}

pub fn run_ryukbot() -> Value {
  let settings = crate::settings::load_settings();

  let dir;

  match find_dir(&settings) {
    Ok(directory) => {
      dir = directory;
    }
    Err(err) => {
      return json!({
          "code": 404,
          "err_text": err
        });
    }
  }

  let file_text = match fs::read_to_string(dir.clone()) {
    Ok(text) => text,
    Err(_) => {
      return json!({
        "code": 404,
        "err_text": "Failed to read _events.txt or KillStreaks.txt\nPlease check your settings to ensure the tf folder is correctly linked.".to_string()
      });
    }
  };

  let re = Regex::new("\\[(.*)\\] (.*) \\(\"(.*)\" at (\\d*)\\)").unwrap();

  let events = re.captures_iter(&file_text);

  let event_len = events.count();

  if event_len == 0 {
    return json!({
      "code": 410,
      "err_text": format!("_events.txt or KillStreaks.txt was found but had no valid events. Please add events before running again.\n\nFile Location: {}", dir)
    });
  }

  match write_cfg(&settings) {
    Ok(_) => {}
    Err(err) => {
      return json!({
        "code": 500,
        "err_text": err
      });
    }
  }

  let mut clips: Vec<Action> = vec![];

  let mut event_count = 0;

  let events = re.captures_iter(&file_text);

  for event_capture in events {
    event_count = event_count + 1;

    let event = Event::new(event_capture).unwrap();

    if event.contains(&"mls_skip") {
      continue;
    }

    if event.contains(&"mls_load_vdm") | event.contains(&"mls_exec") {
      let commands = event.commands();

      let mut demo_found = false;
      let mut exec_found = false;

      for command in commands {
        if exec_found {
          clips.push(Action::Command(Command {
            demo_name: event.demo_name.clone(),
            command_type: CommandType::Console(format!("exec {}", command)),
            tick: event.tick,
            // event: event.clone(),
          }));
          continue;
        }
        
        if demo_found {
          clips.push(Action::Command(Command {
            demo_name: event.demo_name.clone(),
            command_type: CommandType::LoadVdm(format!("{}", command)),
            tick: event.tick,
            // event: event.clone(),
          }));
          continue;
        }

        if command.contains(&"mls_load_vdm") {
          demo_found = true;
          continue;
        }

        if command.contains(&"mls_exec") {
          exec_found = true;
          continue;
        }
      }

      continue;
    }

    if event.contains(&"mls_rec_demo") {
      let demo = load_demo(&settings, &event.demo_name);

      println!("{}", demo);

      if !demo["loaded"].as_bool().unwrap() {
        continue;
      }

      if let Bookmark(val) = &event.value {
        let mut start_event = event.clone();
        let mut end_event = event.clone();

        let start_val = val.replace("mls_rec_demo", "clip_start");
        start_event.value = Bookmark(start_val);
        start_event.tick = settings["recording"]["start_delay"].as_i64().unwrap();

        end_event.value = Bookmark("clip_end".to_string());
        end_event.tick = demo["header"]["ticks"].as_i64().unwrap() - 99;

        let mut clip = Clip::new(start_event, &settings);

        if clip.can_include(&end_event, &settings) {
          clip.include(end_event, &settings);
        }

        clips.push(Action::Clip(clip));
        continue;
      }
    }

    let clip_len = clips.len();

    if clip_len == 0 {
      clips.push(Action::Clip(Clip::new(event, &settings)));
      continue;
    }

    match &mut clips[clip_len - 1] {
      Action::Clip(clip) => {
        if clip.can_include(&event, &settings) {
          clip.include(event, &settings);
          continue;
        }
      }
      _ => {}
    }

    clips.push(Action::Clip(Clip::new(event, &settings)));
  }

  let mut current_demo: String = "".to_string();
  let mut vdm: VDM = VDM::new();
  let mut vdms = vec![];

  for clip in &clips {
    match clip {
      Action::Clip(clip) => {
        if current_demo == clip.demo_name {
          add_clip_to_vdm(&mut vdm, clip, &settings);
          continue;
        }

        if vdm.len() > 0 {
          vdms.push(vdm);
        }

        current_demo = clip.demo_name.clone();
        vdm = VDM::new();
        vdm.name = clip.demo_name.clone();
        start_vdm(&mut vdm, clip, &settings);
      }
      Action::Command(command) => {
        if current_demo == command.demo_name {
          add_command_to_vdm(&mut vdm, command);
          continue;
        }

        if vdm.len() > 0 {
          vdms.push(vdm);
        }

        current_demo = command.demo_name.clone();
        vdm = VDM::new();
        vdm.name = command.demo_name.clone();
        add_command_to_vdm(&mut vdm, command);
      }
    }
  }

  vdms.push(vdm);

  let first_demo = vdms[0].name.clone();

  let vdm_count = &vdms.len();

  for (i, vdm) in vdms.iter().enumerate() {
    let mut folder = format!("{}\\demos", &settings["tf_folder"].as_str().unwrap());

    if settings["absolute_file_paths"].as_bool().unwrap() {
      folder = format!("{}", &settings["tf_folder"].as_str().unwrap());
    }

    let file_location = format!("{}\\{}.vdm", folder, &vdm.name);

    let path = Path::new(&file_location);

    if !path.parent().unwrap().exists() {
      fs::create_dir_all(path).unwrap();
    }

    if settings["safe_mode"].as_i64().is_some() {
      if setting_as_bool(&settings["safe_mode"]) {
        let file_path = Path::new(&file_location);
        if file_path.exists() {
          continue;
        }
      }
    }

    if vdm.len() == 0 {
      continue;
    }

    let vdm = end_vdm(
      &mut vdm.clone(),
      &settings,
      ifelse!(vdms.len() > i + 1, String::from(&vdms[i + 1].name), String::new())
    );

    vdm.export(&file_location);
  }

  let mut backup_location = "".to_string();

  if settings["save_backups"].as_bool().unwrap() {
    let saved = save_backup(&settings);

    backup_location = saved["output_path"].as_str().unwrap().to_owned();
  }

  if settings["clear_events"].as_bool().unwrap() {
    clear_events(settings);
  }

  json!({
    "clips": clips.len(),
    "events": event_count,
    "vdms": vdm_count,
    "code": 200,
    "backup_location": backup_location,
    "first_demo": first_demo
  })
}

fn start_vdm(vdm: &mut VDM, clip: &Clip, settings: &Value) {
  if let Some(start_delay) = settings["recording"]["start_delay"].as_i64() {
    if clip.start_tick > start_delay + 66 {
      let skip_props = vdm.create_action(ActionType::SkipAhead).props_mut();

      if let Some(start_delay) = settings["recording"]["start_delay"].as_i64() {
        skip_props.start_tick = Some(start_delay);
      }
      if let Some(skip_to_tick) = clip.start_tick.checked_sub(66) {
        skip_props.skip_to_tick = Some(skip_to_tick);
      }

      skip_props.name = format!("Skip to first clip");
    }
  }

  record_clip(vdm, clip, settings);
}

fn end_vdm(vdm: &mut VDM, settings: &Value, next_demoname: String) -> VDM {
  let last_action = vdm.last();

  if last_action.props().commands == "quit;" {
    return vdm.to_owned();
  }

  let last_tick = match last_action.props().start_tick {
    Some(tick) => tick,
    None => {
      return vdm.to_owned();
    }
  };

  {
    let exec_commands = vdm.create_action(ActionType::PlayCommands).props_mut();

    exec_commands.start_tick = Some(last_tick + 66);

    if setting_as_bool(&settings["recording"]["record_continuous"]) && next_demoname != "" {
      exec_commands.name = format!("Start the next demo ({}.dem)", next_demoname);
      exec_commands.commands = format!("playdemo {};", next_demoname);
      return vdm.to_owned();
    }

    exec_commands.name = "Exit TF2".to_string();
    exec_commands.commands = "quit;".to_string();
  }

  vdm.to_owned()
}

fn add_command_to_vdm(vdm: &mut VDM, command: &Command) {
  match &command.command_type {
    CommandType::Console(command_str) => {
      let exec_commands = vdm.create_action(ActionType::PlayCommands).props_mut();

      exec_commands.start_tick = Some(command.tick);
      exec_commands.name = "Exec Config Commands".to_string();
      exec_commands.commands = command_str.to_string();
    }
    CommandType::LoadVdm(name) => {
      let exec_commands = vdm.create_action(ActionType::PlayCommands).props_mut();

      exec_commands.start_tick = Some(command.tick);
      exec_commands.name = format!("mls_load_vdm {}", name);
      exec_commands.commands = "quit;".to_string();
    }
  }
}

fn add_clip_to_vdm(vdm: &mut VDM, clip: &Clip, settings: &Value) {
  println!("add_clip_to_vdm: {:?}", vdm);

  let last_tick = match vdm.last().props().start_tick {
    Some(action) => action,
    None => {
      return;
    }
  };

  if clip.start_tick > last_tick + 300 {
    let skip_props = vdm.create_action(ActionType::SkipAhead).props_mut();

    skip_props.name = format!("Skip {} ticks", clip.start_tick - last_tick);
    skip_props.start_tick = Some(last_tick + 66);
    skip_props.skip_to_tick = Some(clip.start_tick - 66);
  }

  record_clip(vdm, clip, settings);
}

fn record_clip(vdm: &mut VDM, clip: &Clip, settings: &Value) {
  let mut vdm_name = vdm.name.clone();

  if settings["absolute_file_paths"].as_bool().unwrap_or(false) {
    vdm_name = vdm_name.replace("demos\\", "");
  }

  let mut suffix = "bm".to_string();

  if clip.has_killstreak {
    suffix = format!("ks{}", clip.ks_value);

    if clip.has_bookmark {
      suffix = format!("bm{}+", clip.ks_value);
    }
  }

  {
    let exec_commands = vdm.create_action(ActionType::PlayCommands).props_mut();

    exec_commands.start_tick = Some(clip.start_tick - 33);
    exec_commands.name = "Exec Melies Commands".to_string();
    exec_commands.commands = format!(
      "exec melies;{}",
      ifelse!(setting_as_bool(&settings["output"]["snd_fix"]), " snd_fix;", "")
    );
  }

  {
    let start_record = vdm.create_action(ActionType::PlayCommands).props_mut();

    let clip_name_fallback = format!(
      "{}_{}-{}_{}",
      vdm_name,
      clip.start_tick,
      clip.end_tick,
      suffix
    );

    let mut clip_name = settings["output"]["clip_name_template"]
      .as_str()
      .unwrap_or(&clip_name_fallback)
      .replace("{demo_name}", &vdm_name)
      .replace("{start_tick}", &clip.start_tick.to_string())
      .replace("{end_tick}", &clip.end_tick.to_string())
      .replace("{suffix}", &suffix)
      .replace("{recording_method}", &settings["output"]["method"].as_str().unwrap())
      .to_string();

    let mut bm_value = clip.bm_value.to_owned();

    bm_value = bm_value.replace("clip_start", "");
    bm_value = bm_value.replace("clip_end", "");

    if
      bm_value != "".to_string() &&
      setting_as_bool(&settings["recording"]["auto_suffix"]) &&
      bm_value != "General".to_string()
    {
      clip_name = clip_name.replace(
        "{bookmarks}",
        &bm_value.replace("-spec", "").trim().replace(" ", "-")
      );
    } else {
      clip_name = clip_name.replace("{bookmarks}", "");
    }

    let mut commands = "".to_string();

    match settings["output"]["method"].as_str().unwrap() {
      "h264" | "jpeg" => {
        commands = format!(
          "host_framerate {}; startmovie {} {}; clear",
          settings["output"]["framerate"],
          clip_name,
          settings["output"]["method"].as_str().unwrap()
        );
      }
      "tga" => {
        commands = format!(
          "host_framerate {}; startmovie {}; clear",
          settings["output"]["framerate"],
          clip_name
        );
      }
      "sparklyfx" => {
        if settings["output"]["folder"].as_str().unwrap().len() > 0 {
          commands = format!(
            "sf_recorder_start {}\\{}",
            settings["output"]["folder"].as_str().unwrap(),
            clip_name
          );
        } else {
          commands = format!("sf_recorder_start; clear");
        }
      }
      "svr" => {
        commands = format!("startmovie {}.mkv", clip_name);
      }
      "svr.mp4" => {
        commands = format!("startmovie {}.mp4", clip_name);
      }
      "svr.mov" => {
        commands = format!("startmovie {}.mov", clip_name);
      }
      "lawena" => {
        commands = format!("startrecording");
      }
      _ => {}
    }

    let commands = check_spec(clip, commands);

    start_record.start_tick = Some(clip.start_tick);
    start_record.name = "Start Recording".to_string();
    start_record.commands = commands;
  }

  {
    let end_record = vdm.create_action(ActionType::PlayCommands).props_mut();
    let mut commands = String::new();

    match settings["output"]["method"].as_str().unwrap() {
      "h264" | "jpeg" | "tga" | "svr" | "svr.mp4" | "svr.mov" => {
        commands = format!(
          "{}; endmovie; host_framerate 0;",
          settings["recording"]["end_commands"].as_str().unwrap()
        );
      }
      "sparklyfx" => {
        commands = format!(
          "{}; sf_recorder_stop;",
          settings["recording"]["end_commands"].as_str().unwrap()
        );
      }
      "lawena" => {
        commands = format!(
          "{}; stoprecording;",
          settings["recording"]["end_commands"].as_str().unwrap()
        );
      }
      _ => {}
    }

    end_record.start_tick = Some(clip.end_tick);
    end_record.name = "Stop Recording".to_string();
    end_record.commands = commands;
  }
}

fn check_spec(clip: &Clip, commands: String) -> String {
  let settings = load_settings();

  if clip.spec_type == 0 {
    return commands;
  }

  let mut new_commands = commands;

  let use_ce_spec: bool = match settings["recording"]["use_ce_spec"].as_bool() {
    Some(val) => val,
    None => false,
  };

  new_commands = format!(
    "{}; {} {}; spec_mode {};",
    new_commands,
    ifelse!(use_ce_spec, "ce_cameratools_spec_steamid", "spec_player"),
    clip.spec_player,
    ifelse!(clip.spec_type == 1, 4, 5)
  );

  return new_commands;
}

fn clear_events(settings: Value) -> Value {
  let dir;

  match find_dir(&settings) {
    Ok(directory) => {
      dir = directory;
    }
    Err(err) => {
      return json!({
                "code": 404,
                "err_text": err
            });
    }
  }

  File::create(dir).unwrap();

  json!({
        "code": 200,
        "events": []
    })
}

fn save_backup(settings: &Value) -> Value {
  let dir;

  match find_dir(&settings) {
    Ok(directory) => {
      dir = directory;
    }
    Err(err) => {
      return json!({
                "code": 404,
                "err_text": err
            });
    }
  }

  let sys_time: DateTime<Local> = Local::now();
  let date = sys_time.format("%Y-%m-%d_%H-%M-%S").to_string().replace("\"", "");

  let user_profile = env::var("USERPROFILE");

  let output_folder = match user_profile {
    Ok(profile) => { format!("{}\\Documents\\Melies\\backups", profile) }
    Err(_) => {
      format!("{}\\backups", std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap())
    }
  };

  let output_path = format!("{}\\{}.txt", output_folder, date);

  fs::create_dir_all(output_folder).unwrap();

  fs::copy(dir, &output_path).unwrap();

  json!({
        "code": 200,
        "output_path": output_path
    })
}
