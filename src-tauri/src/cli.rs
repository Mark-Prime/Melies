use std::{ fs, path::Path };

use serde_json::Value;

use crate::{ demos::scan_demo, event::{ Event, EventStyle }, events };

pub fn run_cli() -> bool {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    return true;
  }

  let pattern = args[1].clone();

  let res = match pattern.as_str() {
    "--run" | "-R" => run(),
    "--events" | "-E" => grab_events(args),
    "--launchTf2" | "-L" | "--launchtf2" => launch_tf2(args),
    "--open" | "-O" => open_demo(args),
    "--help" | "-H" => {
      println!("Key:");
      println!("  <> required argument");
      println!("  [] optional argument");
      println!("");
      println!("Commands:");
      println!("  -R, --run");
      println!("  -E, --events <demo path> [player]");
      println!("  -L, --launchTf2 [preset] [demo name]");
      println!("  -O, --open <demo path> [output path]");
      return false;
    }
    _ => {
      println!("Unknown command");
      return true;
    }
  };

  res
}

fn run() -> bool {
  println!("Running");

  crate::ryukbot::run_ryukbot();

  println!("Done");

  false
}

fn open_demo(args: Vec<String>) -> bool {
  println!("Opening demo");

  if args.len() < 3 {
    println!("Missing demo path");
    return false;
  }

  let demo_path = args[2].clone();
  let settings = crate::settings::load_settings();

  let res = scan_demo(settings, demo_path);

  if args.len() < 4 {
    return false;
  }

  let file = fs::File::create(args[3].clone()).unwrap();

  let writer = std::io::BufWriter::new(file);
  serde_json::to_writer_pretty(writer, &res).unwrap();

  false
}

fn grab_events(args: Vec<String>) -> bool {
  if args.len() < 3 {
    println!("Missing demo path");
    return false;
  }

  let settings = crate::settings::load_settings();
  let automation_settings = &settings["automation"];

  if !automation_settings["enabled"].as_bool().unwrap() {
    println!("Automation is disabled.");
    println!("Are you sure you want to continue? (y/n)");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() != "y" {
      return false;
    }
  }

  println!("Scanning demo");

  let demo = args[2].clone();
  let binding = demo.clone();
  let demo_path = Path::new(&binding);

  let res = scan_demo(crate::settings::load_settings(), demo);

  let mut player_id = String::new();

  if args.len() > 3 && !args[3].is_empty() {
    player_id = get_player_id(args[3].clone(), &res);
  }

  let player_lives = res["data"]["player_lives"].as_object().unwrap();

  let mut events: Vec<Event> = vec![];

  for (key, value) in player_lives {
    if !player_id.is_empty() && key != &player_id {
      continue;
    }

    for (_i, life) in value.as_array().unwrap().iter().enumerate() {
      let mut used_kills: Vec<i64> = vec![];
      let demo_name = demo_path.file_name().unwrap().to_string_lossy().to_string().replace(".dem", "");

      if
        life["killstreak_pointers"].as_array().unwrap().len() > 0 &&
        automation_settings["killstreaks"].as_bool().unwrap()
      {
        for killstreak_pointer in life["killstreak_pointers"].as_array().unwrap() {
          let kills = killstreak_pointer["kills"].as_array().unwrap();

          for kill in kills.iter() {
            if used_kills.contains(&kill.as_i64().unwrap()) {
              continue;
            }

            used_kills.push(kill.as_i64().unwrap());

            let kill = life["kills"]
              .as_array()
              .unwrap()
              .get(kill.as_i64().unwrap() as usize)
              .unwrap();

            events.push(bookmark_kill(kill.clone(), demo_name.clone(), &res["data"]["users"]));
          }
        }
      }

      if
        life["airshots"].as_array().unwrap().len() > 0 &&
        automation_settings["airshots"].as_bool().unwrap()
      {
        for airshot in life["airshots"].as_array().unwrap() {
          if used_kills.contains(&airshot["kill_index"].as_i64().unwrap()) {
            continue;
          }

          let kill = life["kills"]
            .as_array()
            .unwrap()
            .get(airshot["kill_index"].as_i64().unwrap() as usize)
            .unwrap();

          if !is_airshot(&kill) {
            continue;
          }

          used_kills.push(airshot["kill_index"].as_i64().unwrap());

          events.push(bookmark_kill(kill.clone(), demo_name.clone(), &res["data"]["users"]));
        }
      }

      if
        life["med_picks"].as_array().unwrap().len() > 0 &&
        automation_settings["med_picks"].as_bool().unwrap()
      {
        for med_pick in life["med_picks"].as_array().unwrap() {
          if used_kills.contains(&med_pick["kill_index"].as_i64().unwrap()) {
            continue;
          }

          used_kills.push(med_pick["kill_index"].as_i64().unwrap());

          let kill = life["kills"]
            .as_array()
            .unwrap()
            .get(med_pick["kill_index"].as_i64().unwrap() as usize)
            .unwrap();

          events.push(bookmark_kill(kill.clone(), demo_name.clone(), &res["data"]["users"]));
        }
      }
    }
  }

  events.sort_by(|a, b| a.tick.cmp(&b.tick));

  events::directly_save_events(events);

  false
}

