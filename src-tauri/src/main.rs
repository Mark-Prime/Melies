#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::fs::{ File, DirEntry };
use std::io::Write;
use std::str::FromStr;
use std::{ fs, env };
use std::path::Path;
use serde_json::{ self, Value, json, Map };
use regex::Regex;
use tauri::command;
use vdm::VDM;
use vdm::action::ActionType;
use chrono::prelude::*;

use crate::event::EventStyle::{ Bookmark, Killstreak };
use crate::event::Event;
use crate::clip::Clip;
use crate::logstf::parse;
use crate::demos::{ scan_for_demos, scan_demo, validate_demos_folder };

mod event;
mod clip;
mod logstf;
mod demos;

macro_rules! ifelse {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}

macro_rules! extend {
    ($v:expr, $v1:expr, $c:expr) => {
        $v = format!("{}{}", $v, format!($v1, $c))
    };
}

fn setting_as_bin(setting: &Value) -> i64 {
    if !setting.is_boolean() {
        return setting.as_i64().unwrap();
    }

    match setting.as_bool() {
        Some(val) => {
            match val {
                true => 1,
                false => 0
            }
        }
        _ => 0
    }
}

fn setting_as_bool(setting: &Value) -> bool {
    if setting.is_boolean() {
        return setting.as_bool().unwrap();
    }

    match setting.as_i64() {
        Some(val) => {
            match val {
                1 => true,
                _ => false
            }
        }
        _ => false
    }
}

fn write_cfg(settings: &Value) {
    let mut cfg = String::new();

    extend!(
        cfg,
        "echo \"Execing Melies Config\";\r\ncl_drawhud {};\r\n",
        setting_as_bin(&settings["output"]["hud"])
    );
    extend!(cfg, "sv_cheats {};\r\n", "1");
    extend!(cfg, "voice_enable {};\r\n", setting_as_bin(&settings["output"]["voice_chat"]));
    extend!(cfg, "hud_saytext_time {};\r\n", setting_as_bin(&settings["output"]["text_chat"]) * 12);
    extend!(cfg, "crosshair {};\r\n", setting_as_bin(&settings["output"]["crosshair"]));
    extend!(cfg, "r_drawviewmodel {};\r\n", setting_as_bin(&settings["output"]["viewmodel"]));
    extend!(cfg, "viewmodel_fov_demo {};\r\n", setting_as_bin(&settings["recording"]["viewmodel_fov"]));
    extend!(cfg, "fov_desired {};\r\n", setting_as_bin(&settings["recording"]["fov"]));

    if setting_as_bin(&settings["recording"]["third_person"]) == 1 {
        extend!(cfg, "thirdperson{};\r\n", "");
    } else {
        extend!(cfg, "firstperson{};\r\n", "");
    }

    extend!(cfg, "{};\r\n", settings["recording"]["commands"].as_str().unwrap());
    extend!(cfg, "{};\r\n", "alias \"snd_fix\" \"snd_restart; snd_soundmixer Default_mix;\"");

    // let map: &Map<String, Value> = &settings["addons"].as_object().unwrap();
    let addons = settings["addons"].as_object();

    match addons {
        Some(map) => {
            for (k, v) in map {
                extend!(
                    cfg,
                    "\r\necho \"Running {} addon\";\r\n",
                    k
                );
        
                let v_map = v.as_object().unwrap();
        
                for (_ki, vi) in v_map {
        
                    match &vi["type"] {
                        Value::String(vi_type) => {
                            match vi_type.as_str() {
                                "toggle" => {
                                    if vi["value"] == true {
                                        extend!(
                                            cfg, "{};\r\n", vi["command"]
                                        );
                                    }
                                },
                                "bool" => {
                                    extend!(cfg, "{};\r\n", format!("{} {}", vi["command"].to_string().replace("\"", ""), setting_as_bin(&vi["value"])));
                                },
                                "string" | "int" => {
                                    extend!(cfg, "{};\r\n", format!("{} {}", vi["command"].to_string().replace("\"", ""), vi["value"]));
                                },
                                _ => {
                                    continue;
                                }
                            }
                        },
                        _ => {
                            continue;
                        }
                    }
                }
            }
        },
        None => {},
    }

    if setting_as_bin(&settings["output"]["lock"]) == 1 {
        extend!(
            cfg,
            "\r\necho \"Preventing settings from changing\";\r\nalias cl_drawhud \"{}\";\r\n",
            ""
        );
        extend!(cfg, "alias voice_enable \"{}\";\r\n", "");
        extend!(cfg, "alias hud_saytext_time \"{}\";\r\n", "");
        extend!(cfg, "alias crosshair \"{}\";\r\n", "");
        extend!(cfg, "alias r_drawviewmodel \"{}\";\r\n", "");
        extend!(cfg, "alias viewmodel_fov_demo \"{}\";\r\n", "");
        extend!(cfg, "alias fov_desired \"{}\";\r\n", "");
    }

    let mut file = File::create(
        format!("{}\\cfg\\melies.cfg", settings["tf_folder"].as_str().unwrap())
    ).unwrap();

    file.write_all(cfg.as_bytes()).unwrap();
}

