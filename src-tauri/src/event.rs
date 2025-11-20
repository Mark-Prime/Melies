use regex::Captures;
use serde::{ Deserialize, Serialize };
use std::fmt::{ self, Display, Formatter };

#[derive(Debug)]
pub enum EventError {
  InvalidTick,
}

impl Display for EventError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    use EventError::*;
    match self {
      InvalidTick => write!(f, "The tick is not a number."),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EventStyle {
  Bookmark(String),
  Killstreak(i64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
  pub event: String,
  pub demo_name: String,
  pub tick: i64,
  pub value: EventStyle,
  pub notes: String
}

impl Event {
  pub fn new(input: Captures) -> Result<Event, EventError> {
    let tick;

    match input[4].to_string().parse::<i64>() {
      Ok(num) => {
        tick = num;
      }
      Err(_) => {
        return Err(EventError::InvalidTick);
      }
    }

    let split = input[2].to_string();

    let mut event_name: Vec<&str> = split.split_whitespace().collect();
    let value: EventStyle;

    if event_name.len() == 0 {
      return Ok(Event {
        event: input[0].to_string(),
        demo_name: input[3].to_string(),
        tick,
        value: EventStyle::Bookmark("General".to_owned()),
        notes: input[5].to_string()
      });
    }

    let name = event_name[0].to_lowercase();

    match name.as_str() {
      "kill" => 'kill: {
        let streak = event_name[1].to_string();
        let streak_split: Vec<&str> = streak.split(":").collect();
        let parse = streak_split[1].to_string().parse::<i64>();

        if parse.is_err() {
          println!("Killstreak failed to parse: {:#?}", event_name);
          value = EventStyle::Bookmark(event_name.join(" "));
          break 'kill;
        }

        value = EventStyle::Killstreak(parse.unwrap());
      }
      "killstreak" => 'killstreak: {
        let parse = event_name[1].to_string().parse::<i64>();

        if parse.is_err() {
          println!("Killstreak failed to parse: {:#?}", event_name);
          value = EventStyle::Bookmark(event_name.join(" "));
          break 'killstreak;
        }

        value = EventStyle::Killstreak(parse.unwrap());
      }
      "bookmark" => {
        event_name.remove(0);
        value = EventStyle::Bookmark(event_name.join(" "));
      }
      "player" => {
        value = EventStyle::Bookmark("General".to_owned());
      }
      _ => {
        value = EventStyle::Bookmark(event_name.join(" "));
      }
    }

    Ok(Event {
      event: input[0].to_string(),
      demo_name: input[3].to_string(),
      tick,
      value,
      notes: input[5].to_string()
    })
  }

  pub fn commands(&self) -> Vec<&str> {
    if let EventStyle::Bookmark(val) = &self.value {
      return val.split_whitespace().collect();
    }

    vec![]
  }

  pub fn contains(&self, name: &str) -> bool {
    if let EventStyle::Bookmark(val) = &self.value {
      return val.contains(name);
    }

    false
  }
}

impl Display for Event {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.event)
  }
}
