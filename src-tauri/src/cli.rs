use std::{ fs, path::Path, vec };

use serde_json::{Map, Value};

use crate::{ demos::scan_demo, event::{ Event, EventStyle }, events };

#[derive(Debug)]
enum Commands {
  Help,
  Error(String),
  Run {
    options: Map<String, Value>
  },
  LaunchTf2 {
    options: Map<String, Value>
    // --preset
    // --demo
  },
  GrabEvents {
    demo: String,
    options: Map<String, Value>
    // --player
  },
  OpenDemo {
    demo: String,
    options: Map<String, Value>
    // --output
  },
}

impl Commands {
  fn new(mut args: Vec<String>) -> Self {

    args.remove(0);

    let options = Self::get_options(args.clone());

    match args[0].as_str().to_lowercase().as_str() {
      "help" | "--help"  => Commands::Help,
      "run" => Commands::Run {
        options
      },
      "launchtf2" => Commands::LaunchTf2 {
        options
      },
      "events" => {
        if args.len() < 2 {
          return Commands::Error("Missing <demo path>".to_string());
        }
        Commands::GrabEvents {
          demo: args[1].clone(),
          options
        }
      },
      "open" => {
        if args.len() < 2 {
          return Commands::Error("Missing <demo path>".to_string());
        }

        Commands::OpenDemo {
          demo: args[1].clone(),
          options
        }
      },
      _ => Commands::Error(format!("Unknown command: {}", args[1])),
    }
  }

  fn get_options(args: Vec<String>) -> Map<String, Value> {
    let mut options = Map::new();

    for (i, mut arg) in args.iter().skip(1).enumerate() {
      if !arg.contains("--") {
        continue; 
      }

      let binding = arg.clone().replace("--", "");
      arg = &binding;

      if args.len() > i + 2 && !args[i + 2].is_empty() {
        if args[i + 2].contains("--") {
          options.insert(arg.clone(), Value::Bool(true));
          continue;
        }

        options.insert(arg.clone(), Value::String(args[i + 2].clone()));
      } else {
        options.insert(arg.clone(), Value::Bool(true));
      }
    }

    options
  }

  fn options(&self) -> Map<String, Value> {
    match self {
      Commands::Run { options } => options.to_owned(),
      Commands::LaunchTf2 { options } => options.to_owned(),
      Commands::GrabEvents { options, .. } => options.to_owned(),
      Commands::OpenDemo { options, .. } => options.to_owned(),
      _ => Map::new(),
    }
  }
  fn run(&self) -> Option<Value> {
    match self {
      Commands::Run { options } => Some(run(options.clone())),
      Commands::LaunchTf2 { options } => Some(launch_tf2(options.clone())),
      Commands::GrabEvents { demo, options } => Some(grab_events(demo.to_string(), options.clone())),
      Commands::OpenDemo { demo, options } => Some(open_demo(demo.to_string(), options.clone())),
      Commands::Help => {
        self.print_help();
        return None;
      }
      Commands::Error(msg) => {
        println!("{}", msg);
        return None;
      }
    }
  }

  fn print_help(&self) {
    let mut function = String::from("");
    let mut commands = vec!["help", "run", "launchtf2", "events <demo path>", "open <demo path>"];
    let mut args: Vec<&str> = vec![];
    let mut optional_args: Vec<&str> = vec![];

    match self {
      Commands::Run { options: _ } => {
        function = "Runs Méliès and builds the VDMs.".to_string();
        commands = vec![];
        args = vec![];
        optional_args = vec![];
      },
      Commands::LaunchTf2 { options: _ } => {
        function = "Launches Team Fortress 2.".to_string();
        commands = vec![];
        args = vec![];
        optional_args = vec!["--preset <preset>", "--demo <demo name>"];
      },
      Commands::GrabEvents { demo: _, options: _ } => {
        function = "Scrapes events from a demo using the demo parser\n\t\t  and exports them to the _events.txt file.".to_string();
        commands = vec![];
        args = vec!["<demo path>"];
        optional_args = vec!["--player <player name or steamid64>"];
      },
      Commands::OpenDemo { demo: _, options: _ } => {
        function = "Scrapes all demo information and outputs it to\n\t\t  the console or a specified output path.".to_string();
        commands = vec![];
        args = vec!["<demo path>"];
        optional_args = vec!["--output <output path>"];
      },
      _ => {},
    };

    if function.len() > 0 {
      println!("Function: {}", function);
      println!("");
    }
    
    if commands.len() > 0 {
      println!("Commands:");
      for command in commands {
        println!("  {}", command);
      }
      println!("");
    }

    if args.len() > 0 {
      println!("Arguments:");
      for arg in args {
        println!("  {}", arg);
      }
    }

    if optional_args.len() == 0 {
      return;
    }

    println!("Optional Arguments:");
    for arg in optional_args {
      println!("  {}", arg);
    }
  }
}

