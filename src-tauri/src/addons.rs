use std::{env, fs, process::Command};

use serde_json::Value;
use crate::{macros::*, setting_as_bin};

fn compile_addon_settings(v: &Value, depth: usize) -> String {
  let mut cfg = "".to_string();

  let v_map = match v.as_object() {
      Some(v_map) => v_map,
      None => {
          return cfg;
      }
  };

  let tab_depth = "\t".repeat(depth);

  let mut vec = v_map.iter().collect::<Vec<(&String, &Value)>>();

  vec.sort_by(|a, b| (a.1["type"] == "group").cmp(&(b.1["type"] == "group")));

  for (ki, vi) in vec {
      if vi["ignoreIfDefault"] == true && vi["value"] == vi["default"] {
          continue;
      }

      match &vi["type"] {
          Value::String(vi_type) =>
              match vi_type.as_str() {
                  "group" => {
                      let addon_text = compile_addon_settings(&vi["settings"], depth + 1);

                      if addon_text.is_empty() {
                          continue;
                      }

                      extend!(cfg, "\r\n{}\t", tab_depth);
                      extend!(cfg, "\\\\ {}\r\n", ki.as_str());
                      extend!(cfg, "{}", compile_addon_settings(&vi["settings"], depth + 1));
                  }
                  "toggle" => {
                      if vi["value"] == true {
                          extend!(cfg, "{};\r\n", tab_depth.clone() + vi["command"].as_str().unwrap());
                      }
                  }
                  "bool" => {
                      extend!(
                          cfg,
                          "{};\r\n",
                          format!(
                              "{}{} {}",
                              tab_depth.clone(),
                              vi["command"].as_str().unwrap(),
                              setting_as_bin(&vi["value"])
                          )
                      );
                  }
                  "string" | "int" => {
                      if vi["ignoreIfBlank"] == true && vi["value"] == "" {
                          continue;
                      }

                      extend!(
                          cfg,
                          "{};\r\n",
                          format!(
                              "{}{} {}",
                              tab_depth.clone(),
                              vi["command"].as_str().unwrap(),
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

  return cfg;
}

pub fn compile_addons(settings: &Value) -> String {
  let addons = settings["addons"].as_object();

  let mut cfg = "".to_string();

  if addons.is_none() {
      return cfg;
  }

  let addons = addons.unwrap();

  let iter = addons.into_iter();

  let mut vec = iter.collect::<Vec<(&String, &Value)>>();

  vec.sort_by(|a, b| (a.1["type"] == "group").cmp(&(b.1["type"] == "group")));
  
  for (k, v) in addons {
      let addon_text = compile_addon_settings(v, 0);
      
      if addon_text.is_empty() {
          continue;
      }

      extend!(cfg, "\r\necho \"Running {} addon\";\r\n", k);

      extend!(cfg, "{}\r\n", addon_text);
  }

  return cfg;
}


pub fn open_addons_folder() {
  let user_profile = env::var("USERPROFILE");

  let addons_path = match user_profile {
      Ok(profile) => { format!("{}\\Documents\\Melies\\addons", profile) }
      Err(_) => {
          format!(
              "{}\\addons",
              std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap()
          )
      }
  };

  fs::create_dir_all(&addons_path).unwrap();

  Command::new("explorer")
      .arg(addons_path)
      .spawn()
      .unwrap();
}