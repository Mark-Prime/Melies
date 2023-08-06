use std::fs;
use std::cmp::Ordering;
use std::collections::HashMap;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use tf_demo_parser::{ Demo, DemoParser };
use std::path::Path;

use self::custom_analyser::Death;
use self::custom_analyser::DemoTick;
use self::custom_analyser::Spawn;

macro_rules! ifelse {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}

mod custom_analyser;

#[derive(Debug, Serialize, Clone)]
enum Event {
    Death(Death),
    Spawn(Spawn),
    Kill(Death),
    Assist(Death),
    RoundEnd(u32),
}

impl Event {
    fn tick(&self) -> u32 {
        match self {
            Event::Death(val) => {
                return val.tick.into();
            }
            Event::Spawn(val) => {
                return val.tick.into();
            }
            Event::Kill(val) => {
                return val.tick.into();
            }
            Event::Assist(val) => {
                return val.tick.into();
            }
            Event::RoundEnd(val) => {
                return val.to_owned();
            }
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tick().cmp(&other.tick())
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Event {}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.tick() == other.tick()
    }
}

#[derive(Debug, Serialize, Clone)]
struct Killstreak {
    pub kills: Vec<Death>,
    pub classes: Vec<String>,
}

impl Killstreak {
    fn new(death: Death) -> Self {
        Killstreak {
            kills: vec![death],
            classes: vec![],
        }
    }