impl std::fmt::Display for Commands {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Commands::Run { .. } => write!(f, "run"),
      Commands::LaunchTf2 { .. } => write!(f, "launchtf2"),
      Commands::GrabEvents { .. } => write!(f, "events"),
      Commands::OpenDemo { .. } => write!(f, "open"),
      _ => write!(f, "help"),
    }
  }
}

fn print_verbose(options: &Map<String, Value>, msg: &str) {
  let verbose = options.get("verbose").is_some();

  if verbose {
    println!("{}", msg);
  }
}

fn run(options: Map<String, Value>) -> Value {
  print_verbose(&options, "Running");

  let res = crate::ryukbot::run_ryukbot();

  if res.get("err_text").is_some() {
    print_verbose(&options, format!("\nError code: {}\n", res["code"]).as_str());
    print_verbose(&options, format!("{}", res["err_text"].as_str().unwrap()).as_str());
    return res;
  }

  print_verbose(&options, "Success");

  print_verbose(&options, "\nStats:");
  print_verbose(&options, format!("  Events: {}", res["events"]).as_str());
  print_verbose(&options, format!("  Clips: {}", res["clips"]).as_str());
  print_verbose(&options, format!("  VDMs: {}\n", res["vdms"]).as_str());

  print_verbose(&options, "First Demo:");

  println!("{}", res["first_demo"].as_str().unwrap());

  return res;
}

fn open_demo(demo_path: String, options: Map<String, Value>) -> Value {
  let settings = crate::settings::load_settings();

  let res = scan_demo(settings, demo_path);

  if options.get("output").is_none() {
    return Value::from(res);
  }

  let file = fs::File::create(options["output"].as_str().unwrap()).unwrap();

  let writer = std::io::BufWriter::new(file);
  serde_json::to_writer_pretty(writer, &res).unwrap();

  Value::from(res)
}

fn grab_events(demo: String, options: Map<String, Value>) -> Value {
  let settings = crate::settings::load_settings();
  let automation_settings = &settings["automation"];

  if !automation_settings["enabled"].as_bool().unwrap() {
    println!("Automation is disabled.");
    println!("Are you sure you want to continue? (y/n)");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() != "y" {
      return Value::Null;
    }
  }

  print_verbose(&options, "Scanning demo");
  let binding = demo.clone();
  let demo_path = Path::new(&binding);

  let res = scan_demo(crate::settings::load_settings(), demo);

  let mut player_id = String::new();

  if options.get("player").is_some() {
    player_id = get_player_id(options["player"].as_str().unwrap().to_string(), &res);
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

  events::directly_save_events(events)
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

fn launch_tf2(options: Map<String, Value>) -> Value {
  let mut tab = String::from("0");

  if options.get("preset").is_some() {
    if options["preset"].as_str().unwrap().chars().all(char::is_numeric) {
      tab = options["preset"].as_str().unwrap().to_string();
    } else {
      tab = name_to_tab(options["preset"].as_str().unwrap().to_string());
    }
  }

  let mut demo_name = String::new();

  if options.contains_key("demo") && !options["demo"].as_str().unwrap().is_empty() {
    demo_name = options["demo"].as_str().unwrap().to_string();
  }

  let settings = crate::settings::load_settings();

  crate::tf2::run_tf2(&demo_name, &settings, &tab)
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

pub fn run_cli() -> bool {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    return true;
  }

  let command = Commands::new(args);

  if command.options().get("help").is_some() {
    command.print_help();
    return false;
  }

  command.run();

  false
}