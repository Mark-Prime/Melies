use tf_demo_parser::demo::data::{DemoTick, ServerTick};
use tf_demo_parser::demo::gameevent_gen::{
    GameEvent, PlayerChargeDeployedEvent, PlayerDeathEvent, PlayerSpawnEvent, TeamPlayRoundWinEvent,
};
use tf_demo_parser::demo::message::packetentities::{EntityId, PacketEntitiesMessage};
use tf_demo_parser::demo::message::usermessage::{ChatMessageKind, SayText2Message, UserMessage};
use tf_demo_parser::demo::message::{Message, MessageType};
use tf_demo_parser::demo::packet::stringtable::StringTableEntry;
use tf_demo_parser::demo::parser::gamestateanalyser::UserId;
use tf_demo_parser::demo::parser::handler::{BorrowMessageHandler, MessageHandler};
use tf_demo_parser::demo::vector::Vector;
use tf_demo_parser::{ParserState, ReadResult, Stream};
// use bitbuffer::{BitWrite, BitWriteStream, Endianness};
use num_enum::TryFromPrimitive;
use parse_display::{Display, FromStr};
use serde::de::Error;
use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};
use std::convert::TryFrom;
use std::ops::{Index, IndexMut};
use std::vec;
use steamid_ng::SteamID;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    pub kind: ChatMessageKind,
    pub from: EntityId,
    pub name: String,
    pub text: String,
    pub tick: DemoTick,
    // pub message: SayText2Message,
}

impl ChatMessage {
    pub fn from_message(message: &SayText2Message, tick: DemoTick) -> Self {
        ChatMessage {
            kind: message.kind,
            name: message
                .from
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_default(),
            from: message.client,
            text: message.plain_text(),
            tick,
            // message: message.clone()
        }
    }
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Default,
)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum Team {
    #[default]
    Other = 0,
    Spectator = 1,
    Red = 2,
    Blue = 3,
}

impl Team {
    pub fn new<U>(number: U) -> Self
    where
        u8: TryFrom<U>,
    {
        Team::try_from(u8::try_from(number).unwrap_or_default()).unwrap_or_default()
    }
}

#[derive(
    Debug, Clone, Serialize, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Display, FromStr, Default,
)]
#[display(style = "lowercase")]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum Class {
    #[default]
    Other = 0,
    Scout = 1,
    Sniper = 2,
    Soldier = 3,
    Demoman = 4,
    Medic = 5,
    Heavy = 6,
    Pyro = 7,
    Spy = 8,
    Engineer = 9,
}

impl<'de> Deserialize<'de> for Class {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, Debug)]
        #[serde(untagged)]
        enum IntOrStr<'a> {
            Int(u8),
            Str(&'a str),
        }

        let raw = IntOrStr::deserialize(deserializer)?;
        match raw {
            IntOrStr::Int(class) => Class::try_from_primitive(class).map_err(D::Error::custom),
            IntOrStr::Str(class) if class.len() == 1 => {
                Class::try_from_primitive(class.parse().map_err(D::Error::custom)?)
                    .map_err(D::Error::custom)
            }
            IntOrStr::Str(class) => class.parse().map_err(D::Error::custom),
        }
    }
}

