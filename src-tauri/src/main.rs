#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use chrono::prelude::*;
use regex::Regex;
use serde_json::{ self, json, Map, Value };
use std::fs::{ DirEntry, File };
use std::io::Write;
use std::path::Path;
use std::{ env, fs };
use tauri::command;
use vdm::action::{ Action, ActionType };
use vdm::VDM;

use crate::clip::Clip;
use crate::demos::{ scan_demo, scan_for_demos, scan_for_vdms, validate_demos_folder };
use crate::event::Event;
use crate::event::EventStyle::{ Bookmark, Killstreak };
use crate::logstf::parse;

mod clip;
mod demos;
mod event;
mod logstf;

macro_rules! ifelse {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {
            $v
        } else {
            $v1
        }
    };
}

macro_rules! extend {
    ($v:expr, $v1:expr, $c:expr) => {
        $v = format!("{}{}", $v, format!($v1, $c))
    };
}

fn setting_as_bin(setting: &Value) -> i64 {
    if !setting.is_boolean() {
        if let Some(setting_i64) = setting.as_i64() {
            return setting_i64;
        }
    }

    match setting.as_bool() {
        Some(val) =>
            match val {
                true => 1,
                false => 0,
            }
        _ => 0,
    }
}

fn setting_as_bool(setting: &Value) -> bool {
    if setting.is_boolean() {
        return match setting.as_bool() {
            Some(val) => val,
            None => false,
        };
    }

    match setting.as_i64() {
        Some(val) =>
            match val {
                1 => true,
                _ => false,
            }
        _ => false,
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
    extend!(cfg, "tf_use_min_viewmodels {};\r\n", setting_as_bin(&settings["output"]["minmode"]));
    extend!(
        cfg,
        "viewmodel_fov_demo {};\r\n",
        setting_as_bin(&settings["recording"]["viewmodel_fov"])
    );
    extend!(cfg, "fov_desired {};\r\n", setting_as_bin(&settings["recording"]["fov"]));

    if setting_as_bin(&settings["recording"]["third_person"]) == 1 {
        extend!(cfg, "thirdperson{};\r\n", "");
    } else {
        extend!(cfg, "firstperson{};\r\n", "");
    }

    if let Some(commands) = settings["recording"]["commands"].as_str() {
        extend!(cfg, "{}\r\n", commands);
    }

    if setting_as_bin(&settings["recording"]["prevent_taunt"]) == 1 {
        extend!(cfg, "{}\r\n", "alias +taunt \"\"; alias -taunt \"\";");
    }

    extend!(cfg, "{};\r\n", "alias \"snd_fix\" \"snd_restart; snd_soundmixer Default_mix;\"");

    let addons = settings["addons"].as_object();

    match addons {
        Some(map) => {
            for (k, v) in map {
                extend!(cfg, "\r\necho \"Running {} addon\";\r\n", k);

                let v_map = match v.as_object() {
                    Some(v_map) => v_map,
                    None => {
                        continue;
                    }
                };

                for (_ki, vi) in v_map {
                    match &vi["type"] {
                        Value::String(vi_type) =>
                            match vi_type.as_str() {
                                "toggle" => {
                                    if vi["value"] == true {
                                        extend!(cfg, "{};\r\n", vi["command"]);
                                    }
                                }
                                "bool" => {
                                    extend!(
                                        cfg,
                                        "{};\r\n",
                                        format!(
                                            "{} {}",
                                            vi["command"].to_string().replace("\"", ""),
                                            setting_as_bin(&vi["value"])
                                        )
                                    );
                                }
                                "string" | "int" => {
                                    extend!(
                                        cfg,
                                        "{};\r\n",
                                        format!(
                                            "{} {}",
                                            vi["command"].to_string().replace("\"", ""),
                                            vi["value"]
                                        )
                                    );
                                }
                                _ => {
                                    continue;
                                }
                            }
                        _ => {
                            continue;
                        }
                    }
                }
            }
        }
        None => {}
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
        extend!(cfg, "alias tf_use_min_viewmodels \"{}\";\r\n", "");
        extend!(cfg, "alias fov_desired \"{}\";\r\n", "");
    }

    let tf_folder = match settings["tf_folder"].as_str() {
        Some(folder) => folder,
        None => panic!("tf_folder setting is not a string"),
    };

    let mut file = match File::create(format!("{}\\cfg\\melies.cfg", tf_folder)) {
        Ok(file) => file,
        Err(why) => { panic!("Couldn't create melies.cfg: {}", why) }
    };

    match file.write_all(cfg.as_bytes()) {
        Ok(_) => {}
        Err(why) => panic!("Couldn't write melies.cfg: {}", why),
    };
}

fn end_vdm(vdm: &mut VDM, settings: &Value, next_demoname: String) -> VDM {
    let last_tick = match vdm.last().props().start_tick {
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

fn check_spec(clip: &Clip, commands: String) -> String {
    if clip.spec_type == 0 {
        return commands;
    }

    let mut new_commands = commands;

    new_commands = format!(
        "{}; spec_player {}; spec_mode {};",
        new_commands,
        clip.spec_player,
        ifelse!(clip.spec_type == 1, 4, 5)
    );

    return new_commands;
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
        }
    }

    record_clip(vdm, clip, settings);
}

fn add_clip_to_vdm(vdm: &mut VDM, clip: &Clip, settings: &Value) {
    let last_tick = match vdm.last().props().start_tick {
        Some(action) => action,
        None => {
            return;
        }
    };

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
        exec_commands.commands = format!(
            "exec melies;{}",
            ifelse!(setting_as_bool(&settings["output"]["snd_fix"]), " snd_fix;", "")
        );
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
            clip_name = format!(
                "{}_{}",
                clip_name,
                bm_value.replace("spec", "").trim().replace(" ", "-")
            );
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

        commands = format!("{} spec_mode 7;", commands);

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

    Err(format!("File Not Found"))
}

fn find_dir(settings: &Value) -> Result<String, String> {
    let tf_folder = settings["tf_folder"].as_str();

    let files = match tf_folder {
        Some(tf_folder) => fs::read_dir(format!("{}\\demos", tf_folder)),
        None => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "tf_folder not set")),
    };

    match check_dir(files) {
        Ok(res) => {
            return Ok(res);
        }
        Err(_) => {}
    }

    let tf_folder = settings["tf_folder"].as_str();
    let files = match tf_folder {
        Some(tf_folder) => fs::read_dir(format!("{}", tf_folder)),
        None => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "tf_folder not set")),
    };

    match check_dir(files) {
        Ok(res) => {
            return Ok(res);
        }
        Err(_) => {}
    }

    match settings["tf_folder"].as_str() {
        Some(tf_folder) =>
            Err(
                format!("Could not find the _events.txt or KillStreaks.txt files.\r\nPlease check your settings to ensure the tf folder is correctly linked.\r\nIf you do not have either file, please make one in the \\tf or \\tf\\demos folder. \r\n\r\ntf_folder setting: ({})", tf_folder)
            ),
        None => Err("tf_folder setting not set".to_string()),
    }
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
            "err_text": format!("_events.txt or KillStreaks.txt was found but had no valid events. Please add events before running again.\r\n\r\nFile Location: {}", dir)
        });
    }

    write_cfg(&settings);

    let mut clips: Vec<Clip> = vec![];

    let mut event_count = 0;

    let events = re.captures_iter(&file_text);

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
        let mut folder = format!("{}\\demos", &settings["tf_folder"].as_str().unwrap());

        if !Path::new(&folder).exists() {
            folder = format!("{}", &settings["tf_folder"].as_str().unwrap());
        }

        let file_location = format!("{}\\{}.vdm", folder, &vdm.name);

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

    let files = fs
        ::read_dir(format!("{}\\Documents\\Melies\\addons", env::var("USERPROFILE").unwrap()))
        .unwrap();

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
            }
        };

        addons[name] = res;
    }

    addons
}