    fn average(&self) -> u32 {
        let mut total = 0;

        for kill in &self.kills {
            total = total + u32::from(kill.tick);
        }

        return total / (self.kills.len() as u32);
    }
}

#[derive(Debug, Serialize, Clone)]
struct Life {
    pub start: u32,
    pub end: u32,
    pub last_kill_tick: DemoTick,
    pub killstreaks: Vec<Killstreak>,
    pub kills: Vec<Death>,
    pub assists: Vec<Death>,
    pub classes: Vec<String>,
}

impl Life {
    fn new(start: u32, classes: Vec<String>) -> Life {
        Life {
            start,
            end: 0,
            last_kill_tick: DemoTick::from(0),
            killstreaks: vec![],
            kills: vec![],
            assists: vec![],
            classes,
        }
    }
}

pub(crate) fn scan_for_demos(settings: Value) -> Value {
    let mut demos: Vec<Value> = vec![];
    let paths = fs::read_dir(settings["tf_folder"].as_str().unwrap()).unwrap();

    for path in paths {
        let file_name = path.unwrap().path().display().to_string();

        if file_name.ends_with(".dem") {
            let parsed_file_name = file_name.replace(settings["tf_folder"].as_str().unwrap(), "");

            let mut demo = Value::from({});
            demo["name"] = Value::from(parsed_file_name);

            demos.push(demo);
        }
    }

    if Path::new(&format!("{}\\demos", settings["tf_folder"].as_str().unwrap())).exists() {
        let paths = fs
            ::read_dir(format!("{}\\demos", settings["tf_folder"].as_str().unwrap()))
            .unwrap();

        for path in paths {
            let file_name = path.unwrap().path().display().to_string();

            if file_name.ends_with(".dem") {
                let parsed_file_name = file_name.replace(
                    settings["tf_folder"].as_str().unwrap(),
                    ""
                );

                let mut demo = Value::from({});
                demo["name"] = Value::from(parsed_file_name);

                demos.push(demo);
            }
        }
    }

    let mut resp = Value::from({});

    resp["loaded"] = Value::from(true);
    resp["demos"] = Value::from(demos);

    return resp;
}

pub(crate) fn scan_demo(settings: Value, path: String) -> Value {
    println!("{}{}", settings["tf_folder"].as_str().unwrap(), path);

    let file_path = format!("{}{}", settings["tf_folder"].as_str().unwrap(), path);

    let file = fs::read(file_path).unwrap();

    let demo = Demo::new(&file);

    let parser = DemoParser::new_all_with_analyser(
        demo.get_stream(),
        custom_analyser::Analyser::new()
    );
    let (header, state) = parser.parse().unwrap();

    let mut user_events: HashMap<u16, Vec<Event>> = HashMap::new();

    for death in &state.deaths {
        let killer = user_events.entry(death.killer.0.into()).or_insert(vec![]);

        killer.push(Event::Kill(death.clone()));

        let assist_id = death.assister;

        if assist_id.is_some() {
            let assister = user_events.entry(assist_id.unwrap().0.into()).or_insert(vec![]);

            assister.push(Event::Assist(death.clone()));
        }

        let victim = user_events.entry(death.victim.0.into()).or_insert(vec![]);

        victim.push(Event::Death(death.clone()));
    }

    for spawn in &state.spawns {
        let user = user_events.entry(spawn.user.0.into()).or_insert(vec![]);

        user.push(Event::Spawn(spawn.clone()));
    }

    for round in &state.rounds {
        for user in &mut user_events {
            user.1.push(Event::RoundEnd(round.end_tick.into()));
        }
    }

    let mut sorted_events: HashMap<u16, Vec<Event>> = HashMap::new();
    let mut player_lives: HashMap<u16, Vec<Life>> = HashMap::new();
    let mut killstreaks: Vec<Killstreak> = vec![];

    for (key, events) in &user_events {
        let mut current_player = vec![];
        let mut events = events.to_owned();

        events.sort_by(|a, b| a.cmp(b));

        let mut current_life: Life = Life::new(0, vec!["".to_string()]);

        for event in &events {
            let streak_count = current_life.killstreaks.len();
            let mut kill_count = 0;

            if streak_count > 0 {
                kill_count = current_life.killstreaks[streak_count - 1].kills.len();
            }

            match event {
                Event::Spawn(spawn) => {
                    if current_life.start == 0 {
                        current_life = Life::new(spawn.tick.into(), vec![spawn.class.to_string()]);
                    } else if !current_life.classes.contains(&spawn.class.to_string()) {
                        current_life.classes.push(spawn.class.to_string());
                    }
                }
                Event::Kill(kill) => {
                    if kill.killer == kill.victim {
                        continue;
                    }

                    if kill_count == 0 {
                        current_life.killstreaks.push(Killstreak::new(kill.to_owned()));
                    } else if
                        kill.tick.as_i64() <
                        current_life.last_kill_tick.as_i64() +
                            settings["recording"]["before_killstreak_per_kill"].as_i64().unwrap()
                    {
                        current_life.killstreaks[streak_count - 1].kills.push(kill.to_owned());
                    } else if kill_count < 3 {
                        current_life.killstreaks[streak_count - 1].kills = vec![kill.to_owned()];
                    }

                    current_life.last_kill_tick = kill.tick;

                    current_life.kills.push(kill.to_owned());
                }
                Event::Assist(assist) => {
                    current_life.assists.push(assist.to_owned());
                }
                Event::Death(death) => {
                    if current_life.start != 0 {
                        if kill_count > 0 && kill_count < 3 {
                            current_life.killstreaks.remove(streak_count - 1);
                        }

                        for mut ks in current_life.killstreaks.to_owned() {
                            ks.classes = current_life.classes.clone();
                            killstreaks.push(ks);
                        }

                        current_life.end = death.tick.into();

                        current_player.push(current_life);

                        current_life = Life::new(0, vec!["".to_string()]);
                    }
                }
                Event::RoundEnd(tick) => {
                    if kill_count > 0 && kill_count < 3 {
                        current_life.killstreaks.remove(streak_count - 1);
                    }

                    for mut ks in current_life.killstreaks.to_owned() {
                        ks.classes = current_life.classes.clone();
                        killstreaks.push(ks);
                    }

                    current_life.end = tick.to_owned();
                    current_player.push(current_life);

                    current_life = Life::new(0, vec!["".to_string()]);
                }
            }
        }

        player_lives.insert(key.to_owned(), current_player);
        sorted_events.insert(key.to_owned(), events.to_vec());
    }

    killstreaks.sort_by_key(|ks| ks.average());

    let mut start_tick = state.end_tick - header.ticks;

    if header.ticks == 0 {
        start_tick = state.start_tick;
    }

    let resp =
        json!({
        "header": {
            "demo_type": header.demo_type,
            "version": header.version,
            "protocol": header.protocol,
            "server": header.server,
            "nick": header.nick,
            "map": header.map,
            "game": header.game,
            "duration": header.duration,
            "ticks": ifelse!(header.ticks != 0, header.ticks, state.end_tick.0 - state.start_tick.0),
            "frames": header.frames,
            "signon": header.signon,
        },
        "data": {
            // "deaths": state.deaths,
            // "spawns": state.spawns,
            "rounds": state.rounds,
            "users": state.users,
            "chat": state.chat,
            "start_tick": start_tick,
            "end_tick": state.end_tick,
            "user_events": sorted_events,
            "player_lives": player_lives,
            "killstreaks": killstreaks,
        },
        "loaded": true,
        "loading": false
    });

    Value::from(resp)
}
