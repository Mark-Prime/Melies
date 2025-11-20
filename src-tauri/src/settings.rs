use serde::Serialize;
use serde_json::{ json, Map, Value };
use std::{ env, fs::{ self, File }, path::Path };

#[derive(Debug, Serialize)]
pub enum RecordToggle {
  // * 0 = off, 1 = on critical hit, 2 = weapon dependant, 3 = ranged only,  4 = melee only, 5 = always
  Never,
  CriticalHit,
  // MiniCriticalHit,
  AnyCritHit,
  // WeaponDependant,
  // RangedOnly,
  // MeleeOnly,
  Always,
  Passive,
}

pub(crate) fn build_settings() -> Value {
  let user_profile = env::var("USERPROFILE");

  let binding = match user_profile {
    Ok(profile) => { format!("{}\\Documents\\Melies\\settings.json", profile) }
    Err(_) => {
      format!(
        "{}\\settings.json",
        std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap()
      )
    }
  };

  let settings_path = Path::new(&binding);
  let settings_prefix = settings_path.parent().unwrap();
  std::fs::create_dir_all(settings_prefix).unwrap();

  File::create(settings_path).unwrap();

  let mut settings = default_settings();

  fs::write(settings_path, serde_json::to_string_pretty(&settings.to_string()).unwrap()).unwrap();

  settings["addons"] = load_addons();

  settings
}

pub(crate) fn default_settings() -> Value {
  let defaults =
    json!({
      "tf_folder": "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Team Fortress 2\\tf",
      "alt_installs": [],
      "clear_events": true,
      "save_backups": true,
      "safe_mode": true,
      "absolute_file_paths": true,
      "pov_as_stv": false,
      "addons": {},
      "sort_footage" : {
        "skip_buffer": false,
        "skip_to": 13
      },
      "output": {
        "folder": "",
        "method": "tga",
        "framerate": 60,
        "crosshair": false,
        "viewmodel": true,
        "hud": true,
        "text_chat": false,
        "voice_chat": false,
        "minmode": false,
        "snd_fix": true,
        "lock": true,
        "clip_name_template": "{demo_name}_{start_tick}-{end_tick}_{suffix}_{bookmarks}"
      },
      "recording": {
        "commands": "",
        "end_commands": "",
        "start_delay": 50,
        "minimum_ticks_between_clips": 500,
        "before_bookmark": 1000,
        "after_bookmark": 200,
        "minimum_kills_in_streak": 3,
        "before_killstreak_per_kill": 500,
        "after_killstreak": 300,
        "interval_for_rewind_double_taps": 66,
        "rewind_amount": 1000,
        "fov": 90,
        "viewmodel_fov": 90,
        "record_continuous": true,
        "auto_close": true,
        "auto_suffix": true,
        "third_person": false,
        "use_ce_spec": false
      },
      "automation": {
        "enabled": true,
        "med_picks": true,
        "airshots": true,
        "killstreaks": true,
        "whole_life": false,
        "classes": {
            "scout": true,
            "soldier": true,
            "pyro": true,
            "demoman": true,
            "heavyweapons": true,
            "engineer": true,
            "medic": true,
            "sniper": true,
            "spy": true,
        }
      },
      "demo_manager": {
        "default_name": "{date}_{time}_{map}_{ticks}",
        "confirm_delete": true,
        "auto_update": true
      },
      "hlae": {
        "sparklyfx_path": "",
        "hlae_path": "C:\\Program Files (x86)\\HLAE\\HLAE.exe",
        "launch_options": "-nojoy -nosteamcontroller -nohltv -precachefontchars",
        "dxlevel": 100,
        "height": 1080,
        "width": 1920,
        "use_64bit": true,
        "playdemo": true,
        "novid": true,
        "borderless": true,
        "before_batch": "nothing",
        "before_batch_path": "",
        "after_batch": "nothing",
        "after_batch_path": ""
      },
      "features": {
        "basic": {
            "editEvents": true,
            "backups": true,
            "scanDemos": true,
        },
        "advanced": {
            "vdmeditor": true,
            "demoManager": true,
            "sortFootage": true,
        },
        "demo_scanner": {
            "logstf": false,
            "rgl": false,
            "med_picks": true,
            "airshots": true,
            "killstreaks": true,
            "breakdowns": true,
            "chat": true,
            "timeline": true
        },
        "deprecated": {
            "logstf": false
        }
      },
      "advanced": {
        "airshots": {
          "default": true,
          "killer": {
            "scout": RecordToggle::Never,
            "soldier": RecordToggle::Always,
            "pyro": RecordToggle::AnyCritHit,
            "demoman": RecordToggle::Always,
            "heavy": RecordToggle::Never,
            "engineer": RecordToggle::Never,
            "medic": RecordToggle::Always,
            "sniper": RecordToggle::CriticalHit,
            "spy": RecordToggle::CriticalHit,
          },
          "victim": {
            "scout": RecordToggle::Passive,
            "soldier": RecordToggle::Passive,
            "pyro": RecordToggle::Passive,
            "demoman": RecordToggle::Passive,
            "heavy": RecordToggle::Passive,
            "engineer": RecordToggle::Passive,
            "medic": RecordToggle::Passive,
            "sniper": RecordToggle::Passive,
            "spy": RecordToggle::Passive,
          }
        }
      }
    });

  defaults
}

