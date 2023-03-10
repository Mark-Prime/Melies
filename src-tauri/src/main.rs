use std::{fs};
use json::{self, JsonValue};
use regex::Regex;
use tauri::command;

use crate::clip::Clip;

mod event;
mod clip;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn find_dir(settings: JsonValue) -> Result<String, String> {
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

    match find_dir(settings) {
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
    
    let mut event_count = 0;

    let re = Regex::new("\\[(.*)\\] (.*) \\(\"(.*)\" at (\\d*)\\)").unwrap();

    let events = re.captures_iter(&file_text);

    let mut clips: Vec<Clip> = vec![];

    for event_capture in events {
        event_count = event_count +  1;

        let event = event::Event::new(event_capture).unwrap();

        println!("{}", &event);

        if clips.len() == 0 {
            clips.push(Clip::new(event));
            continue;
        }

        clips[clips.len()-1].can_include(event);
    }

    format!("_events.txt contains {} events", event_count)

    // format!("{}\\demos\\_events.txt", settings["tf_folder"])
}

#[command]
fn load_settings() -> Result<String, String> {
    let file = fs::read_to_string("settings.json").unwrap();
    let settings = json::parse(&file).unwrap();

    return Ok(settings.dump());
}

#[command]
fn save_settings(new_settings: String) -> Result<String, String> {
    let settings = json::parse(&new_settings).unwrap();
    fs::write("settings.json", settings.pretty(4)).unwrap();
    return Ok(settings.dump());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ryukbot, load_settings, save_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
