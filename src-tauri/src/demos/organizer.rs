use std::collections::HashMap;

use tf_demo_parser::demo::data::DemoTick;
use tf_demo_parser::demo::header::Header;

use crate::demos::new_analyser::MatchState;

use crate::demos::{ClassSpawn, Event, get_player_class};

pub fn get_classes(state: &MatchState, user_events: &mut HashMap<u16, Vec<Event>>, user_classes: &mut HashMap<u16, Vec<ClassSpawn>>) {
  for spawn in &state.spawns {
    let user_id: u16 = spawn.user.into();
    let user = user_events.entry(user_id).or_insert(vec![]);
    let user_class = user_classes.entry(user_id).or_insert(vec![]);

    user_class.push(ClassSpawn {
      class: spawn.class.clone(),
      tick: spawn.tick,
    });

    user.push(Event::Spawn(spawn.clone()));
  }
}

pub fn get_kills_assists(state: &mut MatchState, user_events: &mut HashMap<u16, Vec<Event>>, user_classes: &HashMap<u16, Vec<ClassSpawn>>) {
  for death in &mut state.deaths {
    let killer = user_events.entry(death.killer.into()).or_insert(vec![]);

    {
      let user_classes_clone = user_classes.clone();
      death.killer_class = get_player_class(user_classes_clone, death.killer.into(), death.tick);
    }

    {
      let user_classes_clone = user_classes.clone();
      death.victim_class = get_player_class(user_classes_clone, death.victim.into(), death.tick);
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
}

pub fn get_rounds(state: &mut MatchState, user_events: &mut HashMap<u16, Vec<Event>>, header: &Header) {
  for round in &state.rounds {
    for user in &mut *user_events {
      user.1.push(Event::RoundEnd(round.end_tick.into()));
    }
  }

  if !state.rounds.is_empty() {
    let round = state.rounds.last_mut().unwrap();

    if round.end_tick == DemoTick::from(0) {
      round.end_tick = DemoTick::from(header.ticks);
    }
  }
}