use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::vec;
use tf_demo_parser::{ Demo, DemoParser };

use crate::demos::new_analyser::Class;

use self::new_analyser::Death;
use self::new_analyser::Spawn;
use tf_demo_parser::demo::data::DemoTick;

// macro_rules! ifelse {
//     ($c:expr, $v:expr, $v1:expr) => {
//         if $c {$v} else {$v1}
//     };
// }

mod new_analyser;

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
struct KillstreakPointer {
    pub owner_id: u16,
    pub life_index: usize,
    pub kills: Vec<usize>,
    pub index: usize,
    pub average: u32,
}

impl KillstreakPointer {
    fn new(owner_id: u16, life_index: usize, kill_index: usize, index: usize) -> Self {
        KillstreakPointer {
            owner_id,
            life_index,
            index,
            kills: vec![kill_index],
            average: 0,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
struct MedPick {
    pub owner_id: u16,
    pub life_index: usize,
    pub kill_index: usize,
}

impl MedPick {
    fn new(owner_id: u16, life_index: usize, kill_index: usize) -> Self {
        MedPick {
            owner_id,
            life_index,
            kill_index
        }
    }
}

#[derive(Debug, Serialize, Clone)]
struct Life {
    pub start: u32,
    pub end: u32,
    pub last_kill_tick: DemoTick,
    pub killstreak_pointers: Vec<KillstreakPointer>,
    pub med_picks: Vec<MedPick>,
    pub kills: Vec<Death>,
    pub assists: Vec<Death>,
    pub classes: Vec<String>,
    pub finalized: bool,
}

impl Life {
    fn new(start: u32, classes: Vec<String>) -> Life {
        Life {
            start,
            end: 0,
            last_kill_tick: DemoTick::from(0),
            killstreak_pointers: vec![],
            med_picks: vec![],
            kills: vec![],
            assists: vec![],
            classes,
            finalized: false,
        }
    }
}

pub(crate) fn validate_demos_folder(settings: &Value) -> bool {
    match fs::read_dir(settings["tf_folder"].as_str().unwrap()) {
        Ok(_) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}

pub(crate) fn scan_for_demos(settings: Value) -> Value {
    let mut demos: Vec<Value> = vec![];

    if !Path::new(settings["tf_folder"].as_str().unwrap()).exists() {
        return Value::from({});
    }

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

#[derive(Clone, Copy, Debug)]
struct ClassSpawn {
    pub class: Class,
    pub tick: DemoTick,
}

fn get_player_class(
    mut user_classes: HashMap<u16, Vec<ClassSpawn>>,
    user_id: u16,
    tick: DemoTick
) -> Class {
    let player = user_classes.entry(user_id).or_insert(vec![]);

    for (i, class) in player.iter().enumerate() {
        if player.len() <= i + 1 {
            return class.class;
        }

        if tick > class.tick && tick < player[i + 1].tick {
            return class.class;
        }
    }

    Class::Other
}

pub(crate) fn scan_demo(settings: Value, path: String) -> Value {
    println!("{}{}", settings["tf_folder"].as_str().unwrap(), path);

    let file_path = format!("{}{}", settings["tf_folder"].as_str().unwrap(), path);

    let file = fs::read(file_path).unwrap();

    let demo = Demo::new(&file);

    let parser = DemoParser::new_all_with_analyser(
        demo.get_stream(),
        new_analyser::Analyser::new()
    );
    let (header, mut state) = match parser.parse() {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return json!({
                "code": 400,
                "err_text": format!("{}", err)
            });
        }
    };

    let mut user_events: HashMap<u16, Vec<Event>> = HashMap::new();
    let mut user_classes: HashMap<u16, Vec<ClassSpawn>> = HashMap::new();

    for spawn in &state.spawns {
        let user = user_events.entry(spawn.user.0.into()).or_insert(vec![]);
        let user_class = user_classes.entry(spawn.user.0.into()).or_insert(vec![]);

        user_class.push(ClassSpawn {
            class: spawn.class.clone(),
            tick: spawn.tick,
        });

        user.push(Event::Spawn(spawn.clone()));
    }

    for death in &mut state.deaths {
        let killer = user_events.entry(death.killer.into()).or_insert(vec![]);

        {
            let user_classes_clone = user_classes.clone();
            death.killer_class = get_player_class(
                user_classes_clone,
                death.killer.into(),
                death.tick
            );
        }

        {
            let user_classes_clone = user_classes.clone();
            death.victim_class = get_player_class(
                user_classes_clone,
                death.victim.into(),
                death.tick
            );
        }

        killer.push(Event::Kill(death.clone()));

        let assist_id = death.assister;

        if assist_id.is_some() {
            let assister = user_events.entry(assist_id.unwrap().into()).or_insert(vec![]);

            assister.push(Event::Assist(death.clone()));
        }

        let victim = user_events.entry(death.victim.into()).or_insert(vec![]);

        victim.push(Event::Death(death.clone()));
    }

    for round in &state.rounds {
        for user in &mut user_events {
            user.1.push(Event::RoundEnd(round.end_tick.into()));
        }
    }

    let mut sorted_events: HashMap<u16, Vec<Event>> = HashMap::new();
    let mut player_lives: HashMap<u16, Vec<Life>> = HashMap::new();
    let mut killstreak_pointers: Vec<KillstreakPointer> = vec![];
    let mut med_picks: Vec<MedPick> = vec![];

    for (key, events) in &user_events {
        let mut current_player = vec![];
        let mut events = events.to_owned();

        events.sort_by(|a, b| a.cmp(b));

        let mut current_life: Life = Life::new(0, vec!["".to_string()]);

        for event in &events {
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

                    if current_life.start == 0 {
                        if current_player.last().is_some() {
                            let previous_life: &Life = current_player.last().unwrap();

                            if previous_life.finalized {
                                continue;
                            }
                        }

                        current_life = match current_player.pop() {
                            Some(val) => val,
                            None => {
                                continue;
                            }
                        };
                    }

                    // println!("{:?}", kill);
                    if kill.victim_class == Class::Medic {
                        current_life.med_picks.push(MedPick::new(*key, current_player.len(), current_life.kills.len()));
                        med_picks.push(MedPick::new(*key, current_player.len(), current_life.kills.len()));
                    }

                    current_life.last_kill_tick = kill.tick;

                    current_life.kills.push(kill.to_owned());
                }
                Event::Assist(assist) => {
                    if current_life.start == 0 {
                        if current_player.last().is_some() {
                            let previous_life: &Life = current_player.last().unwrap();

                            if previous_life.finalized {
                                continue;
                            }
                        }

                        current_life = match current_player.pop() {
                            Some(val) => val,
                            None => {
                                continue;
                            }
                        };
                    }

                    current_life.assists.push(assist.to_owned());
                }
                Event::Death(death) => {
                    if death.killer == 0 {
                        continue;
                    }

                    if current_life.start == 0 {
                        current_life = match current_player.pop() {
                            Some(val) => val,
                            None => {
                                continue;
                            }
                        };
                    }

                    current_life.end = death.tick.into();

                    if !current_life.classes.contains(&"spy".to_string()) {
                        current_life.finalized = true;
                    }

                    current_player.push(current_life);

                    current_life = Life::new(0, vec!["".to_string()]);
                }
                Event::RoundEnd(tick) => {
                    current_life.end = tick.to_owned();
                    current_life.finalized = true;
                    current_player.push(current_life);

                    current_life = Life::new(0, vec!["".to_string()]);
                }
            }
        }

        for (life_index, life) in current_player.iter_mut().enumerate() {
            if life.kills.len() < 3 {
                continue;
            }

            let mut kill_count = 0;
            let mut streak_count = 0;
            let mut last_kill_tick: i64 = 0;

            for (kill_index, kill) in life.kills.iter().enumerate() {
                if kill_count == 0 {
                    life.killstreak_pointers.push(KillstreakPointer::new(key.to_owned(), life_index, kill_index, life.killstreak_pointers.len()));
                    last_kill_tick = kill.tick.0.into();
                    kill_count += 1;
                    streak_count += 1;
                } else if
                    (kill.tick.0 as i64) <
                    last_kill_tick +
                        settings["recording"]["before_killstreak_per_kill"].as_i64().unwrap()
                {
                    life.killstreak_pointers[streak_count - 1].kills.push(kill_index);
                    kill_count += 1;
                } else if kill_count < 3 {
                    kill_count = 1;
                    last_kill_tick = kill.tick.0.into();
                    life.killstreak_pointers[streak_count - 1] = KillstreakPointer::new(key.to_owned(), life_index, kill_index, life.killstreak_pointers.len() - 1);
                }
            }

            life.killstreak_pointers = life.killstreak_pointers
                .iter()
                .map(|v| v.clone())
                .filter(|sen| sen.kills.len() >= 3)
                .collect::<Vec<KillstreakPointer>>();

            for ks in &life.killstreak_pointers {
                killstreak_pointers.push(ks.clone());
            }
        }

        player_lives.insert(key.to_owned(), current_player);
        sorted_events.insert(key.to_owned(), events.to_vec());
    }

    killstreak_pointers.sort_by_key(|ks| ks.average);

    let start_tick = state.start_tick;

    let ticks = header.ticks;

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
            "ticks": ticks,
            "frames": header.frames,
            "signon": header.signon,
        },
        "data": {
            "deaths": state.deaths,
            "spawns": state.spawns,
            "rounds": state.rounds,
            "users": state.users,
            "chat": state.chat,
            "start_tick": start_tick,
            "end_tick": state.end_tick,
            "user_events": sorted_events,
            "player_lives": player_lives,
            "med_picks": med_picks,
            "killstreak_pointers": killstreak_pointers,
            "pauses": state.pauses
        },
        "loaded": true,
        "loading": false
    });

    Value::from(resp)
}
