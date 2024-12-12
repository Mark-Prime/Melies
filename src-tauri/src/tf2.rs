use std::{path::PathBuf, process::Command};
use serde_json::Value;

pub(crate) fn run_tf2(demo_name: &str, settings: &Value) {
  println!("Running TF2");

  let mut launch_options = settings["hlae"]["launch_options"].to_string();

  if demo_name != "" && settings["hlae"]["playdemo"] == true {
    let mut trimmed_name = demo_name.to_string().replace(".dem", "");
    trimmed_name.remove(0);

    println!("Playing demo: {}", trimmed_name);
    launch_options = format!("{} +playdemo {}", launch_options, trimmed_name);
  }

  let mut tf2_path = PathBuf::from(settings["tf_folder"].as_str().unwrap()).parent().unwrap().to_path_buf();

  if settings["hlae"]["use_64bit"] == true {
    tf2_path.push("tf_win64.exe");
  } else {
    tf2_path.push("tf.exe");
  }

  let tf2_str = tf2_path.to_str().unwrap();

  match settings["output"]["method"].as_str().unwrap() {
    "sparklyfx" => {
      let args = vec![
        "-customLoader",
        "-noGui",
        "-autoStart",
        "-hookDllPath",
        settings["hlae"]["sparklyfx_path"].as_str().unwrap(),
        "-programPath",
        tf2_str,
        "-cmdLine",
        launch_options.as_str()
      ];

      Command::new(settings["hlae"]["hlae_path"].as_str().unwrap())
        .args(args)
        .spawn()
        .expect("failed to execute process");
    }
    "svr" => {
      return;
    }
    "svr.mov" => {
      return;
    }
    "svr.mp4" => {
      return;
    }
    _ => {
      Command::new(tf2_path)
        .args(launch_options.as_str().split(" ").collect::<Vec<_>>())
        .spawn()
        .expect("failed to execute process");
    }
  }

  wait_for_tf2(settings);
}

fn wait_for_tf2(settings: &Value) -> bool {
  use sysinfo::System;
  use std::{thread, time};

  loop {
    let five_sec = time::Duration::from_secs(1);

    thread::sleep(five_sec);

    let s = System::new_all();

    let mut process_found = false;

    for _process in s.processes_by_name("tf_win64".as_ref()) {
      process_found = true;
      break;
    }

    if settings["hlae"]["use_64bit"] == true {
      for _process in s.processes_by_name("tf_win64".as_ref()) {
        process_found = true;
        break;
      }
    } else {
      for _process in s.processes_by_name("tf".as_ref()) {
        process_found = true;
        break;
      }
    }

    if !process_found {
      break;
    }
  }

  true
}