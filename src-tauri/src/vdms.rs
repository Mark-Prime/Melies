use std::path::Path;

use serde_json::{json, Value};
use vdm::{
    action::{Action, ActionType},
    VDM,
};

pub(crate) fn json_to_vdm(json: Value) -> VDM {
    let mut vdm = VDM::new();

    for action in json.as_array().unwrap() {
        let factory = action["factory"].as_str().unwrap();

        let factory = match factory {
            "SkipAhead" => ActionType::SkipAhead,
            "StopPlayback" => ActionType::StopPlayback,
            "PlayCommands" => ActionType::PlayCommands,
            "ScreenFadeStart" => ActionType::ScreenFadeStart,
            "TextMessageStart" => ActionType::TextMessageStart,
            "PlayCDTrackStart" => ActionType::PlayCDTrackStart,
            "PlaySoundStart" => ActionType::PlaySoundStart,
            "Pause" => ActionType::Pause,
            "ChangePlaybackRate" => ActionType::ChangePlaybackRate,
            "ZoomFov" => ActionType::ZoomFov,
            _ => panic!("Invalid action type."),
        };

        let mut new_action = Action::new(factory.clone());

        let props = new_action.props_mut();

        props.name = action["name"].as_str().unwrap().to_string();

        if action["start_tick"] != Value::Null {
            props.start_tick = Some(action["start_tick"].as_i64().unwrap());
        }

        if action["start_time"] != Value::Null {
            props.start_time = Some(action["start_time"].as_f64().unwrap());
        }

        match factory {
            ActionType::SkipAhead => {
                if action["skip_to_tick"] != Value::Null {
                    props.skip_to_tick = Some(action["skip_to_tick"].as_i64().unwrap());
                }

                if action["skip_to_time"] != Value::Null {
                    props.skip_to_time = Some(action["skip_to_time"].as_f64().unwrap());
                }
            }
            ActionType::StopPlayback => {}
            ActionType::PlayCommands => {
                if action["commands"] != Value::Null {
                    props.commands = action["commands"].as_str().unwrap().to_string();
                }
            }
            ActionType::ScreenFadeStart => {
                if action["duration"] != Value::Null {
                    props.duration = action["duration"].as_f64().unwrap();
                }

                if action["hold_time"] != Value::Null {
                    props.hold_time = action["hold_time"].as_f64().unwrap();
                }

                if action["fade_in_enabled"] != Value::Null {
                    props.fade_in_enabled = action["fade_in_enabled"].as_bool().unwrap();
                }

                if action["fade_out_enabled"] != Value::Null {
                    props.fade_out_enabled = action["fade_out_enabled"].as_bool().unwrap();
                }

                if action["modulate_enabled"] != Value::Null {
                    props.modulate_enabled = action["modulate_enabled"].as_bool().unwrap();
                }

                if action["stay_out_enabled"] != Value::Null {
                    props.stay_out_enabled = action["stay_out_enabled"].as_bool().unwrap();
                }

                if action["purge_enabled"] != Value::Null {
                    props.purge_enabled = action["purge_enabled"].as_bool().unwrap();
                }

                if action["rgba1"] != Value::Null {
                    props.rgba1 = [
                        action["rgba1"][0].as_u64().unwrap() as u8,
                        action["rgba1"][1].as_u64().unwrap() as u8,
                        action["rgba1"][2].as_u64().unwrap() as u8,
                        action["rgba1"][3].as_u64().unwrap() as u8,
                    ];
                }
            }
            ActionType::TextMessageStart => {
                if action["message"] != Value::Null {
                    props.message = action["message"].as_str().unwrap().to_string();
                }

                if action["font"] != Value::Null {
                    props.font = action["font"].as_str().unwrap().to_string();
                }

                if action["fade_in"] != Value::Null {
                    props.fade_in = action["fade_in"].as_f64().unwrap();
                }

                if action["fade_out"] != Value::Null {
                    props.fade_out = action["fade_out"].as_f64().unwrap();
                }

                if action["hold_time"] != Value::Null {
                    props.hold_time = action["hold_time"].as_f64().unwrap();
                }

                if action["fx_time"] != Value::Null {
                    props.fx_time = action["fx_time"].as_f64().unwrap();
                }

                if action["effect"] != Value::Null {
                    props.effect = match action["effect"].as_str().unwrap() {
                        "Flicker" => vdm::action::TextEffect::Flicker,
                        "FadeInOut" => vdm::action::TextEffect::FadeInOut,
                        "WriteOut" => vdm::action::TextEffect::WriteOut,
                        _ => panic!("Invalid text effect."),
                    };
                }

                if action["xy"] != Value::Null {
                    props.xy = [
                        action["xy"][0].as_f64().unwrap(),
                        action["xy"][1].as_f64().unwrap(),
                    ];
                }

                if action["rgba1"] != Value::Null {
                    props.rgba1 = [
                        action["rgba1"][0].as_u64().unwrap() as u8,
                        action["rgba1"][1].as_u64().unwrap() as u8,
                        action["rgba1"][2].as_u64().unwrap() as u8,
                        action["rgba1"][3].as_u64().unwrap() as u8,
                    ];
                }

                if action["rgba2"] != Value::Null {
                    props.rgba2 = [
                        action["rgba2"][0].as_u64().unwrap() as u8,
                        action["rgba2"][1].as_u64().unwrap() as u8,
                        action["rgba2"][2].as_u64().unwrap() as u8,
                        action["rgba2"][3].as_u64().unwrap() as u8,
                    ];
                }
            }
            ActionType::PlayCDTrackStart => {
                if action["track"] != Value::Null {
                    props.track = action["track"].as_i64().unwrap();
                }
            }
            ActionType::PlaySoundStart => {
                if action["sound"] != Value::Null {
                    props.sound = action["sound"].as_str().unwrap().to_string();
                }
            }
            ActionType::Pause => {
                if action["stop_tick"] != Value::Null {
                    props.stop_tick = Some(action["stop_tick"].as_i64().unwrap());
                }

                if action["stop_time"] != Value::Null {
                    props.stop_time = Some(action["stop_time"].as_f64().unwrap());
                }

                if action["duration"] != Value::Null {
                    props.duration = action["duration"].as_f64().unwrap();
                }
            }
            ActionType::ChangePlaybackRate => {
                if action["stop_tick"] != Value::Null {
                    props.stop_tick = Some(action["stop_tick"].as_i64().unwrap());
                }

                if action["stop_time"] != Value::Null {
                    props.stop_time = Some(action["stop_time"].as_f64().unwrap());
                }

                if action["playback_rate"] != Value::Null {
                    props.playback_rate = action["playback_rate"].as_f64().unwrap();
                }
            }
            ActionType::ZoomFov => {
                if action["spline"] != Value::Null {
                    props.spline = action["spline"].as_bool().unwrap();
                }

                if action["stayout"] != Value::Null {
                    props.stayout = action["stayout"].as_bool().unwrap();
                }

                if action["final_fov"] != Value::Null {
                    props.final_fov = action["final_fov"].as_f64().unwrap();
                }

                if action["fov_fade_out"] != Value::Null {
                    props.fade_out = action["fov_fade_out"].as_f64().unwrap();
                }

                if action["fov_fade_in"] != Value::Null {
                    props.fade_in = action["fov_fade_in"].as_f64().unwrap();
                }

                if action["fov_hold"] != Value::Null {
                    props.hold_time = action["fov_hold"].as_f64().unwrap();
                }
            }
        }

        vdm.actions.push(new_action);
    }

    vdm
}