fn get_player_id(arg: String, res: &serde_json::Value) -> String {
  let users = res["data"]["users"].as_object().unwrap();
  let user_ids = res["data"]["users"].as_object().unwrap().keys().collect::<Vec<&String>>();

  for user in user_ids.iter() {
    if
      users[user.to_owned()]["name"].as_str().unwrap().to_lowercase() == arg.as_str().to_lowercase()
    {
      return user.to_string();
    }

    if users[user.to_owned()]["steamId"].as_str().unwrap() == arg.as_str() {
      return user.to_string();
    }

    if users[user.to_owned()]["steamId64"].as_str().unwrap() == arg.as_str() {
      return user.to_string();
    }
  }

  String::new()
}

fn launch_tf2(args: Vec<String>) -> bool {
  let mut tab = String::from("0");
  let mut demo_name = String::new();

  if args.len() > 2 && !args[2].is_empty() {
    if args[2].chars().all(char::is_numeric) {
      tab = args[2].clone();
    } else {
      tab = name_to_tab(args[2].clone());
    }
  }

  if args.len() > 3 && !args[3].is_empty() {
    demo_name = args[3].clone();
  }

  let settings = crate::settings::load_settings();

  crate::tf2::run_tf2(&demo_name, &settings, &tab);

  false
}

fn name_to_tab(name: String) -> String {
  let settings = crate::settings::load_settings();

  for (i, install) in settings["alt_installs"].as_array().unwrap().iter().enumerate() {
    if install["name"].as_str().unwrap() == name.as_str() {
      return (i + 1).to_string();
    }
  }

  return String::from("0");
}

fn bookmark_kill(kill: Value, demo_name: String, users: &Value) -> Event {
  let mut labels = vec![];

  if is_airshot(&kill) {
    labels.push("Airshot".to_string());
  }

  if is_med_pick(&kill) {
    labels.push("MedPick".to_string());
  }

  let killer_id = kill["killer"].as_i64().unwrap();

  labels.push(
    format!("spec {}", users[&killer_id.to_string()]["steamId64"].as_str().unwrap())
  );

  let event = format!("[CLI] Bookmark {} (\"{}\" at {})", labels.join(" "), demo_name, kill["tick"].as_i64().unwrap());

  Event {
    event: event,
    demo_name: demo_name,
    tick: kill["tick"].as_i64().unwrap(),
    value: EventStyle::Bookmark(labels.join(" ")),
  }
}

fn is_med_pick(kill: &Value) -> bool {
  kill["victim_class"].as_str().unwrap() == "medic"
}

fn is_airshot(kill: &Value) -> bool {
  if !kill["is_airborne"].as_bool().unwrap() { 
    return false; 
  }
  
  let settings = crate::settings::load_settings();

  let airshot_settings = &settings["advanced"]["airshots"];

  let default = airshot_settings["default"].as_bool().unwrap();

  let killer_class = kill["killer_class"].as_str();
  let victim_class = kill["victim_class"].as_str();

  if killer_class.is_none() || victim_class.is_none() {
    return default;
  }

  let killer_settings = airshot_settings["killer"].clone();
  let victim_settings = airshot_settings["victim"].clone();

  let killer = match killer_settings[killer_class.unwrap()].as_str().unwrap() {
    "Never" => false,
    "CriticalHit" => kill["crit_type"].as_i64().unwrap() == 2,
    "MiniCriticalHit" => kill["crit_type"].as_i64().unwrap() == 1,
    "AnyCritHit" => kill["crit_type"].as_i64().unwrap() != 0,
    "Always" => true,
    _ => default,
  };

  let victim = match victim_settings[victim_class.unwrap()].as_str().unwrap() {
    "Never" => false,
    "CriticalHit" => kill["crit_type"].as_i64().unwrap() == 2,
    "MiniCriticalHit" => kill["crit_type"].as_i64().unwrap() == 1,
    "AnyCritHit" => kill["crit_type"].as_i64().unwrap() != 0,
    "Always" => true,
    _ => default,
  };

  killer && victim
}