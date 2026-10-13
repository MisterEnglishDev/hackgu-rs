//! Playing Δ Courageous Engaging Daydream: story triggers, the altar fights, followers and the battle HUD.
//!
//! From data: models and animations (Haseo's twin-blade set `spc/2hsw20.ccs`: nut10/run10/atn10-40/dma10/fat10;
//! Goblin Rookie = `en/egg10.ccs` body with `egg10swd00.ccs` in its `prop1` hand bone, motions egg10 nut10/run10/
//! dma10/dmg20 and `cmn en/egg21.ccs` egg20atn10 (attack), egg20ght00 (stagger)); the event scripts and their order.
//! From the original's footage (reference_captures/run2_field): party HP/SP (Haseo 50/20, Asta 259/47, Iyoten
//! 271/48), Goblin Rookie max HP 102 (shown on the target box before the first hit, f00606), twin-blade hits of 2-6, three goblins per
//! altar, the "Outbreak of Battle" banner, combo counter, "Battle Result" with 3 GP.
//! Not from data (approximations, to be replaced when the battle code is traced): enemy sight cone/range,
//! movement speeds, attack timing, damage formulas, the Rengeki window, party AI.

use std::collections::HashMap;
use std::sync::Arc;

use glam::{Vec2, Vec3};
use hackgu_ccs::{CcsAnimation, CcsFile, Mat4};

use crate::cutscene::{load_ccs, EvTransform};
use crate::mesh::GpuScene;

/// messageInfo 0xE2C5DA3E: the "fragments joined" notice, read from the player's game files.
fn joined_text() -> String {
    static DB: std::sync::OnceLock<Option<hackgu_game::msginfo::MessageInfo>> = std::sync::OnceLock::new();
    DB.get_or_init(|| hackgu_game::msginfo::MessageInfo::load(std::path::Path::new("target"), &crate::auto::get().lang))
        .as_ref()
        .and_then(|m| m.by_id.get(&0xE2C5_DA3E).cloned())
        .unwrap_or_default()
}

/// Facing for a world yaw (0 faces -Y, counter-clockwise positive).
pub fn forward(yaw: f32) -> Vec2 {
    Vec2::new(yaw.sin(), -yaw.cos())
}
pub fn yaw_to(d: Vec2) -> f32 {
    d.x.atan2(-d.y)
}