fn end_vdm(vdm: &mut VDM, settings: &Value, next_demoname: String) -> VDM {
    let last_tick = vdm.last().props().start_tick.unwrap();

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

fn check_spec(clip: &Clip, commands: String) -> String {
    if clip.spec_type == 0 {
        return commands;
    }

    let mut new_commands = commands;

    new_commands = format!("{}; spec_player {}; spec_mode;", new_commands, clip.spec_player);

    if clip.spec_type == 3 {
        new_commands = format!("{} spec_mode;", new_commands);
    }

    return new_commands;
}

fn start_vdm(vdm: &mut VDM, clip: &Clip, settings: &Value) {
    if clip.start_tick > settings["recording"]["start_delay"].as_i64().unwrap() + 66 {
        let skip_props = vdm.create_action(ActionType::SkipAhead).props_mut();

        skip_props.start_tick = Some(settings["recording"]["start_delay"].as_i64().unwrap());
        skip_props.skip_to_tick = Some(clip.start_tick - 66);
    }

    record_clip(vdm, clip, settings);
}

fn add_clip_to_vdm(vdm: &mut VDM, clip: &Clip, settings: &Value) {
    let last_tick = vdm.last().props().start_tick.unwrap();

    if clip.start_tick > last_tick + 300 {
        let skip_props = vdm.create_action(ActionType::SkipAhead).props_mut();

        skip_props.start_tick = Some(last_tick + 66);
        skip_props.skip_to_tick = Some(clip.start_tick - 66);
    }

    record_clip(vdm, clip, settings);
}

fn record_clip(vdm: &mut VDM, clip: &Clip, settings: &Value) {
    let vdm_name = vdm.name.clone();

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
        exec_commands.commands = "exec melies;".to_string();
    }

    {
        let start_record = vdm.create_action(ActionType::PlayCommands).props_mut();

        let mut clip_name = format!(
            "{}_{}-{}_{}",
            vdm_name,
            clip.start_tick,
            clip.end_tick,
            suffix
        );

        let mut bm_value = clip.bm_value.to_owned();

        bm_value = bm_value.replace("clip_start", "");
        bm_value = bm_value.replace("clip_end", "");

        if
            bm_value != "".to_string() &&
            setting_as_bool(&settings["recording"]["auto_suffix"]) &&
            bm_value != "General".to_string()
        {
            clip_name = format!("{}_{}", clip_name, bm_value.replace("spec", "").trim().replace(" ", "-"));
        }

        let mut commands = "".to_string();

        match settings["output"]["method"].as_str().unwrap() {
            "h264" | "jpeg" => {
                commands = format!(
                    "{}host_framerate {}; startmovie {} {}; clear",
                    ifelse!(setting_as_bool(&settings["output"]["snd_fix"]), "snd_fix; ", ""),
                    settings["output"]["framerate"],
                    clip_name,
                    settings["output"]["method"].as_str().unwrap()
                );
            }
            "tga" => {
                commands = format!(
                    "{}host_framerate {}; startmovie {}; clear",
                    ifelse!(setting_as_bool(&settings["output"]["snd_fix"]), "snd_fix; ", ""),
                    settings["output"]["framerate"],
                    clip_name
                );
            }
            "svr" => {
                commands = format!(
                    "{}; startmovie {}.mp4; clear",
                    ifelse!(setting_as_bool(&settings["output"]["snd_fix"]), "snd_fix; ", ""),
                    clip_name
                );
            }
            "lawena" => {
                commands = format!(
                    "{}startrecording",
                    ifelse!(setting_as_bool(&settings["output"]["snd_fix"]), "snd_fix; ", ""),
                );
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
            "h264" | "jpeg" | "tga" | "svr" => {
                commands = format!(
                    "{}; endmovie; host_framerate 0;",
                    settings["recording"]["end_commands"].as_str().unwrap()
                );
            }
            "lawena" => {
                commands = format!(
                    "stoprecording;"
                );
            }
            _ => {}
        }

        if clip.spec_type == 1 {
            commands = format!("{} spec_mode; spec_mode;", commands);
        }

        if clip.spec_type == 3 {
            commands = format!("{} spec_mode;", commands);
        }

        end_record.start_tick = Some(clip.end_tick);
        end_record.name = "Stop Recording".to_string();
        end_record.commands = commands;
    }
}

fn check_dir(files: Result<fs::ReadDir, std::io::Error>) -> Result<String, String> {
    let entries;

    match files {
        Ok(ent) => {
            entries = ent;
        }
        Err(_) => {
            return Err(
                "Could not find the _events.txt or KillStreaks.txt files.\r\nPlease check your settings to ensure the tf folder is correctly linked.\r\nIf you do not have either file, please make one in the \\tf or \\tf\\demos folder.".to_string()
            );
        }
    }

    for entry in entries {
        let dir: DirEntry;

        match entry {
            Ok(directory) => {
                dir = directory;
            }
            Err(err) => {
                return Err(err.to_string());
            }
        }

        let dir_str = dir.path().to_string_lossy().to_string();

        if dir_str.ends_with("\\_events.txt") {
            return Ok(dir_str);
        }

        if dir_str.ends_with("\\KillStreaks.txt") {
            return Ok(dir_str);
        }
    }

    Err(
        format!(
            "File Not Found",
        )
    )
}

fn find_dir(settings: &Value) -> Result<String, String> {
    let files: Result<fs::ReadDir, std::io::Error> = fs::read_dir(format!("{}\\demos", settings["tf_folder"].as_str().unwrap()));

    match check_dir(files) {
        Ok(res) => {
            return Ok(res);
        },
        Err(_) => {}
    }

    let files: Result<fs::ReadDir, std::io::Error> = fs::read_dir(format!("{}", settings["tf_folder"].as_str().unwrap()));

    match check_dir(files) {
        Ok(res) => {
            return Ok(res);
        },
        Err(_) => {}
    }

    Err(
        format!(
            "Could not find the _events.txt or KillStreaks.txt files.\r\nPlease check your settings to ensure the tf folder is correctly linked.\r\nIf you do not have either file, please make one in the \\tf or \\tf\\demos folder. \r\n\r\ntf_folder setting: ({})",
            settings["tf_folder"].as_str().unwrap()
        )
    )
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
fn ryukbot() -> Value {
    let settings = load_settings();

    let dir;

    match find_dir(&settings) {
        Ok(directory) => {
            dir = directory;
        }
        Err(_) => {
            return json!({
                "code": 404,
                "err_text": format!("Could not find _events.txt or KillStreaks.txt\r\nPlease check your settings to ensure the tf folder is correctly linked.\r\n\r\n tf folder setting: {}", settings["tf_folder"].as_str().unwrap())
            });
        }
    }

    let file_text = match fs::read_to_string(dir.clone()) {
        Ok(text) => { text }
        Err(_) => {
            return json!({
                "code": 404,
                "err_text": "Failed to read _events.txt or KillStreaks.txt\r\nPlease check your settings to ensure the tf folder is correctly linked.".to_string()
            });
        }
    };

    let re = Regex::new("\\[(.*)\\] (.*) \\(\"(.*)\" at (\\d*)\\)").unwrap();

    let events = re.captures_iter(&file_text);

    let event_len = events.count();

    if event_len == 0 {
        return json!({
                "code": 410,
                "err_text": format!("_events.txt or KillStreaks.txt was found but found no valid events. Please add events before running again.\r\n\r\nFile Location: {}", dir)
            });
    }

    let re = Regex::new("\\[(.*)\\] (.*) \\(\"(.*)\" at (\\d*)\\)").unwrap();

    let events = re.captures_iter(&file_text);

    write_cfg(&settings);

    let mut clips: Vec<Clip> = vec![];

    let mut event_count = 0;

    for event_capture in events {
        event_count = event_count + 1;

        let event = event::Event::new(event_capture).unwrap();

        let clip_len = clips.len();

        if clip_len == 0 {
            clips.push(Clip::new(event, &settings));
            continue;
        }

        if clips[clip_len - 1].can_include(&event, &settings) {
            clips.last_mut().unwrap().include(event, &settings);
            continue;
        }

        clips.push(Clip::new(event, &settings));
    }

    let mut current_demo: String = "".to_string();
    let mut vdm: VDM = VDM::new();
    let mut vdms = vec![];

    for clip in &clips {
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

    vdms.push(vdm);

    let vdm_count = &vdms.len();

    for (i, vdm) in vdms.iter().enumerate() {
        let file_location = format!(
            "{}\\demos\\{}.vdm",
            &settings["tf_folder"].as_str().unwrap(),
            &vdm.name
        );

        if settings["safe_mode"].as_i64().is_some() {
            if setting_as_bool(&settings["safe_mode"]) {
                let file_path = Path::new(&file_location);
                if file_path.exists() {
                    continue;
                }
            }
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
        "backup_location": backup_location
    })
}

fn load_addons() -> Value {
    fs::create_dir_all(
        format!("{}\\Documents\\Melies\\addons", env::var("USERPROFILE").unwrap())
    ).unwrap();

    let mut addons = json!({});

    let files = fs::read_dir(format!("{}\\Documents\\Melies\\addons", env::var("USERPROFILE").unwrap())).unwrap();

    for file in files {
        let filename_os = file.as_ref().unwrap().file_name();
        let filename = filename_os.to_str().unwrap().to_string();

        if !filename.contains(".json") && !filename.contains(".JSON") {
            continue;
        }

        let mut name = filename.replace(".json", "");
        name = name.replace(".JSON", "");
        
        let data = fs::read_to_string(file.unwrap().path()).expect("Unable to read file");
        let res = match serde_json::from_str(&data) {
            Ok(val) => val,
            Err(_) => {
                continue;
            },
        };

        addons[name] = res;
    }

    addons
}

fn default_settings() -> Value {
    let defaults = json!({
        "tf_folder": "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Team Fortress 2\\tf",
        "clear_events": true,
        "save_backups": true,
        "safe_mode": true,
        "output": {
            "method": "h264",
            "framerate": 60,
            "crosshair": false,
            "viewmodel": true,
            "hud": true,
            "text_chat": false,
            "voice_chat": false,
            "snd_fix": true,
            "lock": true
        },
        "recording": {
            "commands": "",
            "end_commands": "",
            "start_delay": 50,
            "minimum_ticks_between_clips": 500,
            "before_bookmark": 1000,
            "after_bookmark": 200,
            "before_killstreak_per_kill": 500,
            "after_killstreak": 300,
            "interval_for_rewind_double_taps": 66,
            "rewind_amount": 1000,
            "fov": 90,
            "viewmodel_fov": 90,
            "record_continuous": true,
            "auto_close": true,
            "auto_suffix": true,
            "third_person": false
        }
    });

    defaults
}

fn build_settings() -> Value {
    let binding = env::var("USERPROFILE").unwrap() + "\\Documents\\Melies\\settings.json";
    let settings_path = Path::new(&binding);
    let settings_prefix = settings_path.parent().unwrap();
    std::fs::create_dir_all(settings_prefix).unwrap();

    File::create(settings_path).unwrap();

    let mut settings = default_settings();

    fs::write(settings_path, settings.to_string()).unwrap();

    settings["addons"] = load_addons();

    settings
}

fn merge(a: &mut Value, b: Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if k == "addons" {
                    continue;
                }

                if v.is_null() {
                    a.remove(&k);
                    continue;
                }
                
                merge(a.entry(k).or_insert(Value::Null), v);
            } 

            return;
        }
    }

    *a = b;
}

