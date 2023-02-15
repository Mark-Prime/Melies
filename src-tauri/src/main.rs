use std::fs;
use json;
use regex::Regex;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet() -> String {
    let file = fs::read_to_string("settings.json").unwrap();
    let settings = json::parse(&file).unwrap();

    
    let re = Regex::new("\\[(.*)\\] (.*) \\(\"(.*)\" at (\\d*)\\)").unwrap();

    let file_text = match fs::read_to_string(format!("{}\\demos\\_events.txt", settings["tf_folder"])) {
        Ok(text) => {
            text
        },
        Err(err) => {
            return err.to_string();
        }
    };
    
    let mut event_count = 0;

    let events = re.captures_iter(&file_text);

    for event in events {
        event_count = event_count +  1;
        println!("{}", &event[0]);
    }

    format!("_events.txt contains {} events", event_count)

    // format!("{}\\demos\\_events.txt", settings["tf_folder"])
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
