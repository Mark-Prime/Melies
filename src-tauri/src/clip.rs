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
    pub end_tick: i64
}

impl Clip {
    pub fn new(event: Event, settings: &Value) -> Clip {
        let demo_name = String::from(event.demo_name.clone());
        match &event.value {
            Killstreak(killstreak_value) => {
                return Clip {
                    events: vec![event.clone()],
                    demo_name: demo_name.to_string(),
                    has_killstreak: true,
                    has_bookmark: false,
                    ks_value: killstreak_value.to_owned(),
                    bm_value: "".to_string(),
                    start_tick: &event.tick - (settings["recording"]["before_killstreak_per_kill"].as_i64().unwrap() * killstreak_value),
                    end_tick: &event.tick + settings["recording"]["after_killstreak"].as_i64().unwrap()
                };
            },
            Bookmark(bookmark_value) => {
                return Clip {
                    events: vec![event.clone()],
                    demo_name: demo_name.to_string(),
                    has_killstreak: false,
                    has_bookmark: true,
                    ks_value: 0,
                    bm_value: bookmark_value.to_string(),
                    start_tick: &event.tick - settings["recording"]["before_bookmark"].as_i64().unwrap(),
                    end_tick: &event.tick + settings["recording"]["after_bookmark"].as_i64().unwrap()
                };
            },
        }
    }

    pub fn first(&self) -> &Event {
        &self.events[&self.events.len() - 1]
    }

    pub fn last(&self) -> &Event {
        &self.events[0]
    }

    pub fn first_mut(&mut self) -> &mut Event {
        let i = &self.events.len() - 1;
        &mut self.events[i]
    }

    pub fn last_mut(&mut self) -> &mut Event {
        &mut self.events[0]
    }

    fn calc_ks(&mut self, ks_count: i64) {
        if (self.ks_value + 1) == ks_count {
            self.ks_value = ks_count;
            return;
        }
        
        self.ks_value = self.ks_value + ks_count;
    }

    pub fn can_include(&self, event: &Event, settings: &Value ) -> bool {
        if self.demo_name != event.demo_name {
            return false;
        }
        
        match event.value {
            Bookmark(_) => {
                let new_start = &event.tick - settings["recording"]["before_bookmark"].as_i64().unwrap();

                let min_tick_between = settings["recording"]["minimum_ticks_between_clips"].as_i64().unwrap();

                if (new_start - min_tick_between) < self.end_tick {
                    return true;
                }
            },
            Killstreak(killstreak_value) => {
                if killstreak_value.to_owned() == (self.ks_value + 1) {
                    return true;
                }

                let new_start = &event.tick - (settings["recording"]["before_killstreak_per_kill"].as_i64().unwrap() * killstreak_value);

                let min_tick_between = settings["recording"]["minimum_ticks_between_clips"].as_i64().unwrap();

                if (new_start - min_tick_between) < self.end_tick {
                    return true;
                }
            }
        };

        false
    }

    pub fn include(&mut self, event: Event, settings: &Value) {
        let new_end;

        match &event.value {
            Bookmark(_) => {
                new_end = &event.tick + settings["recording"]["after_bookmark"].as_i64().unwrap();
                self.has_bookmark = true;
            },
            Killstreak(killstreak_value) => {
                new_end = &event.tick + settings["recording"]["after_killstreak"].as_i64().unwrap();
                self.has_killstreak = true;

                self.calc_ks(killstreak_value.to_owned());
            }
        };

        if new_end > self.end_tick {
            self.end_tick = new_end;
        }
        
        self.events.push(event);
    }
}