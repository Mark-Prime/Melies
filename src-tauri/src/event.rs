use regex::Captures;

#[derive(Debug)]
pub enum EventError {
    InvalidInput,
    InvalidTick
}

#[derive(Debug, PartialEq)]
pub enum EventStyle {
    Bookmark(String),
    Killstreak(i64)
}

#[derive(Debug)]
pub struct Event {
    event: String,
    pub demo_name: String,
    pub tick: i64,
    pub value: EventStyle 
}

impl Event {
    pub fn new(input: Captures) -> Result<Event, EventError> {
        let tick;

        match input[4].to_string().parse::<i64>() {
            Ok(num) => {
                tick = num
            },
            Err(_) => {
                return Err(EventError::InvalidTick);
            }
        }

        let split = input[2].to_string();

        let mut event_name: Vec<&str> = split.split_whitespace().collect();
        let value: EventStyle;

        let name = event_name[0].to_lowercase();

        match name.as_str() {
            "kill" => {
                let streak = event_name[1].to_string();
                let streak_split: Vec<&str> = streak.split(":").collect();
                value = EventStyle::Killstreak(streak_split[1].to_string().parse::<i64>().unwrap());
            },
            "killstreak" => {
                value = EventStyle::Killstreak(event_name[1].to_string().parse::<i64>().unwrap());
            },
            "bookmark" => {
                event_name.remove(0);
                value = EventStyle::Bookmark(event_name.join(" "));
            },
            _ => {
                value = EventStyle::Bookmark(event_name.join(" "));
            }
        }

        Ok(Event {
            event: input[0].to_string(),
            demo_name: input[3].to_string(),
            tick,
            value
        })
    }
}