#[test]
fn test_class_deserialize() {
    assert_eq!(Class::Scout, serde_json::from_str(r#""scout""#).unwrap());
    assert_eq!(Class::Scout, serde_json::from_str(r#""1""#).unwrap());
    assert_eq!(Class::Scout, serde_json::from_str("1").unwrap());
}

impl Class {
    pub fn new<U>(number: U) -> Self
    where
        u8: TryFrom<U>,
    {
        Class::try_from(u8::try_from(number).unwrap_or_default()).unwrap_or_default()
    }
}

#[derive(Default, Debug, Eq, PartialEq, Deserialize, Clone)]
#[serde(from = "HashMap<Class, u8>")]
pub struct ClassList([u8; 10]);

impl Index<Class> for ClassList {
    type Output = u8;

    #[cfg_attr(feature = "no-panic", no_panic::no_panic)]
    fn index(&self, class: Class) -> &Self::Output {
        &self.0[class as u8 as usize]
    }
}

impl IndexMut<Class> for ClassList {
    #[cfg_attr(feature = "no-panic", no_panic::no_panic)]
    fn index_mut(&mut self, class: Class) -> &mut Self::Output {
        &mut self.0[class as u8 as usize]
    }
}

impl Serialize for ClassList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let count = self.0.iter().filter(|c| **c > 0).count();
        let mut classes = serializer.serialize_map(Some(count))?;
        for (class, count) in self.0.iter().copied().enumerate() {
            if count > 0 {
                classes.serialize_entry(&class, &count)?;
            }
        }

        classes.end()
    }
}

impl From<HashMap<Class, u8>> for ClassList {
    fn from(map: HashMap<Class, u8>) -> Self {
        let mut classes = ClassList::default();

        for (class, count) in map.into_iter() {
            classes[class] = count;
        }

        classes
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Spawn {
    pub user: UserId,
    pub class: Class,
    pub team: Team,
    pub tick: DemoTick,
}

impl Spawn {
    pub fn from_event(event: &PlayerSpawnEvent, tick: DemoTick) -> Self {
        Spawn {
            user: UserId::from(event.user_id),
            class: Class::new(event.class),
            team: Team::new(event.team),
            tick,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub classes: ClassList,
    pub name: String,
    pub user_id: UserId,
    pub steam_id: String,
    pub steam_id64: String,
    #[serde(skip)]
    pub entity_id: EntityId,
    pub team: Team,
}

impl From<tf_demo_parser::demo::data::UserInfo> for UserInfo {
    fn from(info: tf_demo_parser::demo::data::UserInfo) -> Self {
        let mut steam_id64 = 0;

        if info.player_info.steam_id != "BOT" {
            steam_id64 = u64::from(SteamID::from_steam3(&info.player_info.steam_id).unwrap());
        }

        UserInfo {
            classes: ClassList::default(),
            name: info.player_info.name,
            user_id: info.player_info.user_id,
            steam_id64: steam_id64.to_string(),
            steam_id: info.player_info.steam_id,
            entity_id: info.entity_id,
            team: Team::default(),
        }
    }
}

impl PartialEq for UserInfo {
    fn eq(&self, other: &UserInfo) -> bool {
        self.classes == other.classes
            && self.name == other.name
            && self.user_id == other.user_id
            && self.steam_id == other.steam_id
            && self.team == other.team
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Death {
    pub weapon: String,
    pub victim: UserId,
    pub assister: Option<UserId>,
    pub killer: UserId,
    pub tick: DemoTick,
    pub crit_type: u16,
    pub penetration: bool,
    pub killer_class: Class,
    pub victim_class: Class,
    pub is_airborne: bool,
}

impl Death {
    pub fn from_event(event: &PlayerDeathEvent, tick: DemoTick) -> Self {
        let assister = if event.assister < 16 * 1024 {
            Some(UserId::from(event.assister))
        } else {
            None
        };

        let penetration = if event.player_penetrate_count > 0 {
            true
        } else {
            false
        };

        let is_airborne = event.rocket_jump;

        Death {
            assister,
            tick,
            killer: UserId::from(event.attacker),
            weapon: event.weapon.to_string(),
            victim: UserId::from(event.user_id),
            crit_type: event.crit_type,
            penetration,
            killer_class: Class::Other,
            victim_class: Class::Other,
            is_airborne,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Round {
    pub winner: Team,
    pub length: f32,
    pub start_tick: DemoTick,
    pub end_tick: DemoTick,
    pub is_pregame: bool,
}

impl Round {
    pub fn start(tick: DemoTick) -> Self {
        Round {
            winner: Team::new(0),
            length: 0.0,
            start_tick: tick,
            end_tick: DemoTick::from(0),
            is_pregame: false,
        }
    }

    pub fn end(&mut self, event: &TeamPlayRoundWinEvent, tick: DemoTick) {
        self.winner = Team::new(event.team);
        self.length = event.round_time;
        self.end_tick = tick;
    }
}

// #[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
// pub struct World {
//     pub boundary_min: Vector,
//     pub boundary_max: Vector,
// }

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Analyser {
    state: MatchState,
    pause_start: Option<DemoTick>,
    user_id_map: HashMap<EntityId, UserId>,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Uber {
    pub tick: DemoTick,
    pub user_id: UserId,
    pub target_id: UserId,
}

impl Uber {
    pub fn from_event(event: &PlayerChargeDeployedEvent, tick: DemoTick) -> Self {
        Uber {
            tick,
            user_id: UserId::from(event.user_id),
            target_id: UserId::from(event.target_id),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Pause {
    from: DemoTick,
    to: DemoTick,
}

impl MessageHandler for Analyser {
    type Output = MatchState;

    fn does_handle(message_type: MessageType) -> bool {
        matches!(
            message_type,
            MessageType::GameEvent
                | MessageType::UserMessage
                | MessageType::ServerInfo
                | MessageType::NetTick
                | MessageType::SetPause
                | MessageType::PacketEntities
        )
    }

    fn handle_message(&mut self, message: &Message, tick: DemoTick, _parser_state: &ParserState) {
        self.state.end_tick = tick.into();
        match message {
            Message::NetTick(msg) => {
                if self.state.start_tick == 0 {
                    self.state.start_tick = msg.tick;
                }
            }
            Message::ServerInfo(message) => {
                self.state.interval_per_tick = message.interval_per_tick;
            }
            Message::GameEvent(message) => self.handle_event(&message.event, tick),
            Message::UserMessage(message) => self.handle_user_message(message, tick),
            Message::SetPause(message) => {
                if message.pause {
                    self.pause_start = Some(tick);
                } else {
                    let start = self.pause_start.unwrap_or_default();
                    self.state.pauses.push(Pause {
                        from: start,
                        to: tick,
                    })
                }
            }
            Message::PacketEntities(message) => self.handle_packet_entities(message, tick),
            _ => {}
        }
    }

    fn handle_string_entry(
        &mut self,
        table: &str,
        index: usize,
        entry: &StringTableEntry,
        _parser_state: &ParserState,
    ) {
        if table == "userinfo" {
            let _ = self.parse_user_info(
                index,
                entry.text.as_ref().map(|s| s.as_ref()),
                entry.extra_data.as_ref().map(|data| data.data.clone()),
            );
        }
    }

    fn into_output(self, _state: &ParserState) -> Self::Output {
        self.state
    }
}

impl BorrowMessageHandler for Analyser {
    fn borrow_output(&self, _state: &ParserState) -> &Self::Output {
        &self.state
    }
}

impl Analyser {
    pub fn new() -> Self {
        Self::default()
    }

    fn handle_user_message(&mut self, message: &UserMessage, tick: DemoTick) {
        if let UserMessage::SayText2(text_message) = message {
            if text_message.kind == ChatMessageKind::NameChange {
                if let Some(from) = text_message.from.clone() {
                    self.change_name(from.into(), text_message.plain_text());
                }
            } else {
                self.state
                    .chat
                    .push(ChatMessage::from_message(text_message, tick));
            }
        }
    }

    fn change_name(&mut self, from: String, to: String) {
        if let Some(user) = self.state.users.values_mut().find(|user| user.name == from) {
            user.name = to;
        }
    }

    fn handle_event(&mut self, event: &GameEvent, tick: DemoTick) {
        const WIN_REASON_TIME_LIMIT: u8 = 6;

        match event {
            GameEvent::PlayerDeath(event) => {
                self.state.deaths.push(Death::from_event(event, tick));
            }
            GameEvent::PlayerSpawn(event) => {
                let spawn = Spawn::from_event(event, tick);

                if let Some(user_state) = self.state.users.get_mut(&spawn.user) {
                    user_state.classes[spawn.class] += 1;
                    user_state.team = spawn.team;
                }

                self.state.spawns.push(spawn);
            }
            GameEvent::TeamPlayRoundWin(event) => {
                if event.win_reason != WIN_REASON_TIME_LIMIT {
                    if self.state.rounds.is_empty() {
                        self.state.rounds.push(Round::start(tick));
                    }

                    let last_round = self.state.rounds.last_mut().unwrap();
                    last_round.end(event, tick);

                    self.state.rounds.push(Round {
                        winner: Team::new(0),
                        length: 0.0,
                        start_tick: tick,
                        end_tick: DemoTick::from(0),
                        is_pregame: true,
                    });
                }
            }
            GameEvent::TeamPlayRoundStart(_) => {
                if self.state.rounds.is_empty() {
                    self.state.rounds.push(Round {
                        winner: Team::new(0),
                        length: 0.0,
                        start_tick: DemoTick::from(0),
                        end_tick: tick - 1,
                        is_pregame: true,
                    });
                } else {
                    let last_round = self.state.rounds.last_mut().unwrap();

                    if last_round.end_tick == DemoTick::from(0) {
                        last_round.end_tick = tick - 1;
                        last_round.is_pregame = true;
                    }
                }

                self.state.rounds.push(Round::start(tick));
            }
            GameEvent::PlayerChargeDeployed(event) => {
                self.state.ubers.push(Uber::from_event(event, tick));
            }
            _ => {}
        }
    }

    fn handle_packet_entities(&mut self, message: &PacketEntitiesMessage, tick: DemoTick) {
        let mut deaths_this_tick: Vec<u32> = vec![];
        let mut deaths_index: Vec<usize> = vec![];
        let deaths = &mut self.state.deaths;

        for (i, death) in deaths.iter().enumerate().rev() {
            if death.tick != tick {
                break;
            }

            deaths_this_tick.push(death.victim.into());
            deaths_index.push(i);
        }

        for entity in message.entities.iter() {
            if !self.state.id_map.contains_key(&entity.entity_index) {
                continue;
            }

            let user_id: u32 = self
                .state
                .id_map
                .get(&entity.entity_index)
                .unwrap()
                .to_owned()
                .into();

            if !deaths_this_tick.contains(&user_id) {
                continue;
            }

            for prop in entity.props.iter() {
                if prop.identifier.prop_name().is_none() {
                    continue;
                }

                let propname = prop.identifier.prop_name().unwrap().to_string();

                if propname != "m_flFallVelocity" && propname != "m_hGroundEntity" {
                    continue;
                }

                let user = self
                    .state
                    .users
                    .get(self.state.id_map.get(&entity.entity_index).unwrap())
                    .unwrap();

                let user_id: u32 = user.user_id.into();

                let death = &mut deaths
                    [deaths_index[deaths_this_tick.iter().position(|&x| x == user_id).unwrap()]];

                match propname.as_str() {
                    "m_flFallVelocity" => {
                        death.is_airborne = true;
                    }
                    "m_hGroundEntity" => {
                        death.is_airborne = false;
                    }
                    _ => continue,
                }

                break;
            }
        }
    }

    fn parse_user_info(
        &mut self,
        index: usize,
        text: Option<&str>,
        data: Option<Stream>,
    ) -> ReadResult<()> {
        if let Some(user_info) =
            tf_demo_parser::demo::data::UserInfo::parse_from_string_table(index as u16, text, data)?
        {
            let user_id = user_info.player_info.user_id;
            let entity_id = user_info.entity_id;

            self.state
                .users
                .entry(user_info.player_info.user_id)
                .and_modify(|info| {
                    info.entity_id = user_info.entity_id;
                })
                .or_insert_with(|| user_info.into());

            self.state.id_map.insert(entity_id, user_id);
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchState {
    pub users: BTreeMap<UserId, UserInfo>,
    pub user_name_map: BTreeMap<String, u32>,
    pub chat: Vec<ChatMessage>,
    pub deaths: Vec<Death>,
    pub spawns: Vec<Spawn>,
    pub rounds: Vec<Round>,
    pub end_tick: u32,
    pub start_tick: ServerTick,
    pub interval_per_tick: f32,
    pub pauses: Vec<Pause>,
    pub ubers: Vec<Uber>,
    id_map: HashMap<EntityId, UserId>,
}