#[command]
fn load_settings() -> Value {
    let settings_path = env::var("USERPROFILE").unwrap() + "\\Documents\\Melies\\settings.json";

    if Path::new(&settings_path).exists() {
        let file = fs::read_to_string(settings_path).unwrap();
        let settings: Value = serde_json::from_str(&file).unwrap();

        let mut defaults = default_settings();

        merge(&mut defaults, settings);

        defaults["addons"] = load_addons();

        return defaults;
    }

    build_settings()
}

fn save_addons(addons: &Value) {
    let map: &Map<String, Value> = addons.as_object().unwrap();

    for (k, v) in map {
        let addon_path = env::var("USERPROFILE").unwrap() + "\\Documents\\Melies\\addons\\" + k + ".json";
        fs::write(addon_path, v.to_string()).unwrap();
    }
}

#[command]
fn save_settings(new_settings: String) -> Value {
    let settings: Value = serde_json::from_str(&new_settings).unwrap();
    let settings_path = env::var("USERPROFILE").unwrap() + "\\Documents\\Melies\\settings.json";

    if Path::new(&settings_path).exists() {
        let mut defaults = default_settings();

        save_addons(&settings["addons"]);

        merge(&mut defaults, settings);

        fs::write(settings_path, defaults.to_string()).unwrap();

        return defaults;
    }

    build_settings()
}

