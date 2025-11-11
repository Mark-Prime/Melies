use serde_json::{ json, Value };
use reqwest;

pub fn get_users(steam_ids: Vec<String>) -> Value {
  let mut users: Value = json!({});
  let resp = reqwest::blocking::Client
    ::new()
    .post("https://api.rgl.gg/v0/profile/getmany")
    .header("Content-Type", "application/json")
    .body(serde_json::to_string(&steam_ids).unwrap())
    .send();

  if resp.is_err() {
    return users;
  }

  let resp = resp.unwrap();

  // println!("RGL response: {}", resp.text().unwrap());

  users["response"] = resp.json().unwrap();

  let response = users["response"].clone();

  if response["error"].as_str().is_some() {
    return users;
  }

  for user in response.as_array().unwrap() {
    let steam_id = user["steamId"].as_str().unwrap();
    users[steam_id] = user.clone();
  }

  users
}
