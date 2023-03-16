use std::fs::File;
use std::io::Write;
use std::{fs};
use json::{self, JsonValue};
use regex::Regex;
use tauri::command;
use vdm::VDM;
use vdm::action::ActionType;

use crate::clip::Clip;

mod event;
mod clip;

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

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn write_cfg(settings: &JsonValue) {
    let mut cfg = String::new();

    // println!("cl_drawhud {}", settings["output"]["HUD"]);

    extend!(cfg, "echo \"Execing Melies Config\";\r\ncl_drawhud {};\r\n", settings["output"]["HUD"]);
    extend!(cfg, "voice_enable {};\r\n", settings["output"]["voice_chat"]);
    extend!(cfg, "hud_saytext_time {};\r\n", settings["output"]["text_chat"]);
    extend!(cfg, "crosshair {};\r\n", settings["output"]["crosshair"]);
    extend!(cfg, "{};\r\n", settings["recording"]["commands"]);

    if settings["output"]["lock"].as_i64().unwrap() == 1 {
        extend!(cfg, "\r\necho \"Preventing settings from changing\";\r\nalias cl_drawhud \"{}\";\r\n", "");
        extend!(cfg, "alias voice_enable \"{}\";\r\n", "");
        extend!(cfg, "alias hud_saytext_time \"{}\";\r\n", "");
        extend!(cfg, "alias crosshair \"{}\";\r\n", "");
    }

    let mut file = File::create(format!("{}\\cfg\\melies.cfg", settings["tf_folder"])).unwrap();

    file.write_all(cfg.as_bytes()).unwrap();
}

fn end_vdm(vdm: &mut VDM, settings: &JsonValue, next_demoname: String) -> VDM {
    println!("{}", vdm.name);
    let last_tick = vdm.last().props().start_tick.unwrap();

    {
        let mut exec_commands = vdm.create_action(ActionType::PlayCommands).props_mut();

        exec_commands.start_tick = Some(last_tick + 100);

        if settings["recording"]["record_continuous"] == 1 && next_demoname != "" {
            exec_commands.name = format!("Start the next demo ({}.dem)", next_demoname);
            exec_commands.commands = format!("playdemo {};", next_demoname);
            return vdm.to_owned();
        }

        exec_commands.name = "Exit TF2".to_string();
        exec_commands.commands = "quit;".to_string();
    }

    vdm.to_owned()
}

fn start_vdm(vdm: &mut VDM, clip: &Clip, settings: &JsonValue) {
    if clip.start_tick > settings["recording"]["start_delay"].as_i64().unwrap() {
        let mut skip_props = vdm.create_action(ActionType::SkipAhead).props_mut();

        skip_props.start_tick = Some(settings["recording"]["start_delay"].as_i64().unwrap());
        skip_props.skip_to_tick = Some(clip.start_tick - 100);
    }

    record_clip(vdm, clip, settings);
}

fn add_clip_to_vdm(vdm: &mut VDM, clip: &Clip, settings: &JsonValue) {
    let last_tick = vdm.last().props().start_tick.unwrap();
    
    if clip.start_tick > last_tick + 300 {
        let mut skip_props = vdm.create_action(ActionType::SkipAhead).props_mut();

        skip_props.start_tick = Some(last_tick + 100);
        skip_props.skip_to_tick = Some(clip.start_tick - 100);
    }

    record_clip(vdm, clip, settings);
}