#[command]
fn load_events() -> Value {
    let settings = load_settings();

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

    let file_text = match fs::read_to_string(dir) {
        Ok(text) => { text }
        Err(err) => {
            return json!({
                "code": 400,
                "err_text": err.to_string()
            });
        }
    };

    let re = Regex::new("\\[(.*)\\] (.*) \\(\"(.*)\" at (\\d*)\\)").unwrap();

    let events_regex = re.captures_iter(&file_text);

    let mut events = vec![];

    for event_capture in events_regex {
        let event = event::Event::new(event_capture).unwrap();

        events.push(event);
    }

    json!({
        "code": 200,
        "events": events
    })
}

#[command]
fn save_events(new_events: Value) -> Value {
    let mut events: Vec<Event> = vec![];
    let mut new_events_text = String::new();

    for demo in new_events.as_array().unwrap() {
        extend!(new_events_text, "{}\r\n", ">");

        for event in demo.as_array().unwrap() {
            let re = Regex::new("\\[(.*)\\] (.*) \\(\"(.*)\" at (\\d*)\\)").unwrap();

            println!("{}", event["event"]);

            let events_regex = match re.captures(event["event"].as_str().unwrap()) {
                Some(val) => val,
                None => {
                    continue;
                }
            };

            let original_event = event::Event::new(events_regex).unwrap();

            if event["demo_name"].as_str().unwrap() != original_event.demo_name {
                let built_event = build_event_from_json(event);
                extend!(new_events_text, "{}\r\n", built_event.event);
                events.push(built_event);
                continue;
            }

            if event["tick"].as_i64().unwrap() != original_event.tick {
                let built_event = build_event_from_json(event);
                extend!(new_events_text, "{}\r\n", built_event.event);
                events.push(built_event);
                continue;
            }

            match &original_event.value {
                Bookmark(bm) => {
                    if event["isKillstreak"].as_bool().unwrap() {
                        let built_event = build_event_from_json(event);
                        extend!(new_events_text, "{}\r\n", built_event.event);
                        events.push(built_event);
                        continue;
                    }

                    if bm.to_owned() != event["value"]["Bookmark"].as_str().unwrap() {
                        let built_event = build_event_from_json(event);
                        extend!(new_events_text, "{}\r\n", built_event.event);
                        events.push(built_event);
                        continue;
                    }
                }
                Killstreak(ks) => {
                    if !event["isKillstreak"].as_bool().unwrap() {
                        let built_event = build_event_from_json(event);
                        extend!(new_events_text, "{}\r\n", built_event.event);
                        events.push(built_event);
                        continue;
                    }

                    if ks.to_owned() != event["value"]["Killstreak"].as_i64().unwrap() {
                        let built_event = build_event_from_json(event);
                        extend!(new_events_text, "{}\r\n", built_event.event);
                        events.push(built_event);
                        continue;
                    }
                }
            }

            extend!(new_events_text, "{}\r\n", original_event.event);
            events.push(original_event);
        }
    }

    let settings = load_settings();

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

    fs::write(dir, new_events_text).unwrap();

    return json!({
        "code": 200,
        "events": events
    });
}

