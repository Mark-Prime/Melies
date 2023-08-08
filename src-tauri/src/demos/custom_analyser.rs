use bitbuffer::{ BitRead, BitWrite, BitWriteStream, Endianness };
use num_enum::TryFromPrimitive;
use parse_display::{ Display, FromStr };
use serde::de::Error;
use serde::{ ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer };
use std::cmp::Ordering;
use std::collections::{ BTreeMap, HashMap };
use std::convert::TryFrom;
use std::ops::{ Add, Index, IndexMut, Sub };
use steamid_ng::SteamID;
use tf_demo_parser::demo::gameevent_gen::{
    GameEvent,
    PlayerDeathEvent,
    PlayerSpawnEvent,
    TeamPlayRoundWinEvent,
};
use tf_demo_parser::demo::message::packetentities::EntityId;
use tf_demo_parser::demo::message::usermessage::SayText2Message;
use tf_demo_parser::demo::message::{ Message, MessageType };
use tf_demo_parser::demo::packet::stringtable::StringTableEntry;
use tf_demo_parser::demo::parser::handler::{ BorrowMessageHandler, MessageHandler };
use tf_demo_parser::demo::vector::Vector;
use tf_demo_parser::{ ParserState, ReadResult, Stream };

/// Tick relative to the start of the game on the server
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug,
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    BitRead,
    BitWrite,
    Serialize,
    Deserialize,
    Default
)]
pub struct ServerTick(pub u32);

impl ServerTick {
    pub fn range_inclusive(&self, till: Self) -> impl Iterator<Item = Self> {
        (self.0..=till.0).into_iter().map(Self::from)
    }
}

impl PartialEq<u32> for ServerTick {
    fn eq(&self, other: &u32) -> bool {
        *other == self.0
    }
}

impl PartialOrd<u32> for ServerTick {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        other.partial_cmp(&self.0)
    }
}

impl PartialEq<ServerTick> for u32 {
    fn eq(&self, other: &ServerTick) -> bool {
        self.eq(&other.0)
    }
}

impl PartialOrd<ServerTick> for u32 {
    fn partial_cmp(&self, other: &ServerTick) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl From<u32> for ServerTick {
    fn from(tick: u32) -> Self {
        ServerTick(tick)
    }
}

impl From<ServerTick> for u32 {
    fn from(tick: ServerTick) -> Self {
        tick.0
    }
}

impl Add<u32> for ServerTick {
    type Output = ServerTick;

    fn add(self, rhs: u32) -> Self::Output {
        ServerTick(self.0 + rhs)
    }
}

impl Add<ServerTick> for ServerTick {
    type Output = ServerTick;

    fn add(self, rhs: ServerTick) -> Self::Output {
        ServerTick(self.0 + rhs.0)
    }
}

impl Sub<u32> for ServerTick {
    type Output = ServerTick;

    fn sub(self, rhs: u32) -> Self::Output {
        ServerTick(self.0 - rhs)
    }
}

impl Sub<ServerTick> for ServerTick {
    type Output = ServerTick;

    fn sub(self, rhs: ServerTick) -> Self::Output {
        ServerTick(self.0 - rhs.0)
    }
}

/// Tick relative to the start of the demo
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug,
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    BitRead,
    BitWrite,
    Serialize,
    Deserialize,
    Default
)]
pub struct DemoTick(pub u32);

impl DemoTick {
    pub fn range_inclusive(&self, till: Self) -> impl Iterator<Item = Self> {
        (self.0..=till.0).into_iter().map(Self::from)
    }

    pub fn as_i64(&self) -> i64 {
        self.0.into()
    }
}

impl PartialEq<u32> for DemoTick {
    fn eq(&self, other: &u32) -> bool {
        *other == self.0
    }
}

impl PartialOrd<u32> for DemoTick {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        other.partial_cmp(&self.0)
    }
}

impl PartialEq<DemoTick> for u32 {
    fn eq(&self, other: &DemoTick) -> bool {
        self.eq(&other.0)
    }
}

impl PartialOrd<DemoTick> for u32 {
    fn partial_cmp(&self, other: &DemoTick) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl From<u32> for DemoTick {
    fn from(tick: u32) -> Self {
        DemoTick(tick)
    }
}

impl From<DemoTick> for u32 {
    fn from(tick: DemoTick) -> Self {
        tick.0
    }
}

impl Add<u32> for DemoTick {
    type Output = DemoTick;

    fn add(self, rhs: u32) -> Self::Output {
        DemoTick(self.0 + rhs)
    }
}

impl Add<DemoTick> for DemoTick {
    type Output = DemoTick;

    fn add(self, rhs: DemoTick) -> Self::Output {
        DemoTick(self.0 + rhs.0)
    }
}

impl Sub<u32> for DemoTick {
    type Output = DemoTick;

    fn sub(self, rhs: u32) -> Self::Output {
        DemoTick(self.0 - rhs)
    }
}

impl Sub<DemoTick> for DemoTick {
    type Output = DemoTick;