pub(crate) fn vdm_to_json(vdm: VDM) -> Value {
    let mut actions: Vec<Value> = vec![];

    for action in vdm.actions {
        match action {
            Action::SkipAhead(props) => {
                actions.push(json!({
                    "factory": "SkipAhead",
                    "name": props.name,
                    "start_tick": props.start_tick,
                    "start_time": props.start_time,
                    "skip_to_tick": props.skip_to_tick,
                    "skip_to_time": props.skip_to_time
                }));
            }
            Action::StopPlayback(props) => {
                actions.push(json!({
                    "factory": "StopPlayback",
                    "name": props.name,
                    "start_tick": props.start_tick,
                    "start_time": props.start_time,
                }));
            }
            Action::PlayCommands(props) => {
                actions.push(json!({
                    "factory": "PlayCommands",
                    "name": props.name,
                    "start_tick": props.start_tick,
                    "start_time": props.start_time,
                    "commands": props.commands
                }));
            }
            Action::ScreenFadeStart(props) => {
                actions.push(json!({
                    "factory": "ScreenFadeStart",
                    "name": props.name,
                    "start_tick": props.start_tick,
                    "start_time": props.start_time,
                    "duration": props.duration,
                    "hold_time": props.hold_time,
                    "fade_in_enabled": props.fade_in_enabled,
                    "fade_out_enabled": props.fade_out_enabled,
                    "modulate_enabled": props.modulate_enabled,
                    "stay_out_enabled": props.stay_out_enabled,
                    "purge_enabled": props.purge_enabled,
                    "rgba1": props.rgba1,
                }));
            }
            Action::TextMessageStart(props) => {
                actions.push(json!({
                    "factory": "TextMessageStart",
                    "name": props.name,
                    "start_tick": props.start_tick,
                    "start_time": props.start_time,
                    "message": props.message,
                    "font": props.font,
                    "fade_in": props.fade_in,
                    "fade_out": props.fade_out,
                    "hold_time": props.hold_time,
                    "fx_time": props.fx_time,
                    "effect": match props.effect {
                        vdm::action::TextEffect::Flicker => "Flicker",
                        vdm::action::TextEffect::FadeInOut => "FadeInOut",
                        vdm::action::TextEffect::WriteOut => "WriteOut",
                    },
                    "xy": props.xy,
                    "rgba1": props.rgba1,
                    "rgba2": props.rgba2,
                }));
            }
            Action::PlayCDTrackStart(props) => {
                actions.push(json!({
                    "factory": "PlayCDTrackStart",
                    "name": props.name,
                    "start_tick": props.start_tick,
                    "start_time": props.start_time,
                    "track": props.track,
                }));
            }
            Action::PlaySoundStart(props) => {
                actions.push(json!({
                    "factory": "PlaySoundStart",
                    "name": props.name,
                    "start_tick": props.start_tick,
                    "start_time": props.start_time,
                    "sound": props.sound,
                }));
            }
            Action::Pause(props) => {
                actions.push(json!({
                    "factory": "Pause",
                    "name": props.name,
                    "start_tick": props.start_tick,
                    "start_time": props.start_time,
                    "stop_tick": props.stop_tick,
                    "stop_time": props.stop_time,
                    "duration": props.duration
                }));
            }
            Action::ChangePlaybackRate(props) => {
                actions.push(json!({
                    "factory": "ChangePlaybackRate",
                    "name": props.name,
                    "start_tick": props.start_tick,
                    "start_time": props.start_time,
                    "stop_tick": props.stop_tick,
                    "stop_time": props.stop_time,
                    "playback_rate": props.playback_rate
                }));
            }
            Action::ZoomFov(props) => {
                actions.push(json!({
                    "factory": "ZoomFov",
                    "name": props.name,
                    "start_tick": props.start_tick,
                    "start_time": props.start_time,
                    "spline": props.spline,
                    "stayout": props.stayout,
                    "final_fov": props.final_fov,
                    "fov_fade_out": props.fade_out,
                    "fov_fade_in": props.fade_in,
                    "fov_hold": props.hold_time,
                }));
            }
        }
    }

    json!(actions)
}

pub(crate) fn cleanup_renamed_vdms(demo_map: Value, vdms: Value, tf_path: &str) {
    for vdm in vdms["vdms"].as_array().unwrap() {
        let demo_name = format!("{}", vdm["name"].as_str().unwrap());

        let dir = format!("{}{}", tf_path, demo_name);

        if !Path::new(&dir).exists() {
            continue;
        }

        let Ok(mut vdm) = VDM::open(&dir) else {
            continue;
        };

        let props = vdm.last_mut().props_mut();

        if props.commands == "quit;" {
            continue;
        }

        let keys = demo_map.as_object().unwrap().keys();

        let mut changed = false;

        for key in keys {
            let formatted = format!(" {};", key);
            if props.commands.contains(&formatted) {
                changed = true;
                props.commands = props.commands.replace(key, demo_map[key].as_str().unwrap());
            }
        }

        if changed {
            vdm.export(&dir);
        }
    }
}
