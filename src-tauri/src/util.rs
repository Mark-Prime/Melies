use std::fs::DirEntry;
use std::fs;

use serde_json::Value;

pub fn check_dir(files: Result<fs::ReadDir, std::io::Error>) -> Result<String, String> {
  let entries;

  match files {
    Ok(ent) => {
      entries = ent;
    }
    Err(_) => {
      return Err(
        "Could not find the _events.txt or KillStreaks.txt files.\nPlease check your settings to ensure the tf folder is correctly linked.\nIf you do not have either file, please make one in the \\tf or \\tf\\demos folder.".to_string()
      );
    }
  }

  for entry in entries {
    let dir: DirEntry;

    match entry {
      Ok(directory) => {
        dir = directory;
      }
      Err(err) => {
        return Err(err.to_string());
      }
    }

    let dir_str = dir.path().to_string_lossy().to_string();

    if dir_str.ends_with("\\_events.txt") {
      return Ok(dir_str);
    }

    if dir_str.ends_with("\\KillStreaks.txt") {
      return Ok(dir_str);
    }
  }

  Err(format!("File Not Found"))
}

pub fn find_dir(settings: &Value) -> Result<String, String> {
  let tf_folder = settings["tf_folder"].as_str();

  let files = match tf_folder {
    Some(tf_folder) => fs::read_dir(format!("{}\\demos", tf_folder)),
    None => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "tf_folder not set")),
  };

  match check_dir(files) {
    Ok(res) => {
      return Ok(res);
    }
    Err(_) => {}
  }

  let tf_folder = settings["tf_folder"].as_str();
  let files = match tf_folder {
    Some(tf_folder) => fs::read_dir(format!("{}", tf_folder)),
    None => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "tf_folder not set")),
  };

  match check_dir(files) {
    Ok(res) => {
      return Ok(res);
    }
    Err(_) => {}
  }

  match settings["tf_folder"].as_str() {
    Some(tf_folder) =>
      Err(
        format!("Could not find the _events.txt or KillStreaks.txt files.\nPlease check your settings to ensure the tf folder is correctly linked.\nIf you do not have either file, please make one in the \\tf or \\tf\\demos folder. \n\ntf_folder setting: ({})", tf_folder)
      ),
    None => Err("tf_folder setting not set".to_string()),
  }
}