//! Where events play and how they start, from the game's own tables (assets/events/vol1_event_locations.json,
//! exported from hackGU_vol1.dll by tools/event_locations.py; the code path is documented there).
//!
//! Town events play at the stage locator `DMY_ev<locator>` (the DLL formats "DMY_ev%d" with the location row's
//! first field). Start condition 75 = started by the story; 79 = proximity trigger with a radius in cm.

use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Clone, Debug, Deserialize)]
pub struct EventLoc {
    pub event: String,
    pub location: u32,
    pub locator: Option<i32>,
    pub place_type: Option<i32>,
    /// Area Words of the location row (indices into the three word tables; 0,0,0 for towns and the tutorial).
    #[serde(default)]
    pub area_words: Option<[i32; 3]>,
    pub start: Option<i32>,
    pub trigger_radius_cm: Option<i32>,
}

#[derive(Deserialize)]
struct File {
    events: Vec<EventLoc>,
}

pub fn table() -> &'static [EventLoc] {
    static T: OnceLock<Vec<EventLoc>> = OnceLock::new();
    T.get_or_init(|| {
        std::fs::read_to_string("assets/events/vol1_event_locations.json")
            .ok()
            .and_then(|s| serde_json::from_str::<File>(&s).map_err(|e| eprintln!("event locations: {}", e)).ok())
            .map(|f| f.events)
            .unwrap_or_default()
    })
}

pub fn find(id: &str) -> Option<&'static EventLoc> {
    table().iter().find(|e| e.event == id)
}

/// Place type 1 = town.
pub const PLACE_TOWN: i32 = 1;
/// Place type 4 = dungeon.
pub const PLACE_DUNGEON: i32 = 4;

/// The next code-started (start condition 80) dungeon event of the Area with these words: the first one in table order
/// after the Area's arrival event that has not been played (events without a start condition are sub-events and are
/// skipped). Only one is armed at a time.
pub fn next_dungeon_event(words: [usize; 3], done: &[String]) -> Option<&'static EventLoc> {
    let w = words.map(|x| x as i32);
    let chest = chest_event(words, done).map(|e| e.event.as_str());
    table().iter().filter(|e| e.area_words == Some(w) && e.place_type == Some(PLACE_DUNGEON) && e.start.is_some()).find(|e| !done.iter().any(|d| d == &e.event)).filter(|e| e.start == Some(80) && Some(e.event.as_str()) != chest)
}

/// A dungeon event started by opening the last room's offering chest: the code-started (80) event that follows an
/// event without a start condition (PM10350, the chest itself, has no script) in the Area's table. GUESS (not
/// traced): PM10355 (the PK ambush) plays when the chest is opened, as Gaspard's "Go on and open it" suggests.
pub fn chest_event(words: [usize; 3], done: &[String]) -> Option<&'static EventLoc> {
    let w = words.map(|x| x as i32);
    let t = table();
    let played = |e: &EventLoc| done.iter().any(|d| d == &e.event);
    // PM10338/PM10339 (sub-events of the Item Box tutorial) also lack a start condition; the chest is the one whose
    // following code-started event has not been played yet.
    (0..t.len())
        .filter(|&i| t[i].area_words == Some(w) && t[i].place_type == Some(PLACE_DUNGEON) && t[i].start.is_none() && !played(&t[i]))
        .find_map(|i| t[i + 1..].iter().find(|e| e.area_words == Some(w) && e.start == Some(80)).filter(|e| !played(e)))
}

/// The story-started event that directly follows a dungeon event in the table (PM10355 -> PM10360).
pub fn dungeon_follow_up(after: &str, done: &[String]) -> Option<&'static EventLoc> {
    let t = table();
    let i = t.iter().position(|e| e.event == after)?;
    t.get(i + 1).filter(|e| e.start == Some(75) && e.area_words == t[i].area_words && !done.iter().any(|d| d == &e.event))
}

/// The story-started event that plays on arriving in the Area with these words (first such event in table order not
/// yet played), e.g. PM10330 for Peaceful Leading Freedom.
pub fn arrival_event(words: [usize; 3], done: &[String]) -> Option<&'static EventLoc> {
    if words == [0, 0, 0] {
        return None;
    }
    table().iter().find(|e| e.start == Some(75) && e.area_words == Some(words.map(|w| w as i32)) && !done.iter().any(|d| d == &e.event))
}

/// Proximity-triggered town events that are open now: the start-79 events that follow the latest story-started event
/// the player has seen, in table order, minus those already played.
pub fn armed_town_events(done: &[String]) -> Vec<&'static EventLoc> {
    let t = table();
    let last_story = t.iter().rposition(|e| e.start == Some(75) && done.iter().any(|d| d == &e.event));
    let Some(i) = last_story else { return Vec::new() };
    t[i + 1..]
        .iter()
        .take_while(|e| e.start == Some(79))
        .filter(|e| e.place_type == Some(PLACE_TOWN) && !done.iter().any(|d| d == &e.event))
        .collect()
}

/// The story-started event that follows a just-played proximity event (PM10315 -> PM10320).
pub fn follow_up(after: &str, done: &[String]) -> Option<&'static EventLoc> {
    let t = table();
    let i = t.iter().position(|e| e.event == after)?;
    if t[i].start != Some(79) || !after.starts_with("PM") {
        return None;
    }
    t[i + 1..].iter().find(|e| e.start == Some(75) && e.event.starts_with("PM")).filter(|e| !done.iter().any(|d| d == &e.event) && e.place_type == Some(PLACE_TOWN))
}

/// Area Words the story hands over: (event after which they are owned, words as indices into the three word tables).
/// PM10320: (game text) (messageInfo 1E0E6590) and the
/// objective (game text)
/// (ADB559BF). NOT TRACED: where the DLL adds them (no script op in PM10320 does).
pub const GIVEN_WORDS: [(&str, [usize; 3]); 1] = [("PM10320", [11, 27, 21])];

/// Words owned through played events, per slot.
pub fn given_words(done: &[String]) -> [Vec<usize>; 3] {
    let mut out: [Vec<usize>; 3] = Default::default();
    for (ev, w) in GIVEN_WORDS {
        if done.iter().any(|d| d == ev) {
            for c in 0..3 {
                out[c].push(w[c]);
            }
        }
    }
    out
}