    fn sub(self, rhs: DemoTick) -> Self::Output {
        DemoTick(self.0 - rhs.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum Team {
    Other = 0,
    Spectator = 1,
    Red = 2,
    Blue = 3,
}

impl Team {
    pub fn new<U>(number: U) -> Self where u8: TryFrom<U> {
        Team::try_from(u8::try_from(number).unwrap_or_default()).unwrap_or_default()
    }
}

impl Default for Team {
    fn default() -> Self {
        Team::Other
    }
}

#[derive(Debug, Clone, Serialize, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Display, FromStr)]
#[display(style = "lowercase")]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum Class {
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
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
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
                Class::try_from_primitive(class.parse().map_err(D::Error::custom)?).map_err(
                    D::Error::custom
                )
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
    pub fn new<U>(number: U) -> Self where u8: TryFrom<U> {
        Class::try_from(u8::try_from(number).unwrap_or_default()).unwrap_or_default()
    }
}

impl Default for Class {
    fn default() -> Self {
        Class::Other
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
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let count = self.0
            .iter()
            .filter(|c| **c > 0)
            .count();
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

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct UserId(pub u16);

impl<E: Endianness> BitWrite<E> for UserId {
    fn write(&self, stream: &mut BitWriteStream<E>) -> ReadResult<()> {
        (self.0 as u32).write(stream)
    }
}

impl From<u32> for UserId {
    fn from(int: u32) -> Self {
        UserId(int as u16)
    }
}

impl From<u16> for UserId {
    fn from(int: u16) -> Self {
        UserId(int)
    }
}

impl From<UserId> for u16 {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl From<UserId> for u32 {
    fn from(id: UserId) -> Self {
        id.0 as u32
    }
}

impl PartialEq<u16> for UserId {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl UserId {
    pub fn to_u32(&self) -> u32 {
        return self.0 as u32;
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
            user_id: info.player_info.user_id.into(),
            steam_id64: steam_id64.to_string(),
            steam_id: info.player_info.steam_id,
            entity_id: info.entity_id,
            team: Team::default(),
        }
    }
}

impl PartialEq for UserInfo {
    fn eq(&self, other: &UserInfo) -> bool {
        self.classes == other.classes &&
            self.name == other.name &&
            self.user_id == other.user_id &&
            self.steam_id == other.steam_id &&
            self.team == other.team
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
    pub rocket_jump: bool,
    pub penetration: bool,
    pub killer_class: Class,
    pub victim_class: Class,
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

        Death {
            assister,
            tick,
            killer: UserId::from(event.attacker),
            weapon: event.weapon.to_string(),
            victim: UserId::from(event.user_id),
            crit_type: event.crit_type,
            rocket_jump: event.rocket_jump,
            penetration,
            killer_class: Class::Other,
            victim_class: Class::Other
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Round {
    pub winner: Team,
    pub length: f32,
    pub end_tick: DemoTick,
}

impl Round {
    pub fn from_event(event: &TeamPlayRoundWinEvent, tick: DemoTick) -> Self {
        Round {
            winner: Team::new(event.team),
            length: event.round_time,
            end_tick: tick,
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct World {
    pub boundary_min: Vector,
    pub boundary_max: Vector,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Analyser {
    state: MatchState,
    user_id_map: HashMap<EntityId, UserId>,
}

impl MessageHandler for Analyser {
    type Output = MatchState;

    fn does_handle(message_type: MessageType) -> bool {
        matches!(
            message_type,
            MessageType::GameEvent |
                MessageType::UserMessage |
                MessageType::ServerInfo |
                MessageType::NetTick
        )
    }

    fn handle_message(&mut self, message: &Message, tick: u32) {
        self.state.end_tick = ServerTick(tick);
        if tick > 0 && self.state.start_tick.0 == 0 {
            self.state.start_tick = ServerTick(tick);
        }
        match message {
            Message::ServerInfo(message) => {
                self.state.interval_per_tick = message.interval_per_tick;
            }
            Message::GameEvent(message) => self.handle_event(&message.event, DemoTick(tick)),
            Message::UserMessage(message) =>
                match message {
                    tf_demo_parser::demo::message::usermessage::UserMessage::SayText2(text) => {
                        let from = text.clone().from.clone();
                        let message_text: String = text.clone().text;
                        if from.is_some() {
                            let name = text.from.clone().unwrap();
                            if self.state.user_name_map.contains_key(&from.clone().unwrap()) {
                                self.state.chat.push(ChatMessage {
                                    from: self.state.user_name_map[&from.unwrap()].into(),
                                    name,
                                    text: message_text,
                                    tick: tick.into(),
                                    message: *text.to_owned(),
                                });
                            }
                        }
                    }
                    _ => {}
                }
            _ => {}
        }
    }

    fn handle_string_entry(&mut self, table: &str, _index: usize, entry: &StringTableEntry) {
        if table == "userinfo" {
            let _ = self.parse_user_info(
                entry.text.as_ref().map(|s| s.as_ref()),
                entry.extra_data.as_ref().map(|data| data.data.clone())
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

    // fn change_name(&mut self, from: String, to: String) {
    //     if let Some(user) = self.state.users.values_mut().find(|user| user.name == from) {
    //         user.name = to;
    //     }
    // }

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
                    self.state.rounds.push(Round::from_event(event, tick))
                }
            }
            _ => {}
        }
    }

    fn parse_user_info(&mut self, text: Option<&str>, data: Option<Stream>) -> ReadResult<()> {
        if
            let Some(user_info) = tf_demo_parser::demo::data::UserInfo::parse_from_string_table(
                text,
                data
            )?
        {
            let player_info = user_info.player_info.clone();

            self.state.user_name_map
                .entry(player_info.name.into())
                .and_modify(|info| {
                    *info = player_info.user_id;
                })
                .or_insert_with(|| player_info.user_id.into());

            self.state.users
                .entry(user_info.player_info.user_id.into())
                .and_modify(|info| {
                    info.entity_id = user_info.entity_id;
                })
                .or_insert_with(|| user_info.into());
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
    pub end_tick: ServerTick,
    pub start_tick: ServerTick,
    pub interval_per_tick: f32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    from: u32,
    tick: DemoTick,
    name: String,
    text: String,
    message: SayText2Message,
}
