use std::fmt::Write;
use std::fs;

use crate::{impl_open_file, setting_as_bin, CONF_DIR};
use serde_json::Value;

fn compile_addon_settings(v: &Value, depth: usize, buf: &mut String) {
    let v_map = match v.as_object() {
        Some(v_map) => v_map,
        None => return,
    };

    let tab_depth = "\t".repeat(depth);

    let mut vec = v_map.iter().collect::<Vec<(&String, &Value)>>();

    vec.sort_by(|a, b| (a.1["type"] == "group").cmp(&(b.1["type"] == "group")));

    for (ki, vi) in vec {
        if vi["ignoreIfDefault"] == true && vi["value"] == vi["default"] {
            continue;
        }

        match &vi["type"] {
            Value::String(vi_type) => match vi_type.as_str() {
                "group" => {
                    let mut addon_text = String::new();
                    compile_addon_settings(&vi["settings"], depth + 1, &mut addon_text);

                    if addon_text.is_empty() {
                        continue;
                    }

                    write!(buf, "\n{}\t", tab_depth);
                    write!(buf, "\\\\ {}\n", ki.as_str());
                    buf.push_str(&addon_text);
                }
                "toggle" if vi["value"] == true => {
                    write!(
                        buf,
                        "{};\n",
                        tab_depth.clone() + vi["command"].as_str().unwrap()
                    );
                }
                "bool" => {
                    write!(
                        buf,
                        "{}{} {};\n",
                        tab_depth.clone(),
                        vi["command"].as_str().unwrap(),
                        setting_as_bin(&vi["value"])
                    );
                }
                "string" | "int" => {
                    if vi["ignoreIfBlank"] == true && vi["value"] == "" {
                        continue;
                    }

                    write!(
                        buf,
                        "{}{} {};\n",
                        tab_depth.clone(),
                        vi["command"].as_str().unwrap(),
                        vi["value"]
                    );
                }
                _ => {
                    continue;
                }
            },
            _ => {
                continue;
            }
        }
    }
}

pub fn compile_addons(settings: &Value) -> String {
    let addons = settings["addons"].as_object();

    let mut cfg = String::new();

    if addons.is_none() {
        return cfg;
    }

    let addons = addons.unwrap();

    let iter = addons.into_iter();

    let mut vec = iter.collect::<Vec<(&String, &Value)>>();

    vec.sort_by(|a, b| (a.1["type"] == "group").cmp(&(b.1["type"] == "group")));

    for (k, v) in addons {
        let mut addon_text = String::new();
        compile_addon_settings(v, 0, &mut addon_text);

        if addon_text.is_empty() {
            continue;
        }

        write!(&mut cfg, "\necho \"Running {} addon\";\n", k);
        write!(&mut cfg, "{}\n", addon_text);
    }

    return cfg;
}

pub fn open_addons_folder() {
    let addons_path = CONF_DIR.join("addons");

    fs::create_dir_all(&addons_path).unwrap();

    impl_open_file(addons_path);
}
