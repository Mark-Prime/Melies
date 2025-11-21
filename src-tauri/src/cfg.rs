use std::{fs::File, io::Write, path::Path};

use serde_json::Value;

use crate::{addons::compile_addons, macros::extend, settings::setting_as_bin};

pub fn write_cfg(settings: &Value) -> Result<(), String> {
  let mut cfg = String::new();

  extend!(
    cfg,
    "echo \"Execing Melies Config\";\ncl_drawhud {};\n",
    setting_as_bin(&settings["output"]["hud"])
  );
  extend!(cfg, "sv_cheats {};\n", "1");
  extend!(cfg, "voice_enable {};\n", setting_as_bin(&settings["output"]["voice_chat"]));
  extend!(cfg, "hud_saytext_time {};\n", setting_as_bin(&settings["output"]["text_chat"]) * 12);
  extend!(cfg, "crosshair {};\n", setting_as_bin(&settings["output"]["crosshair"]));
  extend!(cfg, "r_drawviewmodel {};\n", setting_as_bin(&settings["output"]["viewmodel"]));
  extend!(cfg, "tf_use_min_viewmodels {};\n", setting_as_bin(&settings["output"]["minmode"]));
  extend!(
    cfg,
    "viewmodel_fov_demo {};\n",
    setting_as_bin(&settings["recording"]["viewmodel_fov"])
  );
  extend!(cfg, "fov_desired {};\n", setting_as_bin(&settings["recording"]["fov"]));

  if setting_as_bin(&settings["recording"]["third_person"]) == 1 {
    extend!(cfg, "thirdperson{};\n", "");
  } else {
    extend!(cfg, "firstperson{};\n", "");
  }

  if let Some(commands) = settings["recording"]["commands"].as_str() {
    extend!(cfg, "{}\n", commands);
  }

  extend!(cfg, "{};\n", "alias \"snd_fix\" \"snd_restart; snd_soundmixer Default_mix;\"");

  extend!(cfg, "{}", compile_addons(settings));

  let tf_folder = match settings["tf_folder"].as_str() {
    Some(folder) => folder,
    None => {
      return Err("tf_folder setting is not a string".to_string());
    }
  };

  let base_folder = Path::new(tf_folder).parent().unwrap();
  let base_folder_str = base_folder.to_str().unwrap();

  let mut installs: Vec<String> = vec!["tf".to_string()];

  for install in settings["alt_installs"].as_array().unwrap() {
    if let None = install["name"].as_str() {
      continue;
    }

    if let None = install["tf_folder"].as_str() {
      continue;
    }

    let mut install_folder = install["tf_folder"].as_str().unwrap().replace(base_folder_str, "");

    if install_folder.starts_with("\\") {
      install_folder.remove(0);
    }

    installs.push(install_folder);
  }

  for install in installs {
    let install_folder_str = format!("{}\\{}", base_folder_str, install);

    let cfg_path = format!("{}\\cfg", install_folder_str);

    if !Path::new(&cfg_path).exists() {
      match std::fs::create_dir_all(Path::new(&cfg_path)) {
        Ok(_) => {}
        Err(why) => {
          return Err(format!("Couldn't create cfg folder: {}", why));
        }
      }
    }

    let mut file = match File::create(format!("{}\\cfg\\melies.cfg", install_folder_str)) {
      Ok(file) => file,
      Err(why) => {
        return Err(format!("Couldn't create melies.cfg: {}", why));
      }
    };

    match file.write_all(cfg.as_bytes()) {
      Ok(_) => {}
      Err(why) => {
        return Err(format!("Couldn't write melies.cfg: {}", why));
      }
    };
  }

  Ok(())
}