fn default_settings() -> Value {
    let defaults =
        json!({
        "tf_folder": "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Team Fortress 2\\tf",
        "clear_events": true,
        "save_backups": true,
        "safe_mode": true,
        "output": {
            "folder": "",
            "method": "tga",
            "framerate": 60,
            "crosshair": false,
            "viewmodel": true,
            "hud": true,
            "text_chat": false,
            "voice_chat": false,
            "minmode": false,
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
            "third_person": false,
            "prevent_taunt": false
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
        let addon_path =
            env::var("USERPROFILE").unwrap() + "\\Documents\\Melies\\addons\\" + k + ".json";
        fs::write(addon_path, serde_json::to_string_pretty(v).unwrap()).unwrap();
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

        fs::write(settings_path, serde_json::to_string_pretty(&defaults).unwrap()).unwrap();

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
        Ok(text) => text,
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
fn load_vdms() -> Result<Value, String> {
    let settings = load_settings();
    if validate_demos_folder(&settings) {
        return Ok(scan_for_vdms(settings));
    }

    Err(String::from("Can't find \\tf folder. Please fix the \"\\tf Folder\" setting in settings."))
}

#[command]
fn load_demos() -> Result<Value, String> {
    let settings = load_settings();
    if validate_demos_folder(&settings) {
        return Ok(scan_for_demos(settings));
    }

    Err(String::from("Can't find \\tf folder. Please fix the \"\\tf Folder\" setting in settings."))
}

pub(crate) fn validate_backups_folder() -> bool {
    match fs::read_dir(format!("{}\\Documents\\Melies\\backups", env::var("USERPROFILE").unwrap())) {
        Ok(_) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}

pub(crate) fn scan_for_backups() -> Value {
    let mut events: Vec<Value> = vec![];

    for entry in fs
        ::read_dir(format!("{}\\Documents\\Melies\\backups", env::var("USERPROFILE").unwrap()))
        .unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.file_name().unwrap().to_str().unwrap().ends_with(".txt") {
            events.push(
                json!({
                    "file_name": path.file_name().unwrap().to_str().unwrap().to_string(),
                })
            );
        }
    }

    json!(events)
}

#[command]
fn load_backups() -> Result<Value, String> {
    if validate_backups_folder() {
        return Ok(scan_for_backups());
    }

    Err(String::from("Can't find backups folder. You may not have backups yet."))
}

#[command]
fn reload_backup(file_name: Value) -> Result<Value, String> {
    let file_path = format!(
        "{}\\Documents\\Melies\\backups\\{}",
        env::var("USERPROFILE").unwrap(),
        file_name.as_str().unwrap()
    );
    let backup_file = Path::new(&file_path);

    let settings = load_settings();

    let dir;

    match find_dir(&settings) {
        Ok(directory) => {
            dir = directory;
        }
        Err(_) => {
            return Err(String::from("Failed to reload backup."));
        }
    }

    if backup_file.exists() {
        let copy = fs::copy(file_path, dir);

        if copy.is_ok() {
            return Ok(json!("Successfully loaded backup."));
        }

        return Err(String::from("Failed to reload backup."));
    }

    Err(String::from("Failed to find backup file."))
}

#[command]
fn parse_demo(path: String) -> Value {
    scan_demo(load_settings(), path)
}

fn vdm_to_json(vdm: VDM) -> Value {
    let mut actions: Vec<Value> = vec![];

    for action in vdm.actions {
        match action {
            Action::SkipAhead(props) => {
                actions.push(
                    json!({
                        "factory": "SkipAhead",
                        "name": props.name,
                        "start_tick": props.start_tick,
                        "start_time": props.start_time,
                        "skip_to_tick": props.skip_to_tick,
                        "skip_to_time": props.skip_to_time
                    })
                );
            }
            Action::StopPlayback(props) => {
                actions.push(
                    json!({
                        "factory": "StopPlayback",
                        "name": props.name,
                        "start_tick": props.start_tick,
                        "start_time": props.start_time,
                    })
                );
            }
            Action::PlayCommands(props) => {
                actions.push(
                    json!({
                        "factory": "PlayCommands",
                        "name": props.name,
                        "start_tick": props.start_tick,
                        "start_time": props.start_time,
                        "commands": props.commands
                    })
                );
            }
            Action::ScreenFadeStart(props) => {
                actions.push(
                    json!({
                        "factory": "ScreenFadeStart",
                        "name": props.name,
                        "start_tick": props.start_tick,
                        "start_time": props.start_time,
                        "duration": props.duration,
                        "hold_time": props.hold_time,
                        "fade_in_enabled": props.fade_in_enabled,
                        "fade_out_enabled": props.fade_out_enabled,
                        "modulate_enabled": props.modulate_enabled,
                        "stay_out_enabled": props.stay_out_enabled,
                        "purge_enabled": props.purge_enabled,
                        "rgba1": props.rgba1,
                    })
                );
            }
            Action::TextMessageStart(props) => {
                actions.push(
                    json!({
                        "factory": "TextMessageStart",
                        "name": props.name,
                        "start_tick": props.start_tick,
                        "start_time": props.start_time,
                        "message": props.message,
                        "font": props.font,
                        "fade_in": props.fade_in,
                        "fade_out": props.fade_out,
                        "hold_time": props.hold_time,
                        "fx_time": props.fx_time,
                        "effect": match props.effect {
                            vdm::action::TextEffect::Flicker => "Flicker",
                            vdm::action::TextEffect::FadeInOut => "FadeInOut",
                            vdm::action::TextEffect::WriteOut => "WriteOut",
                        },
                        "xy": props.xy,
                        "rgba1": props.rgba1,
                        "rgba2": props.rgba2,
                    })
                );
            }
            Action::PlayCDTrackStart(props) => {
                actions.push(
                    json!({
                        "factory": "PlayCDTrackStart",
                        "name": props.name,
                        "start_tick": props.start_tick,
                        "start_time": props.start_time,
                        "track": props.track,
                    })
                );
            }
            Action::PlaySoundStart(props) => {
                actions.push(
                    json!({
                        "factory": "PlaySoundStart",
                        "name": props.name,
                        "start_tick": props.start_tick,
                        "start_time": props.start_time,
                        "sound": props.sound,
                    })
                );
            }
            Action::Pause(props) => {
                actions.push(
                    json!({
                        "factory": "Pause",
                        "name": props.name,
                        "start_tick": props.start_tick,
                        "start_time": props.start_time,
                        "stop_tick": props.stop_tick,
                        "stop_time": props.stop_time,
                        "duration": props.duration
                    })
                );
            }
            Action::ChangePlaybackRate(props) => {
                actions.push(
                    json!({
                        "factory": "ChangePlaybackRate",
                        "name": props.name,
                        "start_tick": props.start_tick,
                        "start_time": props.start_time,
                        "stop_tick": props.stop_tick,
                        "stop_time": props.stop_time,
                        "playback_rate": props.playback_rate
                    })
                );
            }
            Action::ZoomFov(props) => {
                actions.push(
                    json!({
                        "factory": "ZoomFov",
                        "name": props.name,
                        "start_tick": props.start_tick,
                        "start_time": props.start_time,
                        "spline": props.spline,
                        "stayout": props.stayout,
                        "final_fov": props.final_fov,
                        "fov_fade_out": props.fade_out,
                        "fov_fade_in": props.fade_in,
                        "fov_hold": props.hold_time,
                    })
                );
            }
        }
    }

    json!(actions)
}

fn json_to_vdm(json: Value) -> VDM {
    let mut vdm = VDM::new();

    for action in json.as_array().unwrap() {
        let factory = action["factory"].as_str().unwrap();

        let factory = match factory {
            "SkipAhead" => ActionType::SkipAhead,
            "StopPlayback" => ActionType::StopPlayback,
            "PlayCommands" => ActionType::PlayCommands,
            "ScreenFadeStart" => ActionType::ScreenFadeStart,
            "TextMessageStart" => ActionType::TextMessageStart,
            "PlayCDTrackStart" => ActionType::PlayCDTrackStart,
            "PlaySoundStart" => ActionType::PlaySoundStart,
            "Pause" => ActionType::Pause,
            "ChangePlaybackRate" => ActionType::ChangePlaybackRate,
            "ZoomFov" => ActionType::ZoomFov,
            _ => panic!("Invalid action type."),
        };

        let mut new_action = Action::new(factory.clone());

        let props = new_action.props_mut();

        props.name = action["name"].as_str().unwrap().to_string();

        if action["start_tick"] != Value::Null {
            props.start_tick = Some(action["start_tick"].as_i64().unwrap());
        }

        if action["start_time"] != Value::Null {
            props.start_time = Some(action["start_time"].as_f64().unwrap());
        }

        match factory {
            ActionType::SkipAhead => {
                if action["skip_to_tick"] != Value::Null {
                    props.skip_to_tick = Some(action["skip_to_tick"].as_i64().unwrap());
                }

                if action["skip_to_time"] != Value::Null {
                    props.skip_to_time = Some(action["skip_to_time"].as_f64().unwrap());
                }
            }
            ActionType::StopPlayback => {}
            ActionType::PlayCommands => {
                if action["commands"] != Value::Null {
                    props.commands = action["commands"].as_str().unwrap().to_string();
                }
            }
            ActionType::ScreenFadeStart => {
                if action["duration"] != Value::Null {
                    props.duration = action["duration"].as_f64().unwrap();
                }

                if action["hold_time"] != Value::Null {
                    props.hold_time = action["hold_time"].as_f64().unwrap();
                }

                if action["fade_in_enabled"] != Value::Null {
                    props.fade_in_enabled = action["fade_in_enabled"].as_bool().unwrap();
                }

                if action["fade_out_enabled"] != Value::Null {
                    props.fade_out_enabled = action["fade_out_enabled"].as_bool().unwrap();
                }

                if action["modulate_enabled"] != Value::Null {
                    props.modulate_enabled = action["modulate_enabled"].as_bool().unwrap();
                }

                if action["stay_out_enabled"] != Value::Null {
                    props.stay_out_enabled = action["stay_out_enabled"].as_bool().unwrap();
                }

                if action["purge_enabled"] != Value::Null {
                    props.purge_enabled = action["purge_enabled"].as_bool().unwrap();
                }

                if action["rgba1"] != Value::Null {
                    props.rgba1 = [
                        action["rgba1"][0].as_u64().unwrap() as u8,
                        action["rgba1"][1].as_u64().unwrap() as u8,
                        action["rgba1"][2].as_u64().unwrap() as u8,
                        action["rgba1"][3].as_u64().unwrap() as u8,
                    ];
                }
            }
            ActionType::TextMessageStart => {
                if action["message"] != Value::Null {
                    props.message = action["message"].as_str().unwrap().to_string();
                }

                if action["font"] != Value::Null {
                    props.font = action["font"].as_str().unwrap().to_string();
                }

                if action["fade_in"] != Value::Null {
                    props.fade_in = action["fade_in"].as_f64().unwrap();
                }

                if action["fade_out"] != Value::Null {
                    props.fade_out = action["fade_out"].as_f64().unwrap();
                }

                if action["hold_time"] != Value::Null {
                    props.hold_time = action["hold_time"].as_f64().unwrap();
                }

                if action["fx_time"] != Value::Null {
                    props.fx_time = action["fx_time"].as_f64().unwrap();
                }

                if action["effect"] != Value::Null {
                    props.effect = match action["effect"].as_str().unwrap() {
                        "Flicker" => vdm::action::TextEffect::Flicker,
                        "FadeInOut" => vdm::action::TextEffect::FadeInOut,
                        "WriteOut" => vdm::action::TextEffect::WriteOut,
                        _ => panic!("Invalid text effect."),
                    };
                }

                if action["xy"] != Value::Null {
                    props.xy = [
                        action["xy"][0].as_f64().unwrap(),
                        action["xy"][1].as_f64().unwrap(),
                    ];
                }

                if action["rgba1"] != Value::Null {
                    props.rgba1 = [
                        action["rgba1"][0].as_u64().unwrap() as u8,
                        action["rgba1"][1].as_u64().unwrap() as u8,
                        action["rgba1"][2].as_u64().unwrap() as u8,
                        action["rgba1"][3].as_u64().unwrap() as u8,
                    ];
                }

                if action["rgba2"] != Value::Null {
                    props.rgba2 = [
                        action["rgba2"][0].as_u64().unwrap() as u8,
                        action["rgba2"][1].as_u64().unwrap() as u8,
                        action["rgba2"][2].as_u64().unwrap() as u8,
                        action["rgba2"][3].as_u64().unwrap() as u8,
                    ];
                }
            }
            ActionType::PlayCDTrackStart => {
                if action["track"] != Value::Null {
                    props.track = action["track"].as_i64().unwrap();
                }
            }
            ActionType::PlaySoundStart => {
                if action["sound"] != Value::Null {
                    props.sound = action["sound"].as_str().unwrap().to_string();
                }
            }
            ActionType::Pause => {
                if action["stop_tick"] != Value::Null {
                    props.stop_tick = Some(action["stop_tick"].as_i64().unwrap());
                }

                if action["stop_time"] != Value::Null {
                    props.stop_time = Some(action["stop_time"].as_f64().unwrap());
                }

                if action["duration"] != Value::Null {
                    props.duration = action["duration"].as_f64().unwrap();
                }
            }
            ActionType::ChangePlaybackRate => {
                if action["stop_tick"] != Value::Null {
                    props.stop_tick = Some(action["stop_tick"].as_i64().unwrap());
                }

                if action["stop_time"] != Value::Null {
                    props.stop_time = Some(action["stop_time"].as_f64().unwrap());
                }

                if action["playback_rate"] != Value::Null {
                    props.playback_rate = action["playback_rate"].as_f64().unwrap();
                }
            }
            ActionType::ZoomFov => {
                if action["spline"] != Value::Null {
                    props.spline = action["spline"].as_bool().unwrap();
                }

                if action["stayout"] != Value::Null {
                    props.stayout = action["stayout"].as_bool().unwrap();
                }

                if action["final_fov"] != Value::Null {
                    props.final_fov = action["final_fov"].as_f64().unwrap();
                }

                if action["fov_fade_out"] != Value::Null {
                    props.fade_out = action["fov_fade_out"].as_f64().unwrap();
                }

                if action["fov_fade_in"] != Value::Null {
                    props.fade_in = action["fov_fade_in"].as_f64().unwrap();
                }

                if action["fov_hold"] != Value::Null {
                    props.hold_time = action["fov_hold"].as_f64().unwrap();
                }
            }
        }

        vdm.actions.push(new_action);
    }

    vdm
}

#[command]
fn load_vdm(name: Value) -> Value {
    let settings = load_settings();

    let dir = format!("{}{}", settings["tf_folder"].as_str().unwrap(), name.as_str().unwrap());

    let vdm = VDM::open(&dir).unwrap();

    vdm_to_json(vdm)
}

#[command]
fn save_vdm(name: Value, vdm: Value) -> Value {
    let settings = load_settings();

    let dir = format!("{}{}", settings["tf_folder"].as_str().unwrap(), name.as_str().unwrap());

    let vdm = json_to_vdm(vdm);

    vdm.export(&dir);

    json!({
        "success": true
    })
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
                load_vdm,
                load_vdms,
                save_vdm,
                load_demos,
                load_backups,
                reload_backup,
                parse_demo
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
