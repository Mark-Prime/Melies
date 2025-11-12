#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use regex::Regex;
use sanitize_filename::sanitize;
use serde_json::{ self, json, Value };
use std::path::{ Path, PathBuf };
use std::process::Command;
use std::{ env, fs };
use tauri::command;
use vdm::VDM;
use trash;

use crate::demos::*;
use crate::logstf::parse;
use crate::vdms::*;
use crate::ryukbot::run_ryukbot;
use crate::util::*;

mod addons;
mod clip;
mod demos;
mod event;
mod logstf;
mod macros;
mod settings;
mod vdms;
mod tf2;
mod rgl;
mod cli;
mod weapons;
mod batch_automation;
mod ryukbot;
mod util;
mod cfg;
mod events;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
fn ryukbot() -> Value {
  run_ryukbot()
}

#[command]
fn load_settings() -> Value {
  settings::load_settings()
}

#[command]
fn save_settings(new_settings: String) -> Value {
  settings::save_settings(new_settings)
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
  crate::events::save_events(new_events)
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
  let user_profile = env::var("USERPROFILE");

  let backups_folder = match user_profile {
    Ok(profile) => { format!("{}\\Documents\\Melies\\backups", profile) }
    Err(_) => {
      format!("{}\\backups", std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap())
    }
  };

  match fs::read_dir(backups_folder) {
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

  let user_profile = env::var("USERPROFILE");

  let backups_folder = match user_profile {
    Ok(profile) => { format!("{}\\Documents\\Melies\\backups", profile) }
    Err(_) => {
      format!("{}\\backups", std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap())
    }
  };

  for entry in fs::read_dir(backups_folder).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();

    if path.is_file() && path.file_name().unwrap().to_str().unwrap().ends_with(".txt") {
      events.push(
        json!({
                "file_name": path.file_name().unwrap().to_str().unwrap().to_string(),
                "created": entry.metadata().unwrap().created().unwrap(),
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
  let user_profile = env::var("USERPROFILE");

  let backups_folder = match user_profile {
    Ok(profile) => { format!("{}\\Documents\\Melies\\backups", profile) }
    Err(_) => {
      format!("{}\\backups", std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap())
    }
  };

  let file_path = format!("{}\\{}", backups_folder, file_name.as_str().unwrap());
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

  println!("Reloading backup: {}", file_path);

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

#[command]
fn open_addons_folder() {
  addons::open_addons_folder();
}

#[command]
fn delete_demo(file_name: Value) {
  let settings = load_settings();

  let file_path = format!(
    "{}{}",
    settings["tf_folder"].as_str().unwrap(),
    file_name.as_str().unwrap()
  );
  let vdm_file_path = format!(
    "{}{}",
    settings["tf_folder"].as_str().unwrap(),
    file_name.as_str().unwrap().replace(".dem", ".vdm")
  );

  let path = Path::new(&file_path);
  let vdm_path = Path::new(&vdm_file_path);

  if path.exists() {
    trash::delete(path).unwrap();
  }

  if vdm_path.exists() {
    trash::delete(vdm_path).unwrap();
  }
}

#[command]
fn delete_vdm(file_name: Value) {
  let settings = load_settings();

  let file_path = format!(
    "{}{}",
    settings["tf_folder"].as_str().unwrap(),
    file_name.as_str().unwrap().replace(".dem", ".vdm")
  );

  let path = Path::new(&file_path);

  if path.exists() {
    trash::delete(path).unwrap();
  }
}

#[command]
fn create_vdm(file_name: Value) {
  let settings = load_settings();

  let file_path = format!(
    "{}{}",
    settings["tf_folder"].as_str().unwrap(),
    file_name.as_str().unwrap().replace(".dem", ".vdm")
  );

  let path = Path::new(&file_path);

  if !path.exists() {
    let vdm = VDM::new();
    vdm.export(&file_path);
  }
}

#[command]
fn load_theme() -> Value {
  let user_profile = env::var("USERPROFILE");

  let settings_path = match user_profile {
    Ok(profile) => { format!("{}\\Documents\\Melies\\theme.json", profile) }
    Err(_) => {
      format!(
        "{}\\theme.json",
        std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap()
      )
    }
  };

  if Path::new(&settings_path).exists() {
    let file = fs::read_to_string(settings_path).unwrap();
    let mut theme: Value = serde_json::from_str(&file).unwrap();

    theme["has_theme"] = json!(true);

    return theme;
  }

  return json!({
        "has_theme": false
    });
}

#[command]
fn open_themes_folder() {
  let user_profile = env::var("USERPROFILE");

  let addons_path = match user_profile {
    Ok(profile) => { format!("{}\\Documents\\Melies", profile) }
    Err(_) => {
      format!("{}", std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap())
    }
  };

  fs::create_dir_all(&addons_path).unwrap();

  Command::new("explorer").arg(addons_path).spawn().unwrap();
}

#[command]
fn open_install_folder(install: &str) {
  let settings = load_settings();
  let tf_folder = settings["tf_folder"].as_str().unwrap();
  let tf_parent = PathBuf::from(tf_folder).parent().unwrap().to_path_buf();

  let relative_path = install.replace(&format!("{}\\", tf_parent.to_str().unwrap()), "");

  let install_folder = format!("{}\\{}", tf_parent.to_str().unwrap(), relative_path);

  fs::create_dir_all(&install_folder).unwrap();

  Command::new("explorer").arg(install_folder).spawn().unwrap();
}

#[command]
fn rename_file(old_path: &str, new_path: &str) {
  let path = Path::new(old_path);
  let new_path = Path::new(new_path);

  let sanitized_path = sanitize_name(new_path);

  fs::rename(path, sanitized_path).unwrap();
}

#[command]
fn cleanup_rename(demo_map: Value) {
  let events_obj = load_events();

  let events = match events_obj["events"].as_array() {
    Some(val) => val.to_owned(),
    None => {
      return;
    }
  };

  save_events(cleanup_renamed_events(demo_map.clone(), events));
  let settings = load_settings();

  cleanup_renamed_vdms(
    demo_map,
    scan_for_vdms(load_settings()),
    settings["tf_folder"].as_str().unwrap()
  );
}

fn sanitize_name(path: &Path) -> PathBuf {
  let mut result = path.to_owned();

  let file_name = path.file_name().unwrap().to_str().unwrap();
  let sanitized = sanitize(file_name);

  result.set_file_name(sanitized);

  if let Some(ext) = path.extension() {
    result.set_extension(ext);
  }

  result
}

#[command]
fn is_steam_running() -> bool {
  use sysinfo::System;

  let s = System::new_all();

  let mut process_found = false;

  for _process in s.processes_by_name("steam".as_ref()) {
    process_found = true;
    break;
  }

  process_found
}

#[command]
fn launch_tf2(demo_name: &str, tab: &str) -> Value {
  tf2::run_tf2(demo_name, &load_settings(), tab)
}

#[command]
fn get_next_demo() -> Value {
  tf2::get_next_demo(&load_settings())
}

#[command]
fn is_tf2_running() -> Value {
  serde_json::Value::Bool(tf2::is_tf2_running())
}

#[tauri::command]
fn load_files(folder: &str) -> Value {
  let path = std::path::Path::new(folder);
  if !path.exists() {
    return json!({});
  }

  let entries = std::fs::read_dir(path).unwrap();

  let mut videos: Vec<Value> = vec![];

  for entry in entries {
    let path = entry.unwrap().path();
    if !path.is_dir() {
      continue;
    }

    let has_layers = std::fs::read_dir(path.to_str().unwrap().to_string() + "\\take0000");

    let layers = match has_layers {
      Ok(layers) => layers,
      Err(_) => {
        continue;
      }
    };

    let mut video_layers = json!({});

    for layer in layers {
      let path = layer.unwrap().path();
      if path.is_dir() {
        continue;
      }

      let file_name = path.file_name().unwrap().to_str().unwrap();

      if !matches!(file_name.rsplit('.').next(), Some("mp4" | "avi" | "mkv")) {
        continue;
      }

      video_layers[file_name.replace(".mp4", "").replace(".avi", "").replace(".mkv", "")] = json!(
        path.to_str().unwrap().to_string()
      );
    }

    let video: Value =
      json!({
            "name": path.file_name().unwrap().to_str().unwrap(),
            "path": path.to_str().unwrap(),
            "layers": video_layers
        });

    videos.push(video);
  }

  json!(videos)
}

#[tauri::command]
fn open_file(path: &str) {
  opener::open(path).unwrap();
}

#[tauri::command]
fn delete_file(path: &str) {
  trash::delete(path).unwrap();
}

#[tauri::command]
fn build_install(folder_name: &str) -> Value {
  tf2::build_new_install(folder_name, &load_settings())
}

#[tauri::command]
fn get_weapons() -> Value {
  weapons::get_weapons_as_json()
}

#[tauri::command]
fn before_batch() -> Value {
  batch_automation::before_batch(&load_settings())
}

#[tauri::command]
fn after_batch() -> Value {
  batch_automation::after_batch(&load_settings())
}

#[tauri::command]
fn get_rgl_users(steam_ids: Vec<String>) -> Value {
  rgl::get_users(steam_ids)
}

fn main() {
  let open_ui = cli::run_cli();

  if !open_ui {
    return;
  }

  tauri::Builder
    ::default()
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
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
        parse_demo,
        delete_demo,
        delete_vdm,
        create_vdm,
        load_theme,
        open_themes_folder,
        open_addons_folder,
        open_install_folder,
        rename_file,
        cleanup_rename,
        is_steam_running,
        launch_tf2,
        get_next_demo,
        is_tf2_running,
        before_batch,
        after_batch,
        load_files,
        open_file,
        delete_file,
        build_install,
        get_weapons,
        get_rgl_users
      ]
    )
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
