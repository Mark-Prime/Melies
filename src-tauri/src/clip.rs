use json::JsonValue;

use crate::event::Event;
use crate::event::EventStyle::*;

enum ClipError {
    
}

pub struct Clip {
    events: Vec<Event>,
    demo_name: String,
    pub has_killstreak: bool,
    pub has_bookmark: bool,
    ks_value: i64,
    bm_value: String
}

impl Clip {
    pub fn new(event: Event) -> Clip {
        let demo_name = String::from(event.demo_name.clone());
        match &event.value {
            Killstreak(killstreak_value) => {
                return Clip {
                    events: vec![event.clone()],
                    demo_name: demo_name.to_string(),
                    has_killstreak: true,
                    has_bookmark: false,
                    ks_value: killstreak_value.to_owned(),
                    bm_value: String::new()
                };
            },
            Bookmark(bookmark_value) => {
                return Clip {
                    events: vec![event.clone()],
                    demo_name: demo_name.to_string(),
                    has_killstreak: false,
                    has_bookmark: true,
                    ks_value: 0,
                    bm_value: bookmark_value.to_owned()
                };
            },
        }
    }

    fn first(&self) -> &Event {
        return &self.events[&self.events.len() - 1];
    }

    fn last(&self) -> &Event {
        return &self.events[0];
    }

    pub fn can_include(&self, event: Event ) -> bool {
        // println!("{:?}", &self.first());
        println!("{:?}", &self.last());
        println!("{:?}", event);
        return true;
    }
}