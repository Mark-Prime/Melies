use reqwest;
use serde_json::{self, json, Value};
use steamid_ng::SteamID;

pub(crate) fn parse(url: Value) -> Value {
    let log_id = url.as_str().unwrap();
    println!("Logs.tf url: https://logs.tf/json/{}", log_id);
    let res = reqwest::blocking::get(format!("https://logs.tf/json/{}", log_id));

    let binding;

    if let Ok(resp) = res {
        binding = resp.text().unwrap();
    } else {
        return json!({
            "code": 404,
            "err_text": "Unable to load logs.tf data"
        });
    }

    let resp = binding.as_str();
    let json_data = serde_json::from_str(resp);

    if json_data.is_err() {
        return json!({
            "code": 404,
            "err_text": "Unable to load logs.tf data"
        });
    }

    let mut info: Value = json_data.unwrap();

    let players = &info["players"].as_object().unwrap().to_owned();

    let mut demo_api = "https://api.demos.tf/demos/?".to_owned();

    let mut important_info = Value::from({});

    for player in players {
        let steamid3 = player.0;
        let steamid64 = u64::from(SteamID::from_steam3(steamid3).unwrap());
        info["players"][steamid3]["steamid64"] = Value::from(steamid64.to_string());
        demo_api = demo_api + "players[]=" + &steamid64.to_string() + "&";
    }

    let demo_api = format!(
        "{}map={}&after={}&before={}",
        demo_api,
        info["info"]["map"].as_str().unwrap(),
        info["info"]["date"],
        info["info"]["date"].as_i64().unwrap() + 300
    );

    let binding = reqwest::blocking::get(demo_api).unwrap().text().unwrap();
    let resp = binding.as_str();
    let demo_info: Value = serde_json::from_str(resp).unwrap();

    if demo_info[0].is_null() {
        println!("Unable to find demo");
    } else {
        println!("Demos.tf url: https://demos.tf/{}", demo_info[0]["id"]);
        println!("Download: {}", demo_info[0]["url"].as_str().unwrap());

        important_info["demo_url"] =
            Value::from(format!("https://demos.tf/{}", demo_info[0]["id"]));
        important_info["demo_download"] = Value::from(demo_info[0]["url"].as_str().unwrap());
    }

    important_info["players"] = info["players"].clone();
    important_info["rounds"] = info["rounds"].clone();
    important_info["killstreaks"] = info["killstreaks"].clone();
    important_info["names"] = info["names"].clone();
    important_info["info"] = info["info"].clone();

    return important_info;
}