/// Altar-local position of the altar centre in PM10120/PM10125 (the goblins mill about 2-6 m ahead of the event
/// origin on the plinth; the party starts ~23 m away on +y).
const EVENT_ALTAR_CENTRE: Vec2 = Vec2::new(0.0, 4.0);
/// Distance from an altar at which its event (or fight) begins.
const ALTAR_TRIGGER: f32 = 22.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Story {
    /// Before the first altar.
    Explore,
    /// Altar k's event has played; its goblins are up.
    AltarFight(usize),
    /// All three altars cleared and the fragments joined; PM10135 next.
    FragmentsJoined,
    /// Heading for the Beast Temple.
    ToTemple,
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum EnemyState {
    Idle,
    Chase,
    Attack,
    Hurt,
    Stagger,
    Down,
    Gone,
}

struct Enemy {
    actor: usize,
    sword: Option<usize>,
    altar: usize,
    pos: Vec3,
    yaw: f32,
    hp: i32,
    max_hp: i32,
    state: EnemyState,
    t: f32,
    anim: usize,
    anim_t: f32,
    anim_loop: bool,
    cooldown: f32,
    hits_taken: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlayerAct {
    Free,
    /// Combo step 0..3 (atn10..atn40), time in the swing.
    Attack(usize, f32),
    Hurt(f32),
    Rengeki(f32),
}

struct Popup {
    pos: Vec3,
    text: String,
    t: f32,
    color: [u8; 3],
}

struct Follower {
    actor: usize,
    ccs: Arc<CcsFile>,
    anims: Vec<CcsAnimation>,
    pos: Vec3,
    yaw: f32,
    anim: usize,
    t: f32,
    slot: Vec2,
    hp: i32,
    max_hp: i32,
    sp: i32,
    attack_cd: f32,
}

pub struct Results {
    pub gp: i32,
}

/// What the app should do after an update.
pub enum FieldEvent {
    StartScript(String, Option<EvTransform>),
    Message(String),
    /// Altar chest k opened (play its opening animation), with the message shown.
    ChestOpened(usize, String),
}

pub struct FieldPlay {
    pub story: Story,
    altars_cleared: Vec<bool>,
    chests_opened: Vec<bool>,
    event_played: Vec<bool>,
    enemies: Vec<Enemy>,
    gob_ccs: Arc<CcsFile>,
    gob_anims: HashMap<String, CcsAnimation>,
    sword_ccs: Option<Arc<CcsFile>>,
    /// Haseo's twin-blade animations, appended to the player scene: name -> index.
    hsw: HashMap<String, usize>,
    act: PlayerAct,
    queued_attack: bool,
    pub hp: i32,
    pub max_hp: i32,
    pub sp: i32,
    combo: u32,
    combo_t: f32,
    popups: Vec<Popup>,
    followers: Vec<Follower>,
    pub in_battle: bool,
    banner_t: f32,
    target: Option<usize>,
    pub results: Option<Results>,
    gp: i32,
    fragments: u32,
    message: Option<(String, f32)>,
    rengeki_window: f32,
    followers_shown: bool,
    /// Footstep trail for the minimap (world positions, every 3 m).
    trail: Vec<Vec3>,
    /// mapicon atlas (field/mapmenu@ENG.ccs TEX_mapicon) for the minimap markers.
    pub map_icons: Option<egui::TextureHandle>,
}

fn find_anim(list: &[CcsAnimation], suffix: &str) -> Option<usize> {
    list.iter().position(|a| a.name.ends_with(suffix))
}

impl FieldPlay {
    /// Spawn goblins, followers and the battle animation sets into the field scene.
    pub fn new(scene: &mut GpuScene, device: &wgpu::Device, queue: &wgpu::Queue, layout: &wgpu::BindGroupLayout, char_ccs: &CcsFile) -> Option<Self> {
        let area = crate::field::area();
        let gob = Arc::new(load_ccs(&area.enemy.model)?);
        // Animations keyed by their action suffix (last 5 characters: nut10, run10, atn10, dma10, ght00, ...).
        let key = |n: &str| n.get(n.len().saturating_sub(5)..).unwrap_or(n).to_string();
        let mut gob_anims: HashMap<String, CcsAnimation> = HashMap::new();
        if let Some(extra) = area.enemy.extra_anims.as_ref().and_then(|f| load_ccs(f)) {
            for a in extra.animations() {
                gob_anims.insert(key(&a.name), a);
            }
        }
        for a in gob.animations() {
            gob_anims.insert(key(&a.name), a);
        }
        let sword_ccs = area.enemy.weapon.as_ref().and_then(|f| load_ccs(f)).map(Arc::new);
        // Haseo's twin-blade battle set rides on the 2hsw01 skeleton.
        let mut hsw = HashMap::new();
        if let Some(tb) = load_ccs("2hsw20.ccs") {
            for a in tb.animations() {
                let key = a.name.trim_start_matches("ANM_2hsw20").to_string();
                hsw.insert(key, scene.animations.len());
                scene.animations.push(a);
            }
        }
        let _ = char_ccs;
        let mut fp = FieldPlay {
            story: Story::Explore,
            altars_cleared: vec![false; area.altars.len()],
            chests_opened: vec![false; area.altars.len()],
            event_played: vec![false; area.altars.len()],
            enemies: Vec::new(),
            gob_ccs: gob,
            gob_anims,
            sword_ccs,
            hsw,
            act: PlayerAct::Free,
            queued_attack: false,
            hp: 50,
            max_hp: 50,
            sp: 20,
            combo: 0,
            combo_t: 0.0,
            popups: Vec::new(),
            followers: Vec::new(),
            in_battle: false,
            banner_t: 0.0,
            target: None,
            results: None,
            gp: 0,
            fragments: 0,
            message: None,
            rengeki_window: 0.0,
            followers_shown: false,
            trail: Vec::new(),
            map_icons: None,
        };
        // Goblins: three on every altar.
        for k in 0..area.altars.len() {
            let c = area.altar_world(k);
            for n in 0..area.enemy.per_altar {
                let a = n as f32 * std::f32::consts::TAU / 3.0 + 0.5;
                let pos = c + Vec3::new(a.cos(), a.sin(), 0.0) * 3.5;
                fp.spawn_goblin(scene, device, queue, layout, k, pos, yaw_to((c - pos).truncate()));
            }
        }
        // Followers: Asta (HP 259 / SP 47) and Iyoten (HP 271 / SP 48), as in the original's party HUD.
        let slots = [Vec2::new(-2.2, -0.6), Vec2::new(2.2, -0.6)];
        for (pm, &slot) in area.party.iter().zip(slots.iter()) {
            let (hp, sp) = (pm.hp, pm.sp);
            if let Some(c) = load_ccs(&pm.model) {
                scene.spawn_actor_model(device, queue, layout, &c, Vec3::ZERO, 0.0);
                let actor = scene.spawned_actors.len() - 1;
                scene.set_actor_visible(actor, false);
                let anims = c.animations();
                fp.followers.push(Follower { actor, ccs: Arc::new(c), anims, pos: Vec3::ZERO, yaw: 0.0, anim: 0, t: 0.0, slot, hp, max_hp: hp, sp, attack_cd: 0.0 });
            }
        }
        // Debug: HACKGU_STORY=chests starts with all three altars cleared (chests still closed).
        if std::env::var("HACKGU_STORY").as_deref() == Ok("chests") {
            fp.altars_cleared = vec![true; fp.altars_cleared.len()];
            fp.event_played = vec![true; fp.event_played.len()];
            // HACKGU_FRAGMENTS=n: the first n chests are already open.
            let n: usize = std::env::var("HACKGU_FRAGMENTS").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
            for k in 0..n.min(fp.chests_opened.len()) {
                fp.chests_opened[k] = true;
            }
            fp.fragments = n.min(fp.chests_opened.len()) as u32;
            for e in &mut fp.enemies {
                e.state = EnemyState::Gone;
                Self::set_goblin_visible(scene, e, false);
            }
        }
        // Debug: HACKGU_STORY=temple starts with the Symbol Fragments joined (temple open).
        if std::env::var("HACKGU_STORY").as_deref() == Ok("temple") {
            fp.story = Story::ToTemple;
            fp.altars_cleared = vec![true; fp.altars_cleared.len()];
            fp.event_played = vec![true; fp.event_played.len()];
        }
        Some(fp)
    }

    fn spawn_goblin(&mut self, scene: &mut GpuScene, device: &wgpu::Device, queue: &wgpu::Queue, layout: &wgpu::BindGroupLayout, altar: usize, pos: Vec3, yaw: f32) {
        scene.spawn_actor_model(device, queue, layout, &self.gob_ccs, pos, yaw);
        let actor = scene.spawned_actors.len() - 1;
        let wclump = crate::field::area().enemy.weapon_clump.clone().unwrap_or_default();
        let sword = self.sword_ccs.as_ref().and_then(|s| scene.spawn_prop(device, queue, layout, s, &wclump));
        // The sword prop spawns at the world origin (beside the arrival platform) and is only moved by the pose update,
        // which does not run during events: keep it hidden until its goblin has placed it once.
        if let Some(s) = sword {
            for (_, _, subs) in &scene.props[s].models {
                for &si in subs {
                    scene.submeshes[si].visible = false;
                }
            }
        }
        let idle = self.gob_anim_index("nut10");
        self.enemies.push(Enemy { actor, sword, altar, pos, yaw, hp: crate::field::area().enemy.hp, max_hp: crate::field::area().enemy.hp, state: EnemyState::Idle, t: 0.0, anim: idle, anim_t: 0.0, anim_loop: true, cooldown: 0.0, hits_taken: 0 });
    }

    fn gob_anim_index(&self, key: &str) -> usize {
        // Stable index: position in a sorted key list.
        let mut keys: Vec<&String> = self.gob_anims.keys().collect();
        keys.sort();
        keys.iter().position(|k| k.as_str() == key).unwrap_or(0)
    }
    fn gob_anim(&self, idx: usize) -> Option<&CcsAnimation> {
        let mut keys: Vec<&String> = self.gob_anims.keys().collect();
        keys.sort();
        keys.get(idx).and_then(|k| self.gob_anims.get(*k))
    }

    /// The app hands over player control (movement and locomotion animation) while this is true.
    pub fn locks_player(&self) -> bool {
        !matches!(self.act, PlayerAct::Free) || self.results.is_some()
    }
    /// Battle stance animations replace the field idle/run while fighting.
    pub fn drives_player_anim(&self) -> bool {
        self.in_battle || !matches!(self.act, PlayerAct::Free)
    }

    /// The event frame that puts an altar event's party on the player's side of altar `k`.
    pub fn altar_event_frame(k: usize, player: Vec3) -> EvTransform {
        let c = crate::field::area().altar_world(k);
        let d = (player - c).truncate().normalize_or(Vec2::Y);
        // Script +y (towards the party) points at the player.
        let yaw = (-d.x).atan2(d.y);
        let (s, co) = yaw.sin_cos();
        let off = Vec2::new(co * EVENT_ALTAR_CENTRE.x - s * EVENT_ALTAR_CENTRE.y, s * EVENT_ALTAR_CENTRE.x + co * EVENT_ALTAR_CENTRE.y);
        EvTransform { origin: Vec3::new(c.x - off.x, c.y - off.y, 0.0), yaw }
    }

    /// "A" pressed during free play.
    pub fn press_a(&mut self, player: Vec3, player_yaw: f32) -> Option<FieldEvent> {
        if self.results.is_some() {
            self.results = None;
            return None;
        }
        if self.message.is_some() {
            self.message = None;
            return None;
        }
        if self.rengeki_window > 0.0 {
            self.rengeki_window = 0.0;
            self.act = PlayerAct::Rengeki(0.0);
            return None;
        }
        match self.act {
            PlayerAct::Attack(_, _) => {
                self.queued_attack = true;
                return None;
            }
            PlayerAct::Hurt(_) | PlayerAct::Rengeki(_) => return None,
            PlayerAct::Free => {}
        }
        // Chests on cleared altars.
        let area = crate::field::area();
        for k in 0..area.altars.len() {
            let c = area.altar_world(k);
            if (c - player).truncate().length() < 2.6 && self.altars_cleared[k] && !self.chests_opened[k] {
                self.chests_opened[k] = true;
                self.fragments += 1;
                let msg = if self.fragments as usize >= self.chests_opened.len() { joined_text() } else { "Obtained a Symbol Fragment.".to_string() };
                self.message = Some((msg.clone(), 4.0));
                if self.fragments as usize >= self.chests_opened.len() {
                    self.story = Story::FragmentsJoined;
                }
                return Some(FieldEvent::ChestOpened(k, msg));
            }
        }
        // Attack (also starts a fight: an ambush when the goblin does not see Haseo).
        let ambush = self.enemies.iter().position(|e| {
            e.state == EnemyState::Idle && (e.pos - player).truncate().length() < 2.6 && !Self::sees(e, player)
        });
        if let Some(i) = ambush {
            self.start_battle(self.enemies[i].altar);
            self.damage_enemy(i, 30, "Ambush!");
        }
        let _ = player_yaw;
        if self.in_battle || ambush.is_some() {
            self.act = PlayerAct::Attack(0, 0.0);
            self.queued_attack = false;
        }
        None
    }

    fn sees(e: &Enemy, p: Vec3) -> bool {
        let d = (p - e.pos).truncate();
        let dist = d.length();
        if dist > 9.0 {
            return false;
        }
        let f = forward(e.yaw);
        dist < 1.2 || f.dot(d / dist.max(0.001)) > 0.5 // ~60 deg either side
    }

    fn start_battle(&mut self, altar: usize) {
        if !self.in_battle {
            self.in_battle = true;
            self.banner_t = 2.0;
        }
        for e in &mut self.enemies {
            if e.altar == altar && e.state == EnemyState::Idle {
                e.state = EnemyState::Chase;
            }
        }
    }

    fn damage_enemy(&mut self, i: usize, dmg: i32, label: &str) {
        let e = &mut self.enemies[i];
        if matches!(e.state, EnemyState::Down | EnemyState::Gone) {
            return;
        }
        e.hp -= dmg;
        e.hits_taken += 1;
        self.target = Some(i);
        self.combo += 1;
        self.combo_t = 1.5;
        let p = e.pos + Vec3::new(0.0, 0.0, 1.4);
        self.popups.push(Popup { pos: p, text: if label.is_empty() { dmg.to_string() } else { format!("{} {}", label, dmg) }, t: 0.0, color: [255, 255, 255] });
        if e.hp <= 0 {
            e.hp = 0;
            e.state = EnemyState::Down;
            e.t = 0.0;
        } else if e.hits_taken % 4 == 0 {
            e.state = EnemyState::Stagger;
            e.t = 0.0;
            self.rengeki_window = 1.0;
        } else if e.state != EnemyState::Stagger {
            e.state = EnemyState::Hurt;
            e.t = 0.0;
        }
    }

    /// Per-frame update. `in_event` pauses everything but rendering.
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        dt: f32,
        scene: &mut GpuScene,
        queue: &wgpu::Queue,
        char_ccs: &CcsFile,
        player_pos: Vec3,
        player_yaw: &mut f32,
        player_moving: bool,
        col: Option<&hackgu_world::CollisionMesh>,
        in_event: bool,
    ) -> Vec<FieldEvent> {
        let mut out = Vec::new();
        let ground = |p: Vec3| -> f32 { col.and_then(|c| c.get_ground_height(p.x, p.y, p.z + 1.5)).unwrap_or(p.z) };
        self.banner_t = (self.banner_t - dt).max(0.0);
        self.combo_t -= dt;
        if self.combo_t <= 0.0 {
            self.combo = 0;
        }
        self.rengeki_window = (self.rengeki_window - dt).max(0.0);
        if let Some((_, t)) = &mut self.message {
            *t -= dt;
            if *t <= 0.0 {
                self.message = None;
            }
        }
        for p in &mut self.popups {
            p.t += dt;
            p.pos.z += dt * 0.6;
        }
        self.popups.retain(|p| p.t < 1.0);

        if !in_event && !self.followers_shown {
            self.place_followers(scene, player_pos, *player_yaw);
        }

        if self.trail.last().map(|l| (*l - player_pos).truncate().length() > 3.0).unwrap_or(true) {
            self.trail.push(player_pos);
            if self.trail.len() > 12 {
                self.trail.remove(0);
            }
        }

        // Story triggers (only in free play).
        if !in_event && self.results.is_none() {
            let area = crate::field::area();
            for k in 0..area.altars.len() {
                let c = area.altar_world(k);
                let d = (c - player_pos).truncate().length();
                if !self.event_played[k] && d < ALTAR_TRIGGER {
                    self.event_played[k] = true;
                    // An altar opens with its event when the area defines one (tutorial: altars 1 and 2);
                    // otherwise it goes straight to battle.
                    if let Some(s) = area.events.altars.get(k).cloned().flatten() {
                        out.push(FieldEvent::StartScript(s, Some(Self::altar_event_frame(k, player_pos))));
                        self.story = Story::AltarFight(k);
                        return out;
                    }
                }
            }
            // The temple door opens once the Symbol Fragments are joined; entering it loads the statue room (PM10140).
            if self.story == Story::ToTemple && area.temple.is_some() && (crate::field::temple_door_world() - player_pos).truncate().length() < 2.5 {
                self.story = Story::Done;
                if let Some(s) = area.events.statue.clone() {
                    out.push(FieldEvent::StartScript(s, None));
                    return out;
                }
            }
            if self.story == Story::FragmentsJoined && self.message.is_none() {
                self.story = Story::ToTemple;
                if let Some(s) = area.events.fragments_joined.clone() {
                    out.push(FieldEvent::StartScript(s, Some(EvTransform { origin: player_pos, yaw: *player_yaw })));
                    return out;
                }
            }
        }

        // Enemies.
        let mut attacks_on_player = 0;
        let party_targets: Vec<Vec3> = std::iter::once(player_pos).chain(self.followers.iter().map(|f| f.pos)).collect();
        for i in 0..self.enemies.len() {
            if in_event {
                break;
            }
            let (state, pos) = (self.enemies[i].state, self.enemies[i].pos);
            if state == EnemyState::Gone {
                continue;
            }
            // Noticing the player starts the fight at this altar.
            if state == EnemyState::Idle && Self::sees(&self.enemies[i], player_pos) {
                let altar = self.enemies[i].altar;
                self.start_battle(altar);
            }
            let e = &mut self.enemies[i];
            e.t += dt;
            e.cooldown = (e.cooldown - dt).max(0.0);
            let nearest = party_targets.iter().copied().min_by(|a, b| (*a - pos).length().total_cmp(&(*b - pos).length())).unwrap_or(player_pos);
            let to = (nearest - pos).truncate();
            let dist = to.length();
            match e.state {
                EnemyState::Idle => {}
                EnemyState::Chase => {
                    e.yaw = yaw_to(to);
                    if dist > 1.6 {
                        let step = to / dist * 3.2 * dt;
                        e.pos.x += step.x;
                        e.pos.y += step.y;
                    } else if e.cooldown <= 0.0 {
                        e.state = EnemyState::Attack;
                        e.t = 0.0;
                    }
                }
                EnemyState::Attack => {
                    // egg20atn10: the swing lands about halfway through (approximation).
                    if e.t >= 0.6 && e.t - dt < 0.6 && dist < 2.2 && (nearest - player_pos).length() < 0.01 {
                        attacks_on_player += 1;
                    }
                    if e.t > 1.2 {
                        e.state = EnemyState::Chase;
                        e.cooldown = 1.5 + (i as f32 * 0.37) % 1.0;
                    }
                }
                EnemyState::Hurt => {
                    if e.t > 0.45 {
                        e.state = EnemyState::Chase;
                    }
                }
                EnemyState::Stagger => {
                    if e.t > 1.4 {
                        e.state = EnemyState::Chase;
                    }
                }
                EnemyState::Down => {
                    if e.t > 1.8 {
                        e.state = EnemyState::Gone;
                        scene.set_actor_visible(e.actor, false);
                        if let Some(s) = e.sword {
                            for (_, _, subs) in &scene.props[s].models {
                                for &si in subs {
                                    scene.submeshes[si].visible = false;
                                }
                            }
                        }
                    }
                }
                EnemyState::Gone => {}
            }
            e.pos.z = ground(e.pos);
        }
        if std::env::var_os("HACKGU_TRACE_PLAYER").is_some() {
            static mut N: u32 = 0;
            unsafe {
                N += 1;
                if N % 30 == 0 {
                    for e in self.enemies.iter().filter(|e| e.altar == 2) {
                        println!("[gob] {:?} hp {} pos {:.1?}", e.state, e.hp, e.pos);
                    }
                }
            }
        }
        // Keep bodies apart (no actor collision in the data we use): goblins stay 1.3 m from Haseo and each other.
        for i in 0..self.enemies.len() {
            if matches!(self.enemies[i].state, EnemyState::Gone | EnemyState::Down) {
                continue;
            }
            let mut push = Vec2::ZERO;
            let me = self.enemies[i].pos.truncate();
            let d = me - player_pos.truncate();
            if d.length() < 1.3 {
                push += d.normalize_or(Vec2::X) * (1.3 - d.length());
            }
            for j in 0..self.enemies.len() {
                if i == j || matches!(self.enemies[j].state, EnemyState::Gone | EnemyState::Down) {
                    continue;
                }
                let d = me - self.enemies[j].pos.truncate();
                if d.length() < 1.2 {
                    push += d.normalize_or(Vec2::Y) * (1.2 - d.length()) * 0.5;
                }
            }
            self.enemies[i].pos.x += push.x;
            self.enemies[i].pos.y += push.y;
        }
        for _ in 0..attacks_on_player {
            if !matches!(self.act, PlayerAct::Hurt(_) | PlayerAct::Rengeki(_)) {
                let dmg = 3 + (self.hp as usize % 3) as i32;
                self.hp = (self.hp - dmg).max(1); // The tutorial cannot be lost here (Game Over not implemented).
                self.popups.push(Popup { pos: player_pos + Vec3::new(0.0, 0.0, 1.6), text: dmg.to_string(), t: 0.0, color: [255, 120, 120] });
                self.act = PlayerAct::Hurt(0.0);
            }
        }

        // Player actions.
        let swing_len = |k: usize| [0.45f32, 0.45, 0.5, 0.65][k.min(3)];
        match self.act {
            PlayerAct::Attack(k, t) => {
                let t2 = t + dt;
                // Face the nearest live enemy at the start of a swing.
                if t == 0.0 {
                    if let Some(e) = self.enemies.iter().filter(|e| !matches!(e.state, EnemyState::Down | EnemyState::Gone) && e.state != EnemyState::Idle || self.in_battle).filter(|e| !matches!(e.state, EnemyState::Down | EnemyState::Gone)).min_by(|a, b| (a.pos - player_pos).length().total_cmp(&(b.pos - player_pos).length())) {
                        if (e.pos - player_pos).length() < 4.0 {
                            *player_yaw = yaw_to((e.pos - player_pos).truncate());
                        }
                    }
                }
                // Twin blades strike twice per swing (2-6 each, as in the original's hit numbers).
                for hit_at in [0.35f32, 0.55] {
                    let at = swing_len(k) * hit_at;
                    if t < at && t2 >= at {
                        let f = forward(*player_yaw);
                        let hits: Vec<usize> = self
                            .enemies
                            .iter()
                            .enumerate()
                            .filter(|(_, e)| !matches!(e.state, EnemyState::Down | EnemyState::Gone))
                            .filter(|(_, e)| {
                                let d = (e.pos - player_pos).truncate();
                                d.length() < 2.6 && f.dot(d.normalize_or_zero()) > 0.3
                            })
                            .map(|(i, _)| i)
                            .collect();
                        for i in hits {
                            let dmg = 2 + ((k as i32 * 3 + i as i32 + self.combo as i32) % 5);
                            let altar = self.enemies[i].altar;
                            self.start_battle(altar);
                            self.damage_enemy(i, dmg, "");
                        }
                    }
                }
                if t2 >= swing_len(k) {
                    if self.queued_attack && k < 3 {
                        self.act = PlayerAct::Attack(k + 1, 0.0);
                        self.queued_attack = false;
                    } else {
                        self.act = PlayerAct::Free;
                    }
                } else {
                    self.act = PlayerAct::Attack(k, t2);
                }
            }
            PlayerAct::Hurt(t) => {
                self.act = if t + dt > 0.4 { PlayerAct::Free } else { PlayerAct::Hurt(t + dt) };
            }
            PlayerAct::Rengeki(t) => {
                let t2 = t + dt;
                if t < 0.4 && t2 >= 0.4 {
                    if let Some(i) = self.target {
                        self.damage_enemy(i, 20, "Rengeki!");
                    }
                }
                self.act = if t2 > 0.9 { PlayerAct::Free } else { PlayerAct::Rengeki(t2) };
            }
            PlayerAct::Free => {}
        }

        // Battle end: every goblin of the fought altar is gone.
        if self.in_battle {
            let any_alive = self.enemies.iter().any(|e| e.state != EnemyState::Gone && e.state != EnemyState::Idle);
            if !any_alive && self.enemies.iter().all(|e| e.state != EnemyState::Down) {
                self.in_battle = false;
                self.gp += 3;
                self.results = Some(Results { gp: 3 });
                for k in 0..3 {
                    if self.enemies.iter().filter(|e| e.altar == k).all(|e| e.state == EnemyState::Gone) {
                        self.altars_cleared[k] = true;
                        self.event_played[k] = true;
                    }
                }
                self.act = PlayerAct::Free;
                self.target = None;
            }
        }

        // Animate enemies.
        let mut keys: Vec<String> = self.gob_anims.keys().cloned().collect();
        keys.sort();
        let idx_of = |k: &str| keys.iter().position(|x| x == k).unwrap_or(0);
        for e in &mut self.enemies {
            if e.state == EnemyState::Gone {
                continue;
            }
            let (want, looping) = match e.state {
                EnemyState::Idle => ("nut10", true),
                EnemyState::Chase => ("run10", true),
                EnemyState::Attack => ("atn10", false),
                EnemyState::Hurt => ("dma10", false),
                EnemyState::Stagger => ("ght00", true),
                EnemyState::Down => ("dmg20", false),
                EnemyState::Gone => ("nut10", true),
            };
            let want_i = idx_of(want);
            if e.anim != want_i {
                e.anim = want_i;
                e.anim_t = 0.0;
                e.anim_loop = looping;
            }
            e.anim_t += dt * 30.0;
            let Some(anim) = self.gob_anims.get(&keys[e.anim]) else { continue };
            let last = anim.frame_count.saturating_sub(1).max(1) as f32;
            let f = if e.anim_loop { e.anim_t % last } else { e.anim_t.min(last) };
            let Some(act) = scene.spawned_actors.get(e.actor) else { continue };
            let Some(clump) = act.clump.clone() else { continue };
            let nodes = act.scene_nodes.clone();
            let world = anim.compute_world_matrices(&clump, &nodes, &self.gob_ccs, f);
            scene.update_actor_with_matrices(queue, e.actor, e.pos, e.yaw, &world);
            // Sword in the right hand's prop bone.
            if let Some(s) = e.sword {
                let prop = clump.bone_obj_ids.iter().find(|&&id| self.gob_ccs.object_name(id).map(|n| n.ends_with("prop1")).unwrap_or(false)).and_then(|id| world.get(id)).copied();
                if let Some(m) = prop {
                    let root = Mat4::from_translation([e.pos.x, e.pos.y, e.pos.z]).mul(&Mat4::from_rot_z(e.yaw)).mul(&m);
                    scene.update_prop_matrix(queue, s, &root);
                    // Shown with its goblin (hidden at spawn, see spawn_goblin).
                    let goblin_visible = scene.spawned_actors.get(e.actor).and_then(|a| a.submesh_indices.first()).and_then(|&i| scene.submeshes.get(i)).is_some_and(|sm| sm.visible);
                    for (_, _, subs) in &scene.props[s].models {
                        for &si in subs {
                            scene.submeshes[si].visible = goblin_visible;
                        }
                    }
                }
            }
        }

        // Followers: trail behind Haseo; in battle they close in on the target.
        let fwd = forward(*player_yaw);
        let right = Vec2::new(-fwd.y, fwd.x);
        let target_pos = self.target.and_then(|i| self.enemies.get(i)).filter(|e| !matches!(e.state, EnemyState::Down | EnemyState::Gone)).map(|e| e.pos);
        for (n, f) in self.followers.iter_mut().enumerate() {
            if in_event {
                break;
            }
            let goal = match (self.in_battle, target_pos) {
                (true, Some(tp)) => tp.truncate() + Vec2::new(if n == 0 { -1.4 } else { 1.4 }, 1.2),
                _ => player_pos.truncate() - fwd * (1.8 + f.slot.y) + right * f.slot.x,
            };
            let d = goal - f.pos.truncate();
            let dist = d.length();
            let moving = dist > 0.6;
            if moving {
                let speed = if dist > 4.0 { 6.0 } else { 3.6 };
                let step = d / dist * (speed * dt).min(dist);
                f.pos.x += step.x;
                f.pos.y += step.y;
                f.yaw = yaw_to(d);
            } else if let Some(tp) = target_pos {
                f.yaw = yaw_to((tp - f.pos).truncate());
            }
            f.pos.z = ground(f.pos);
            // Allies chip in while next to the target (party AI not traced; approximation).
            f.attack_cd -= dt;
            let mut cast = false;
            if self.in_battle && !moving && f.attack_cd <= 0.0 {
                f.attack_cd = 2.2 + n as f32 * 0.5;
                cast = true;
            }
            let want = if cast { "mag00" } else if moving { "run00" } else { "nut00" };
            let wi = find_anim(&f.anims, want).or_else(|| find_anim(&f.anims, "nut00")).unwrap_or(0);
            if wi != f.anim && !(f.anims.get(f.anim).map(|a| a.name.ends_with("mag00")).unwrap_or(false) && f.t < 20.0) {
                f.anim = wi;
                f.t = 0.0;
            }
            f.t += dt * 30.0;
            if let Some(anim) = f.anims.get(f.anim) {
                let last = anim.frame_count.saturating_sub(1).max(1) as f32;
                let fr = f.t % last;
                scene.update_actor_animation(queue, f.actor, f.pos, f.yaw, anim, &f.ccs, fr);
            }
            if cast {
                if let Some(i) = self.target {
                    if let Some(e) = self.enemies.get_mut(i) {
                        if !matches!(e.state, EnemyState::Down | EnemyState::Gone) && (e.pos - f.pos).length() < 3.0 {
                            e.hp -= 6;
                            self.popups.push(Popup { pos: e.pos + Vec3::new(0.0, 0.0, 1.4), text: "6".into(), t: 0.0, color: [255, 230, 160] });
                            if e.hp <= 0 {
                                e.hp = 0;
                                e.state = EnemyState::Down;
                                e.t = 0.0;
                            }
                        }
                    }
                }
            }
        }

        // Haseo's animation while fighting (field idle/run are left to the app otherwise).
        if self.drives_player_anim() {
            let want = match self.act {
                PlayerAct::Attack(k, _) => ["atn10", "atn20", "atn30", "atn40"][k.min(3)],
                PlayerAct::Hurt(_) => "dma10",
                PlayerAct::Rengeki(_) => "ski10",
                PlayerAct::Free if player_moving => "run10",
                PlayerAct::Free => "nut10",
            };
            if let Some(&i) = self.hsw.get(want) {
                if scene.active_anim_idx != Some(i) {
                    scene.select_animation(i, queue, char_ccs);
                }
            }
        }
        out
    }

    fn altar_of_event(id: &str) -> Option<usize> {
        crate::field::area().events.altars.iter().position(|e| e.as_deref() == Some(id))
    }

    fn set_goblin_visible(scene: &mut GpuScene, e: &Enemy, v: bool) {
        scene.set_actor_visible(e.actor, v);
        if let Some(s) = e.sword {
            for (_, _, subs) in &scene.props[s].models {
                for &si in subs {
                    scene.submeshes[si].visible = v;
                }
            }
        }
    }

    /// An altar event shows its own goblins (cast ゴブナイトA-C); the field's are hidden meanwhile.
    pub fn event_started(&mut self, scene: &mut GpuScene, id: &str) {
        if let Some(k) = Self::altar_of_event(id) {
            for e in self.enemies.iter().filter(|e| e.altar == k) {
                Self::set_goblin_visible(scene, e, false);
            }
        }
    }

    /// After an altar event the field goblins stand where the event left its cast.
    pub fn event_ended(&mut self, scene: &mut GpuScene, id: &str, goblin_marks: &[(Vec3, f32)]) {
        if let Some(k) = Self::altar_of_event(id) {
            let mut marks = goblin_marks.iter();
            for e in self.enemies.iter_mut().filter(|e| e.altar == k) {
                if let Some(&(p, yaw)) = marks.next() {
                    e.pos = p;
                    e.yaw = yaw;
                }
                if e.state != EnemyState::Gone {
                    Self::set_goblin_visible(scene, e, true);
                }
            }
        }
    }

    /// Show the followers next to Haseo (after PM10105/PM10110 hand over to free play).
    pub fn place_followers(&mut self, scene: &mut GpuScene, player_pos: Vec3, player_yaw: f32) {
        self.followers_shown = true;
        let fwd = forward(player_yaw);
        let right = Vec2::new(-fwd.y, fwd.x);
        for f in &mut self.followers {
            let p = player_pos.truncate() - fwd * (1.8 + f.slot.y) + right * f.slot.x;
            f.pos = Vec3::new(p.x, p.y, player_pos.z);
            f.yaw = player_yaw;
            scene.set_actor_visible(f.actor, true);
        }
    }
    pub fn hide_followers(&mut self, scene: &mut GpuScene) {
        self.followers_shown = false;
        for f in &self.followers {
            scene.set_actor_visible(f.actor, false);
        }
    }

    /// Minimap (top left). Panel measured on the original (960x540: x 48..215, y 27..194, near-black translucent);
    /// land colour (97,109,95) sampled; north-up; the zoom (one 16 m cell = 10 px, from island sizes) and marker sizes are estimates.
    /// Land/bridges come from field::GRID, markers from TEX_mapicon (player arrow at 15,4 34x55; dots 30x30).
    pub fn draw_minimap(&mut self, ctx: &egui::Context, player: Vec3, yaw: f32) {
        if self.map_icons.is_none() {
            if let Some(c) = load_ccs("mapmenu@ENG.ccs").or_else(|| load_ccs("mapmenu.ccs")) {
                for t in c.objects_of_type(hackgu_ccs::objects::SECTION_TEXTURE) {
                    if c.object_name(t.object_id) == Some("TEX_mapicon") {
                        if let Ok(d) = hackgu_ccs::DecodedTexture::from_payload_with_clut("mapicon", &t.payload, None) {
                            let img = egui::ColorImage::from_rgba_unmultiplied([d.width, d.height], &d.rgba);
                            self.map_icons = Some(ctx.load_texture("mapicon", img, egui::TextureOptions::LINEAR));
                        }
                    }
                }
            }
        }
        let screen = ctx.screen_rect();
        let (w, h) = if screen.width() / screen.height() > 16.0 / 9.0 { (screen.height() * 16.0 / 9.0, screen.height()) } else { (screen.width(), screen.width() * 9.0 / 16.0) };
        let o = egui::Pos2::new(screen.center().x - w * 0.5, screen.center().y - h * 0.5);
        let k = w / 960.0;
        let panel = egui::Rect::from_min_max(o + egui::vec2(48.0, 27.0) * k, o + egui::vec2(215.0, 194.0) * k);
        let p = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("minimap"))).with_clip_rect(panel);
        p.rect_filled(panel, 14.0 * k, egui::Color32::from_rgba_unmultiplied(8, 10, 12, 200));
        let area = crate::field::area();
        let px_per_m = 10.0 / area.cell * k;
        let centre = panel.center();
        let to_map = |wp: Vec3| egui::Pos2::new(centre.x + (wp.x - player.x) * px_per_m, centre.y - (wp.y - player.y) * px_per_m);
        let land = egui::Color32::from_rgb(97, 109, 95);
        let grid = &area.grid;
        let at = |i: usize, j: usize| grid.get(j).and_then(|r| r.as_bytes().get(i).copied()).unwrap_or(b'.');
        for j in 0..grid.len() {
            for i in 0..grid[j].len() {
                let a = to_map(crate::field::world(i as f32, j as f32));
                let b = to_map(crate::field::world(i as f32 + 1.0, j as f32 + 1.0));
                let r = egui::Rect::from_two_pos(a, b);
                if !r.intersects(panel) {
                    continue;
                }
                let c = at(i, j);
                if c == b'#' {
                    p.rect_filled(r.expand(0.6), 0.0, land);
                } else if c == b'=' {
                    let vertical = at(i, j + 1) != b'.' || (j > 0 && at(i, j - 1) != b'.');
                    let bw = 0.45 * r.width().min(r.height());
                    let br = if vertical { egui::Rect::from_center_size(r.center(), egui::vec2(bw, r.height() + 1.0)) } else { egui::Rect::from_center_size(r.center(), egui::vec2(r.width() + 1.0, bw)) };
                    p.rect_filled(br, 0.0, egui::Color32::from_rgb(150, 120, 80));
                    for s in 0..6 {
                        let t = (s as f32 + 0.5) / 6.0;
                        let seg = if vertical {
                            [egui::pos2(br.left(), br.top() + br.height() * t), egui::pos2(br.right(), br.top() + br.height() * t)]
                        } else {
                            [egui::pos2(br.left() + br.width() * t, br.top()), egui::pos2(br.left() + br.width() * t, br.bottom())]
                        };
                        p.line_segment(seg, egui::Stroke::new(1.0, egui::Color32::from_rgb(90, 70, 45)));
                    }
                }
            }
        }
        let icons = self.map_icons.clone();
        let icon = |c: egui::Pos2, size: f32, uv: [f32; 4]| {
            if let Some(t) = &icons {
                p.image(t.id(), egui::Rect::from_center_size(c, egui::vec2(size, size) * k), egui::Rect::from_min_max(egui::pos2(uv[0] / 512.0, uv[1] / 512.0), egui::pos2(uv[2] / 512.0, uv[3] / 512.0)), egui::Color32::WHITE);
            }
        };
        // Platforms (blue dot) and unopened altar chests (grey dot).
        for pl in [crate::field::arrival_platform(), crate::field::exit_platform()] {
            icon(to_map(pl), 11.0, [81.0, 17.0, 111.0, 47.0]);
        }
        for n in 0..area.altars.len() {
            if !self.chests_opened.get(n).copied().unwrap_or(true) {
                icon(to_map(area.altar_world(n)), 10.0, [273.0, 17.0, 303.0, 47.0]);
            }
        }
        for t in &self.trail {
            p.circle_filled(to_map(*t), 2.6 * k, egui::Color32::from_rgba_unmultiplied(235, 235, 235, 200));
        }
        // Player arrow (TEX_mapicon 15,4 34x55, pointing up), rotated to the facing.
        if let Some(t) = &icons {
            let f = forward(yaw);
            let dir = egui::vec2(f.x, -f.y);
            let side = egui::vec2(-dir.y, dir.x);
            let (hw, hh) = (6.0 * k, 10.0 * k);
            let corner = |sx: f32, sy: f32| centre + side * (sx * hw) - dir * (sy * hh);
            let uv = |u: f32, v: f32| egui::pos2(u / 512.0, v / 512.0);
            let mut mesh = egui::Mesh::with_texture(t.id());
            for (pt, tuv) in [(corner(-1.0, -1.0), uv(15.0, 4.0)), (corner(1.0, -1.0), uv(49.0, 4.0)), (corner(1.0, 1.0), uv(49.0, 59.0)), (corner(-1.0, 1.0), uv(15.0, 59.0))] {
                mesh.vertices.push(egui::epaint::Vertex { pos: pt, uv: tuv, color: egui::Color32::WHITE });
            }
            mesh.indices = vec![0, 1, 2, 0, 2, 3];
            p.add(egui::Shape::mesh(mesh));
        }
    }

    /// Followers' (hp, max hp, sp) in party order (for the Main Menu).
    pub fn follower_status(&self) -> Vec<(i32, i32, i32)> {
        self.followers.iter().map(|f| (f.hp, f.max_hp, f.sp)).collect()
    }

    /// Battle HUD, messages and results (egui, gamma space).
    pub fn draw_hud(&self, ctx: &egui::Context, project: &dyn Fn(Vec3) -> Option<egui::Pos2>) {
        let screen = ctx.screen_rect();
        let (w, h) = if screen.width() / screen.height() > 16.0 / 9.0 { (screen.height() * 16.0 / 9.0, screen.height()) } else { (screen.width(), screen.width() * 9.0 / 16.0) };
        let o = egui::Pos2::new(screen.center().x - w * 0.5, screen.center().y - h * 0.5);
        let at = |x: f32, y: f32| egui::Pos2::new(o.x + x / 960.0 * w, o.y + y / 540.0 * h);
        // Sizes are capital heights in 540p reference pixels (Segoe UI capitals are 0.70 of the font size).
        let font = |px: f32| egui::FontId::proportional(px / 540.0 * h / 0.70);
        let p = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("field_hud")));
        let poly = |pts: &[(f32, f32)], c: egui::Color32| {
            p.add(egui::Shape::convex_polygon(pts.iter().map(|&(x, y)| at(x, y)).collect(), c, egui::Stroke::new(1.0, egui::Color32::from_rgb(120, 110, 110))));
        };
        // Party boxes (measured from the original: three rows at y 404, 437, 470; x 30..360).
        let rows: Vec<(i32, i32, i32, i32)> = std::iter::once((self.hp, self.max_hp, self.sp, 20)).chain(self.followers.iter().map(|f| (f.hp, f.max_hp, f.sp, f.sp))).collect();
        for (n, (hp, max_hp, sp, max_sp)) in rows.iter().enumerate() {
            let y = 404.0 + n as f32 * 33.0;
            poly(&[(44.0, y - 14.0), (348.0, y - 14.0), (360.0, y), (348.0, y + 14.0), (44.0, y + 14.0), (32.0, y)], egui::Color32::from_rgba_unmultiplied(10, 8, 8, 215));
            p.text(at(152.0, y - 1.0), egui::Align2::LEFT_CENTER, "HP", font(12.0), egui::Color32::WHITE);
            p.text(at(232.0, y - 1.0), egui::Align2::RIGHT_CENTER, hp.to_string(), font(12.0), egui::Color32::WHITE);
            p.text(at(260.0, y - 1.0), egui::Align2::LEFT_CENTER, "SP", font(12.0), egui::Color32::WHITE);
            p.text(at(340.0, y - 1.0), egui::Align2::RIGHT_CENTER, sp.to_string(), font(12.0), egui::Color32::WHITE);
            let bar = |x0: f32, x1: f32, frac: f32, c: egui::Color32| {
                p.rect_filled(egui::Rect::from_min_max(at(x0, y + 9.0), at(x1, y + 11.0)), 0.0, egui::Color32::from_rgb(50, 45, 45));
                p.rect_filled(egui::Rect::from_min_max(at(x0, y + 9.0), at(x0 + (x1 - x0) * frac.clamp(0.0, 1.0), y + 11.0)), 0.0, c);
            };
            bar(136.0, 236.0, *hp as f32 / (*max_hp).max(1) as f32, egui::Color32::from_rgb(225, 175, 60));
            bar(244.0, 344.0, *sp as f32 / (*max_sp).max(1) as f32, egui::Color32::from_rgb(70, 190, 200));
        }
        // Target box.
        if let Some(e) = self.target.and_then(|i| self.enemies.get(i)).filter(|e| e.state != EnemyState::Gone) {
            poly(&[(696.0, 66.0), (916.0, 66.0), (928.0, 80.0), (928.0, 118.0), (916.0, 132.0), (696.0, 132.0), (684.0, 118.0), (684.0, 80.0)], egui::Color32::from_rgba_unmultiplied(10, 8, 8, 215));
            p.text(at(805.0, 87.0), egui::Align2::CENTER_CENTER, "Goblin Rookie", font(13.0), egui::Color32::WHITE);
            p.text(at(835.0, 111.0), egui::Align2::RIGHT_CENTER, "HP", font(11.0), egui::Color32::WHITE);
            p.text(at(908.0, 111.0), egui::Align2::RIGHT_CENTER, e.hp.to_string(), font(12.0), egui::Color32::WHITE);
            p.rect_filled(egui::Rect::from_min_max(at(700.0, 120.0), at(910.0, 123.0)), 0.0, egui::Color32::from_rgb(170, 30, 40));
            p.rect_filled(egui::Rect::from_min_max(at(700.0, 120.0), at(700.0 + 210.0 * e.hp as f32 / e.max_hp as f32, 123.0)), 0.0, egui::Color32::from_rgb(235, 190, 60));
        }
        // Combo.
        if self.combo >= 2 {
            p.text(at(110.0, 70.0), egui::Align2::CENTER_CENTER, format!("Combo {} hits!", self.combo), font(15.0), egui::Color32::from_rgb(250, 220, 120));
        }
        // Damage numbers.
        for pp in &self.popups {
            if let Some(s) = project(pp.pos) {
                let a = (255.0 * (1.0 - pp.t)).clamp(0.0, 255.0) as u8;
                let c = egui::Color32::from_rgba_unmultiplied(pp.color[0], pp.color[1], pp.color[2], a);
                p.text(s, egui::Align2::CENTER_CENTER, &pp.text, font(16.0), c);
            }
        }
        if self.rengeki_window > 0.0 {
            p.text(at(560.0, 383.0), egui::Align2::CENTER_CENTER, "Rengeki!  Press A", font(15.0), egui::Color32::from_rgb(255, 210, 90));
        }
        if self.banner_t > 0.0 {
            let a = (self.banner_t.min(1.0) * 255.0) as u8;
            p.text(at(480.0, 250.0), egui::Align2::CENTER_CENTER, "Outbreak of Battle", font(22.0), egui::Color32::from_rgba_unmultiplied(255, 255, 255, a));
        }
        if let Some((m, _)) = &self.message {
            poly(&[(200.0, 330.0), (760.0, 330.0), (774.0, 345.0), (760.0, 360.0), (200.0, 360.0), (186.0, 345.0)], egui::Color32::from_rgba_unmultiplied(10, 8, 8, 225));
            p.text(at(480.0, 345.0), egui::Align2::CENTER_CENTER, m, font(13.0), egui::Color32::WHITE);
        }
        if let Some(r) = &self.results {
            p.rect_filled(screen, 0.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 120));
            p.rect_filled(egui::Rect::from_min_max(at(0.0, 24.0), at(960.0, 58.0)), 0.0, egui::Color32::from_rgba_unmultiplied(14, 10, 10, 230));
            p.text(at(48.0, 41.0), egui::Align2::LEFT_CENTER, "Battle Result", font(13.5), egui::Color32::WHITE);
            p.text(at(910.0, 41.0), egui::Align2::RIGHT_CENTER, "A: Next", font(13.5), egui::Color32::WHITE);
            poly(&[(330.0, 222.0), (630.0, 222.0), (644.0, 238.0), (630.0, 254.0), (330.0, 254.0), (316.0, 238.0)], egui::Color32::from_rgba_unmultiplied(14, 10, 10, 235));
            p.text(at(480.0, 238.0), egui::Align2::CENTER_CENTER, "Acquired GP/Item", font(13.0), egui::Color32::WHITE);
            poly(&[(226.0, 256.0), (734.0, 256.0), (748.0, 270.0), (748.0, 286.0), (734.0, 300.0), (226.0, 300.0), (212.0, 286.0), (212.0, 270.0)], egui::Color32::from_rgba_unmultiplied(40, 10, 12, 235));
            p.text(at(480.0, 277.0), egui::Align2::CENTER_CENTER, format!("{} GP", r.gp), font(13.0), egui::Color32::WHITE);
        }
    }
}
