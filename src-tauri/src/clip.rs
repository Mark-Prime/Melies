use serde_json::Value;

use crate::event::Event;
use crate::event::EventStyle::*;

#[derive(Debug)]
pub struct Clip {
    events: Vec<Event>,
    pub demo_name: String,
    pub has_killstreak: bool,
    pub has_bookmark: bool,
    pub ks_value: i64,
    pub bm_value: String,
    pub start_tick: i64,
    pub end_tick: i64,
    pub spec_type: i64,
    pub spec_player: String,
    long_clip: bool,
}

impl Clip {
    pub fn new(event: Event, settings: &Value) -> Clip {
        let demo_name = String::from(event.demo_name.clone());
        match &event.value {
            Killstreak(val) => {
                let mut start_tick = &event.tick
                    - settings["recording"]["before_killstreak_per_kill"]
                        .as_i64()
                        .unwrap()
                        * val;

                if start_tick < 0 {
                    start_tick = settings["recording"]["start_delay"].as_i64().unwrap();
                }

                return Clip {
                    events: vec![event.clone()],
                    demo_name: demo_name.to_string(),
                    has_killstreak: true,
                    has_bookmark: false,
                    ks_value: val.to_owned(),
                    bm_value: "".to_string(),
                    start_tick,
                    end_tick: &event.tick
                        + settings["recording"]["after_killstreak"].as_i64().unwrap(),
                    spec_type: 0,
                    spec_player: "".to_string(),
                    long_clip: false,
                };
            }
            Bookmark(val) => {
                let mut start_tick =
                    &event.tick - settings["recording"]["before_bookmark"].as_i64().unwrap();

                if start_tick < 0 {
                    start_tick = settings["recording"]["start_delay"].as_i64().unwrap();
                }

                let mut clip = Clip {
                    events: vec![event.clone()],
                    demo_name: demo_name.to_string(),
                    has_killstreak: false,
                    has_bookmark: true,
                    ks_value: 0,
                    bm_value: val.to_string(),
                    start_tick,
                    end_tick: &event.tick
                        + settings["recording"]["after_bookmark"].as_i64().unwrap(),
                    spec_type: 0,
                    spec_player: "".to_string(),
                    long_clip: false,
                };

                let commands: Vec<&str> = event.commands();

                if commands.contains(&"spec_third") {
                    clip.spec_type = 3;
                    clip.spec_player = commands.last().unwrap().to_string();
                } else if commands.contains(&"spec") {
                    clip.spec_type = 1;
                    clip.spec_player = commands.last().unwrap().to_string();
                }

                if commands.contains(&"clip_start") {
                    clip.long_clip = true;
                    clip.start_tick = event.tick;

                    if clip.start_tick < 0 {
                        clip.start_tick = settings["recording"]["start_delay"].as_i64().unwrap();
                    }
                }

                return clip;
            }
        }
    }

    fn push_bm_value(&mut self, bm_value: &str) {
        let mut split: Vec<&str> = self.bm_value.split(" ").collect();

        if split.contains(&bm_value) {
            return;
        }

        split.push(bm_value);

        self.bm_value = split.join(" ");
    }

    fn calc_ks(&mut self, ks_count: i64) {
        if self.ks_value + 1 == ks_count {
            self.ks_value = ks_count;
            return;
        }

        self.ks_value = self.ks_value + ks_count;
    }

    pub fn can_include(&mut self, event: &Event, settings: &Value) -> bool {
        if self.demo_name != event.demo_name {
            return false;
        }

        match &event.value {
            Bookmark(bm) => {
                if self.long_clip && bm.contains("clip_end") {
                    self.long_clip = false;
                    return true;
                }

                let mut new_start = event.tick;
                let mut min_tick_between = 0;

                if !bm.contains("clip_start") {
                    if let Some(before_bookmark) = settings["recording"]["before_bookmark"].as_i64()
                    {
                        new_start = event.tick - before_bookmark;
                    }
                    if let Some(min_tick_between_clips) =
                        settings["recording"]["minimum_ticks_between_clips"].as_i64()
                    {
                        min_tick_between = min_tick_between_clips;
                    }
                }

                if new_start - min_tick_between < self.end_tick {
                    return true;
                }
            }
            Killstreak(killstreak_value) => {
                if killstreak_value.to_owned() == self.ks_value + 1 {
                    return true;
                }

                let new_start = &event.tick
                    - settings["recording"]["before_killstreak_per_kill"]
                        .as_i64()
                        .unwrap()
                        * killstreak_value;

                let min_tick_between = settings["recording"]["minimum_ticks_between_clips"]
                    .as_i64()
                    .unwrap();

                if new_start - min_tick_between < self.end_tick {
                    return true;
                }
            }
        }

        false
    }

    pub fn include(&mut self, event: Event, settings: &Value) {
        let mut new_end;

        match &event.value {
            Bookmark(bm) => {
                new_end = &event.tick + settings["recording"]["after_bookmark"].as_i64().unwrap();
                self.has_bookmark = true;

                let commands: Vec<&str> = event.commands();

                if commands.contains(&"spec_third") {
                    self.spec_type = 3;
                    self.spec_player = commands.last().unwrap().to_string();
                } else if commands.contains(&"spec") {
                    self.spec_type = 1;
                    self.spec_player = commands.last().unwrap().to_string();
                }

                if commands.contains(&"clip_start") {
                    self.long_clip = true;
                }

                if bm.contains("clip_end") {
                    new_end = event.tick;
                    self.long_clip = false;
                }

                for val in commands {
                    self.push_bm_value(val);
                }
            }
            Killstreak(killstreak_value) => {
                new_end = &event.tick + settings["recording"]["after_killstreak"].as_i64().unwrap();
                self.has_killstreak = true;

                self.calc_ks(killstreak_value.to_owned());
            }
        }

        if new_end > self.end_tick {
            self.end_tick = new_end;
        }

        self.events.push(event);
    }
}