fn build_event_from_json(event_json: &Value) -> Event {
    let sys_time: DateTime<Local> = Local::now();

    match event_json["isKillstreak"].as_bool().unwrap() {
        true => {
            return Event {
                event: format!(
                    "[{}] Killstreak {} (\"{}\" at {})",
                    sys_time.format("%Y/%m/%d %H:%M").to_string().replace("\"", ""),
                    event_json["value"]["Killstreak"],
                    event_json["demo_name"].as_str().unwrap(),
                    event_json["tick"].as_i64().unwrap()
                ),
                demo_name: event_json["demo_name"].as_str().unwrap().to_string(),
                tick: event_json["tick"].as_i64().unwrap(),
                value: Killstreak(event_json["value"]["Killstreak"].as_i64().unwrap()),
            };
        }
        false => {
            if event_json["value"]["Bookmark"] == "General" {
                return Event {
                    event: format!(
                        "[{}] Bookmark {} (\"{}\" at {})",
                        sys_time.format("%Y/%m/%d %H:%M").to_string(),
                        event_json["value"]["Bookmark"].as_str().unwrap(),
                        event_json["demo_name"].as_str().unwrap(),
                        event_json["tick"].as_i64().unwrap()
                    ),
                    demo_name: event_json["demo_name"].as_str().unwrap().to_string(),
                    tick: event_json["tick"].as_i64().unwrap(),
                    value: Bookmark(event_json["value"]["Bookmark"].as_str().unwrap().to_string()),
                };
            }

            return Event {
                event: format!(
                    "[{}] {} (\"{}\" at {})",
                    sys_time.format("%Y/%m/%d %H:%M").to_string(),
                    event_json["value"]["Bookmark"].as_str().unwrap(),
                    event_json["demo_name"].as_str().unwrap(),
                    event_json["tick"].as_i64().unwrap()
                ),
                demo_name: event_json["demo_name"].as_str().unwrap().to_string(),
                tick: event_json["tick"].as_i64().unwrap(),
                value: Bookmark(event_json["value"]["Bookmark"].as_str().unwrap().to_string()),
            };
        }
    }
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

    let output_path = format!(
        "{}\\Documents\\Melies\\backups\\{}.txt",
        env::var("USERPROFILE").unwrap(),
        date
    );

    fs::create_dir_all(
        format!("{}\\Documents\\Melies\\backups", env::var("USERPROFILE").unwrap())
    ).unwrap();

    fs::copy(dir, &output_path).unwrap();

    json!({
        "code": 200,
        "output_path": output_path
    })
}

#[command]
fn parse_log(url: Value) -> Value {
    parse(url)
}

#[command]
fn load_demos() -> Result<Value, String> {
    let settings = load_settings();
    if validate_demos_folder(&settings) {
        return Ok(scan_for_demos(settings));
    }

    return Err(String::from("Can't find \\tf folder. Please fix the \"\\tf Folder\" setting in settings."));
}

#[command]
fn parse_demo(path: String) -> Value {
    scan_demo(load_settings(), path)
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![
                ryukbot,
                load_settings,
                save_settings,
                load_events,
                save_events,
                parse_log,
                load_demos,
                parse_demo
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