pub(crate) fn load_settings() -> Value {
  let user_profile = env::var("USERPROFILE");

  let settings_path = match user_profile {
    Ok(profile) => { format!("{}\\Documents\\Melies\\settings.json", profile) }
    Err(_) => {
      format!(
        "{}\\settings.json",
        std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap()
      )
    }
  };

  if Path::new(&settings_path).exists() {
    let file = fs::read_to_string(settings_path).unwrap();
    let settings: Value = serde_json::from_str(&file).unwrap();

    let mut defaults = default_settings();

    merge(&mut defaults, settings);

    // I have no idea why this needs to happen but it prevents a crash
    if defaults.is_string() {
      defaults = serde_json::from_str(&defaults.as_str().unwrap()).unwrap();
    }

    defaults["addons"] = load_addons();

    return defaults;
  }

  build_settings()
}

pub(crate) fn save_settings(new_settings: String) -> Value {
  let settings: Value = serde_json::from_str(&new_settings).unwrap();

  let user_profile = env::var("USERPROFILE");

  let settings_path = match user_profile {
    Ok(profile) => { format!("{}\\Documents\\Melies\\settings.json", profile) }
    Err(_) => {
      format!(
        "{}\\settings.json",
        std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap()
      )
    }
  };

  if Path::new(&settings_path).exists() {
    let mut defaults = default_settings();

    save_addons(&settings["addons"]);

    merge(&mut defaults, settings);

    fs::write(settings_path, serde_json::to_string_pretty(&defaults).unwrap()).unwrap();

    return defaults;
  }

  build_settings()
}

pub(crate) fn load_addons() -> Value {
  let user_profile = env::var("USERPROFILE");

  let addons_path = match user_profile {
    Ok(profile) => { format!("{}\\Documents\\Melies\\addons", profile) }
    Err(_) => {
      format!("{}\\addons", std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap())
    }
  };

  fs::create_dir_all(&addons_path).unwrap();

  let mut addons = json!({});

  let files = fs::read_dir(addons_path).unwrap();

  for file in files {
    let filename_os = file.as_ref().unwrap().file_name();
    let filename = filename_os.to_str().unwrap().to_string();

    if !filename.contains(".json") && !filename.contains(".JSON") {
      continue;
    }

    let mut name = filename.replace(".json", "");
    name = name.replace(".JSON", "");

    let data = fs::read_to_string(file.unwrap().path()).expect("Unable to read file");
    let res = match serde_json::from_str(&data) {
      Ok(val) => val,
      Err(_) => {
        continue;
      }
    };

    addons[name] = res;
  }

  addons
}

fn save_addons(addons: &Value) {
  let map: &Map<String, Value> = addons.as_object().unwrap();

  for (k, v) in map {
    let user_profile = env::var("USERPROFILE");

    let addon_path = match user_profile {
      Ok(profile) => { format!("{}\\Documents\\Melies\\addons\\{}.json", profile, k) }
      Err(_) => {
        format!(
          "{}\\addons\\{}.json",
          std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap(),
          k
        )
      }
    };

    fs::write(addon_path, serde_json::to_string_pretty(v).unwrap()).unwrap();
  }
}

fn merge(a: &mut Value, b: Value) {
  match (a, b) {
    (a @ &mut Value::Object(_), Value::Object(b)) => {
      let a = a.as_object_mut().unwrap();
      for (k, v) in b {
        merge(a.entry(k).or_insert(Value::Null), v);
      }
    }
    (a, b) => {
      *a = b;
    }
  }
}

pub fn setting_as_bin(setting: &Value) -> i64 {
  if !setting.is_boolean() {
    if let Some(setting_i64) = setting.as_i64() {
      return setting_i64;
    }
  }

  match setting.as_bool() {
    Some(val) =>
      match val {
        true => 1,
        false => 0,
      }
    _ => 0,
  }
}

pub fn setting_as_bool(setting: &Value) -> bool {
  if setting.is_boolean() {
    return match setting.as_bool() {
      Some(val) => val,
      None => false,
    };
  }

  match setting.as_i64() {
    Some(val) =>
      match val {
        1 => true,
        _ => false,
      }
    _ => false,
  }
}