fn record_clip(vdm: &mut VDM, clip: &Clip, settings: &JsonValue) {
    let vdm_name = vdm.name.clone();

    let mut suffix = "bm".to_string();

    if clip.has_killstreak {
        suffix = format!("ks{}", clip.ks_value);

        if clip.has_bookmark {
            suffix = format!("bm{}+", clip.ks_value);
        }
    }
    
    {
        let mut exec_commands = vdm.create_action(ActionType::PlayCommands).props_mut();

        exec_commands.start_tick = Some(clip.start_tick - 50);
        exec_commands.name = "Exec Melies Commands".to_string();
        exec_commands.commands = "exec melies;".to_string();
    }

    {
        let mut start_record = vdm.create_action(ActionType::PlayCommands).props_mut();

        let commands = format!(
            "{}host_framerate {}; startmovie {} {}; clear;",
            ifelse!(settings["output"]["snd_fix"] == 1, "snd_restart; ", ""),
            settings["output"]["framerate"],
            format!("{}_{}-{}_{}", vdm_name, clip.start_tick, clip.end_tick, suffix),
            settings["output"]["method"]
        );

        start_record.start_tick = Some(clip.start_tick);
        start_record.name = "Start Recording".to_string();
        start_record.commands = commands;
    }

    {
        let mut end_record = vdm.create_action(ActionType::PlayCommands).props_mut();

        end_record.start_tick = Some(clip.end_tick);
        end_record.name = "Stop Recording".to_string();
        end_record.commands = format!(
            "{}; host_framerate 0; endmovie;",
            settings["recording"]["end_commands"],
        );
    }
}

fn find_dir(settings: &JsonValue) -> Result<String, String> {
    for entry in fs::read_dir(format!("{}\\demos", settings["tf_folder"])).unwrap() {
        let dir = entry.unwrap();
        let dir_str = dir.path().to_string_lossy().to_string();

        if dir_str.ends_with("\\_events.txt") {
            return Ok(dir_str);
        }

        if dir_str.ends_with("\\KillStreaks.txt") {
            return Ok(dir_str);
        }
    }

    for entry in fs::read_dir(format!("{}", settings["tf_folder"])).unwrap() {
        let dir = entry.unwrap();
        let dir_str = dir.path().to_string_lossy().to_string();

        if dir_str.ends_with("\\_events.txt") {
            return Ok(dir_str);
        }

        if dir_str.ends_with("\\KillStreaks.txt") {
            return Ok(dir_str);
        }
    }
    Err(format!("File Not Found: Please ensure the setting tf_folder is correct: ({})", settings["tf_folder"]))
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
fn ryukbot() -> String {
    let file = fs::read_to_string("settings.json").unwrap();
    let settings = json::parse(&file).unwrap();

    let dir;

    match find_dir(&settings) {
        Ok(directory) => {
            dir = directory;
        },
        Err(err) => {
            return err;
        }
    };

    let file_text = match fs::read_to_string(dir) {
        Ok(text) => {
            text
        },
        Err(err) => {
            return err.to_string();
        }
    };

    write_cfg(&settings);
    
    let mut event_count = 0;

    let re = Regex::new("\\[(.*)\\] (.*) \\(\"(.*)\" at (\\d*)\\)").unwrap();

    let events = re.captures_iter(&file_text);

    let mut clips: Vec<Clip> = vec![];

    for event_capture in events {
        event_count = event_count +  1;

        let event = event::Event::new(event_capture).unwrap();

        // println!("{}", &event);

        if clips.len() == 0 {
            clips.push(Clip::new(event, &settings));
            continue;
        }

        if clips.last().unwrap().can_include(&event, &settings) {
            clips.last_mut().unwrap().include(event, &settings);
            // println!("THESE CAN COMBINE!");
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

    for (i, vdm) in vdms.iter().enumerate() {
        let vdm = end_vdm(&mut vdm.clone(), &settings, ifelse!(vdms.len() > i + 1, String::from(&vdms[i + 1].name), String::new()));
        vdm.export(&format!("{}\\demos\\{}.vdm", settings["tf_folder"], vdm.name));
    }

    format!("_events.txt contains {} clips from {} events", clips.len(), event_count)
}

#[command]
fn load_settings() -> Result<String, String> {
    let file = fs::read_to_string("settings.json").unwrap();
    let settings = json::parse(&file).unwrap();

    Ok(settings.dump())
}

#[command]
fn save_settings(new_settings: String) -> Result<String, String> {
    let settings = json::parse(&new_settings).unwrap();
    fs::write("settings.json", settings.pretty(4)).unwrap();
    Ok(settings.dump()) 
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ryukbot, load_settings, save_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
