use std::fs;

use chrono::{DateTime, Local};
use regex::Regex;
use serde_json::{Value, json};

use crate::{event::{Event, EventStyle::{ Bookmark, Killstreak }}, macros::extend, settings::load_settings, util::find_dir};

pub fn save_events(new_events: Value) -> Value {
  let new_events = new_events.as_array().unwrap();
  let mut events: Vec<Event> = vec![];
  let mut new_events_text = String::new();

  println!("{:#?}", new_events);

  for demo in new_events {
    println!("demo: {:#?}", demo);
    extend!(new_events_text, "{}\r\n", ">");

    for event in demo.as_array().unwrap() {
      let re = Regex::new("\\[(.*)\\] (.*) \\(\"(.*)\" at (\\d*)\\)").unwrap();

      if event["event"].as_str().is_none() {
        continue;
      }

      println!("event: {:#?}", event);

      let events_regex = match re.captures(event["event"].as_str().unwrap()) {
        Some(val) => val,
        None => {
          println!("Failed to parse event: {}", event["event"].as_str().unwrap());
          continue;
        }
      };

      let original_event = Event::new(events_regex).unwrap();

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

  println!("{:#?}", new_events_text);

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