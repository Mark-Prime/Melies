use std::process::Command;
use serde_json::{self, json, Value};

pub fn before_batch(settings: &Value) -> Value {
  match settings["hlae"]["before_batch"].as_str().unwrap() {
    "nothing" => return json!({}),
    "open" => {
      let _ = Command::new("explorer").arg(settings["output"]["folder"].as_str().unwrap()).spawn().unwrap();
    },
    "run" => {
      let _ = Command::new(settings["hlae"]["before_batch_path"].as_str().unwrap()).spawn().unwrap();
    },
    _ => return json!({}),
  }
  json!({})
} 

pub fn after_batch(settings: &Value) -> Value {
  match settings["hlae"]["after_batch"].as_str().unwrap() {
    "nothing" => return json!({}),
    "open" => {
      let _ = Command::new("explorer").arg(settings["output"]["folder"].as_str().unwrap()).spawn().unwrap();
    },
    "shutdown" => {
      let _ = Command::new("shutdown /s /t 0").spawn();
    },
    "run" => {
      let _ = Command::new(settings["hlae"]["after_batch_path"].as_str().unwrap()).spawn().unwrap();
    },
    _ => return json!({}),
  }
  json!({})
}