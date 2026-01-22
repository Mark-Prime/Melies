#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::{ self, Value };
use std::{ env };
use tauri::command;
use trash;

use melies_rust;

#[command]
fn ryukbot() -> Value {
  melies_rust::ryukbot::run_ryukbot()
}

#[command]
fn load_settings() -> Value {
  melies_rust::settings::load_settings()
}

#[command]
fn save_settings(new_settings: String) -> Value {
  melies_rust::settings::save_settings(new_settings)
}

#[command]
fn load_events() -> Value {
  melies_rust::load_events()
}

#[command]
fn save_events(new_events: Value) -> Value {
  melies_rust::events::save_events(new_events)
}

#[command]
fn parse_log(url: Value) -> Value {
  melies_rust::logstf::parse(url)
}

#[command]
fn load_vdms() -> Result<Value, String> {
  melies_rust::load_vdms()
}

#[command]
fn load_demos() -> Result<Value, String> {
  melies_rust::load_demos()
}

#[command]
fn load_backups() -> Result<Value, String> {
  melies_rust::load_backups()
}

#[command]
fn reload_backup(file_name: Value) -> Result<Value, String> {
  melies_rust::reload_backup(file_name)
}

#[command]
fn parse_demo(path: String) -> Value {
  melies_rust::demos::scan_demo(load_settings(), path)
}

#[command]
fn load_vdm(name: Value) -> Value {
  melies_rust::load_vdm(name)
}

#[command]
fn save_vdm(name: Value, vdm: Value) -> Value {
  melies_rust::save_vdm(name, vdm)
}

#[command]
fn open_addons_folder() {
  melies_rust::addons::open_addons_folder();
}

#[command]
fn delete_demo(file_name: Value) {
  melies_rust::delete_demo(file_name);
}

#[command]
fn delete_vdm(file_name: Value) {
  melies_rust::delete_vdm(file_name);
}

#[command]
fn create_vdm(file_name: Value) {
  melies_rust::create_vdm(file_name);
}

#[command]
fn load_theme() -> Value {
  melies_rust::load_theme()
}

#[command]
fn open_themes_folder() {
  melies_rust::open_themes_folder();
}

#[command]
fn open_install_folder(install: &str) {
  melies_rust::open_install_folder(install);
}

#[command]
fn rename_file(old_path: &str, new_path: &str) {
  melies_rust::rename_file(old_path, new_path);
}

#[command]
fn cleanup_rename(demo_map: Value) {
  melies_rust::cleanup_rename(demo_map);
}

#[command]
fn is_steam_running() -> bool {
  melies_rust::is_steam_running()
}

#[command]
fn launch_tf2(demo_name: &str, tab: &str) -> Value {
  melies_rust::tf2::run_tf2(demo_name, &load_settings(), tab)
}

#[command]
fn get_next_demo(demo_name: &str) -> Value {
  melies_rust::tf2::get_next_demo(&load_settings(), demo_name)
}

#[command]
fn is_tf2_running() -> Value {
  serde_json::Value::Bool(melies_rust::tf2::is_tf2_running())
}

#[command]
fn load_files(folder: &str) -> Value {
  melies_rust::load_files(folder)
}

#[command]
fn open_file(path: &str) {
  opener::open(path).unwrap();
}

#[command]
fn delete_file(path: &str) {
  trash::delete(path).unwrap();
}

#[command]
fn build_install(folder_name: &str) -> Value {
  melies_rust::tf2::build_new_install(folder_name, &load_settings())
}

#[command]
fn get_weapons() -> Value {
  melies_rust::weapons::get_weapons_as_json()
}

#[command]
fn before_batch() -> Value {
  melies_rust::batch_automation::before_batch(&load_settings())
}

#[command]
fn after_batch() -> Value {
  melies_rust::batch_automation::after_batch(&load_settings())
}

#[command]
fn get_rgl_users(steam_ids: Vec<String>) -> Value {
  melies_rust::rgl::get_users(steam_ids)
}

fn main() {
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
