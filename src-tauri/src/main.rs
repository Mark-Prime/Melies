use std::fs;
// use std::fs::File;
// use std::io::prelude::*;
use json;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let mut file = fs::read("settings.json").unwrap();
    let mut contents = String::from_utf8_lossy(&file);
    let mut settings = json::parse(&contents).unwrap();

    // println!("{}", settings["message"]);

    format!("Hello, {}! {}", name, settings["message"])
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
