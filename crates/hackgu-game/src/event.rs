//! CyberConnect2 Script & Event System (`ccEventBase`, `ccEvent*`).
//! Reconstructed from binary decompilation and CSV scripts (`PM10100.csv`, etc.).

use glam::Vec3;
use serde::{Deserialize, Serialize};

/// High-level engine events dispatched during cutscenes, story quests, and field interactions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GameEvent {
    /// Initialize player position and orientation (`ccEventHasewoInit`)
    PlayerInit {
        spawn_pos: Vec3,
        spawn_yaw: f32,
    },
    /// Lock or unlock player movement / camera controls (`ccEventPlayer`)
    PlayerControlLock {
        locked: bool,
    },
    /// Register an actor into the scene cast list (`ccEventEntry`, opcode 63)
    ActorRegister {
        role: String,
        model: String,
        instance_name: String,
        actor_id: u32,
    },
    /// Position and orient an actor in the stage (`ccEventManipObj`, opcode 32)
    ActorPlace {
        instance_name: String,
        position: Vec3,
        yaw: f32,
        visible: bool,
    },
    /// Trigger character skeletal animation playback (`ccEventAction`, opcode 7)
    ActorPlayAnim {
        instance_name: String,
        anim_name: String,
        blend_frames: u32,
    },
    /// Direct an actor to navigate to a target coordinate (`ccEventAction`, opcode 9)
    ActorWalkTo {
        instance_name: String,
        destination: Vec3,
        speed: f32,
    },
    /// Despawn / delete an actor from the current scene (`ccEventDeleteObj`, opcode 96)
    ActorDespawn {
        instance_name: String,
    },
    /// Cutscene camera cut or spline transition (`ccEventCamera`, opcode 60)
    CameraCut {
        eye: Vec3,
        target: Vec3,
        fov: f32,
        duration: f32,
    },
    /// Translate an object over time (`ccEventTrans`)
    TranslateObject {
        obj_id: u32,
        target_pos: Vec3,
        speed: f32,
    },
    /// Rotate an object over time (`ccEventRotate`)
    RotateObject {
        obj_id: u32,
        target_rot: Vec3,
        angular_speed: f32,
    },
    /// Screen fade in or fade out (`ccEventFade`, opcodes 39, 40)
    ScreenFade {
        color_rgba: [f32; 4],
        duration: f32,
        fade_in: bool,
    },
    /// Play background music track (`ccEventSeOn`, opcode 123)
    PlayBgm {
        bgm_id: u32,
        fade_duration: f32,
    },
    /// Fade out / stop background music (`opcode 135`)
    StopBgm {
        fade_duration: f32,
    },
    /// Display dialogue message window (`ccEventMsg` / `ccEventDtmsgwindow`)
    ShowMessage {
        speaker: String,
        text: String,
    },
    /// Wait for a duration in seconds (`ccEventWait`, opcode 4)
    Wait {
        duration: f32,
    },
    /// Transition into combat encounter (`ccEventBattle`)
    BattleTransition {
        encounter_id: u32,
        bgm_override: Option<String>,
    },
    /// Transition to a new area / stage (`ccEventAreamove`)
    AreaMove {
        stage_name: String,
        spawn_point_id: u32,
    },
    /// Set a story / quest flag (`ccEventFlag`)
    SetFlag {
        flag_id: u32,
        value: i32,
    },
    /// Open the Chaos Gate Area Selection menu
    OpenChaosGate,
    /// Trigger Chim Chim kick reaction
    KickChimBarrel {
        barrel_id: u32,
        chim_count: u32,
    },
}

/// Helper function to parse `[X|Y|Z]` vector coordinate strings used throughout CC2 CSV scripts.
pub fn parse_vec3_bracket(s: &str) -> Option<Vec3> {
    let trimmed = s.trim().trim_start_matches('[').trim_end_matches(']');
    let parts: Vec<&str> = trimmed.split('|').collect();
    if parts.len() == 3 {
        let x = parts[0].trim().parse::<f32>().ok()?;
        let y = parts[1].trim().parse::<f32>().ok()?;
        let z = parts[2].trim().parse::<f32>().ok()?;
        Some(Vec3::new(x, y, z))
    } else {
        None
    }
}

/// CyberConnect2 Event Script Parser for `.csv` event scripts (`PM10100.csv`, etc.).
#[derive(Clone, Debug, Default)]
pub struct EventScript {
    pub name: String,
    pub spawn_pos: Option<Vec3>,
    pub spawn_yaw: Option<f32>,
    pub events: Vec<GameEvent>,
}

impl EventScript {
    /// Parse authentic CC2 CSV script into a sequence of executable `GameEvent`s.
    pub fn parse_csv(csv_text: &str) -> Self {
        let mut script = Self::default();
        let lines: Vec<&str> = csv_text.lines().collect();

        if lines.is_empty() {
            return script;
        }

        // Parse line 1 (header: name, spawn_pos, yaw)
        let header_parts: Vec<&str> = lines[0].split(',').collect();
        if !header_parts.is_empty() {
            script.name = header_parts[0].trim().to_string();
        }
        if header_parts.len() > 2 {
            if let Some(pos) = parse_vec3_bracket(header_parts[2]) {
                script.spawn_pos = Some(pos);
            }
        }
        if header_parts.len() > 3 {
            if let Ok(yaw) = header_parts[3].trim().parse::<f32>() {
                script.spawn_yaw = Some(yaw);
            }
        }

        if let (Some(pos), Some(yaw)) = (script.spawn_pos, script.spawn_yaw) {
            script.events.push(GameEvent::PlayerInit {
                spawn_pos: pos,
                spawn_yaw: yaw,
            });
        }

        for line in &lines[1..] {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 5 {
                continue;
            }

            // Search for recognized CC2 opcode in the line
            let mut opcode_info = None;
            for (idx, p) in parts.iter().enumerate() {
                if let Ok(op) = p.trim().parse::<u32>() {
                    if [4, 7, 9, 32, 39, 40, 41, 60, 61, 62, 63, 75, 81, 96, 123, 135].contains(&op) {
                        opcode_info = Some((idx, op));
                        break;
                    }
                }
            }

            let (op_idx, opcode) = match opcode_info {
                Some(info) => info,
                None => continue,
            };

            match opcode {
                // Opcode 63: Register Cast Actor (Role, Model, InstanceName, ID)
                63 => {
                    let role = parts.get(op_idx + 1).unwrap_or(&"").trim().to_string();
                    let model = parts.get(op_idx + 2).unwrap_or(&"").trim().to_string();
                    let instance_name = parts.get(op_idx + 3).unwrap_or(&"").trim().to_string();
                    let actor_id = parts.get(op_idx + 4).and_then(|s| s.trim().parse::<u32>().ok()).unwrap_or(0);
                    if !instance_name.is_empty() {
                        script.events.push(GameEvent::ActorRegister {
                            role,
                            model,
                            instance_name,
                            actor_id,
                        });
                    }
                }
                // Opcode 32: Actor Place (InstanceName, Position, Yaw, Visible)
                32 => {
                    let instance_name = parts.get(op_idx + 2).unwrap_or(&"").trim().to_string();
                    let pos = parts.get(op_idx + 3).and_then(|s| parse_vec3_bracket(s)).unwrap_or(Vec3::ZERO);
                    let yaw = parts.get(op_idx + 4).and_then(|s| s.trim().parse::<f32>().ok()).unwrap_or(0.0);
                    let visible = parts.get(op_idx + 5).map_or(true, |s| s.trim().eq_ignore_ascii_case("ON"));
                    if !instance_name.is_empty() {
                        script.events.push(GameEvent::ActorPlace {
                            instance_name,
                            position: pos,
                            yaw,
                            visible,
                        });
                    }
                }
                // Opcode 7: Play Animation (InstanceName, AnimName, BlendFrames)
                7 => {
                    let instance_name = parts.get(op_idx + 2).unwrap_or(&"").trim().to_string();
                    let anim_name = parts.get(op_idx + 3).unwrap_or(&"").trim().to_string();
                    let blend_frames = parts.get(op_idx + 4).and_then(|s| s.trim().parse::<u32>().ok()).unwrap_or(0);
                    if !instance_name.is_empty() && !anim_name.is_empty() {
                        script.events.push(GameEvent::ActorPlayAnim {
                            instance_name,
                            anim_name,
                            blend_frames,
                        });
                    }
                }
                // Opcode 9: Move / Walk To (InstanceName, TargetPos, Speed)
                9 => {
                    let instance_name = parts.get(op_idx + 2).unwrap_or(&"").trim().to_string();
                    let dest = parts.get(op_idx + 3).and_then(|s| parse_vec3_bracket(s)).unwrap_or(Vec3::ZERO);
                    let speed = parts.get(op_idx + 4).and_then(|s| s.trim().parse::<f32>().ok()).unwrap_or(1.0);
                    if !instance_name.is_empty() {
                        script.events.push(GameEvent::ActorWalkTo {
                            instance_name,
                            destination: dest,
                            speed,
                        });
                    }
                }
                // Opcode 96: Despawn Actor (InstanceName)
                96 => {
                    let instance_name = parts.get(op_idx + 2).unwrap_or(&"").trim().to_string();
                    if !instance_name.is_empty() {
                        script.events.push(GameEvent::ActorDespawn { instance_name });
                    }
                }
                // Opcode 60: Camera Track
                60 => {
                    // Ver0003:2:[pos_start]:[target_start]:roll:fov:near:far:...
                    let track_data = parts.iter().find(|p| p.trim().starts_with("Ver")).unwrap_or(&"");
                    if !track_data.is_empty() {
                        let subparts: Vec<&str> = track_data.split(':').collect();
                        if subparts.len() >= 4 {
                            let eye = parse_vec3_bracket(subparts[2]).unwrap_or(Vec3::new(0.0, 5.0, 2.0));
                            let target = parse_vec3_bracket(subparts[3]).unwrap_or(Vec3::ZERO);
                            let fov = subparts.get(5).and_then(|s| s.parse::<f32>().ok()).unwrap_or(60.0);
                            let duration = subparts.get(6).and_then(|s| s.parse::<f32>().ok()).unwrap_or(3.0);
                            script.events.push(GameEvent::CameraCut {
                                eye,
                                target,
                                fov,
                                duration,
                            });
                        }
                    }
                }
                // Opcode 4: Wait (Frames / 30.0 fps)
                4 => {
                    let raw_frames = parts.get(op_idx + 2).filter(|s| !s.trim().is_empty())
                        .or_else(|| parts.get(op_idx + 1));
                    let frames = raw_frames.and_then(|s| s.trim().parse::<f32>().ok()).unwrap_or(30.0);
                    let duration = (frames / 30.0).max(0.033);
                    script.events.push(GameEvent::Wait { duration });
                }
                // Opcode 39: Screen Fade In
                39 => {
                    let raw_frames = parts.get(op_idx + 2).filter(|s| !s.trim().is_empty())
                        .or_else(|| parts.get(op_idx + 1));
                    let frames = raw_frames.and_then(|s| s.trim().parse::<f32>().ok()).unwrap_or(30.0);
                    let duration = (frames / 30.0).max(0.033);
                    script.events.push(GameEvent::ScreenFade {
                        color_rgba: [0.0, 0.0, 0.0, 1.0],
                        duration,
                        fade_in: true,
                    });
                }
                // Opcode 40: Screen Fade Out
                40 => {
                    let raw_frames = parts.get(op_idx + 2).filter(|s| !s.trim().is_empty())
                        .or_else(|| parts.get(op_idx + 1));
                    let frames = raw_frames.and_then(|s| s.trim().parse::<f32>().ok()).unwrap_or(30.0);
                    let duration = (frames / 30.0).max(0.033);
                    script.events.push(GameEvent::ScreenFade {
                        color_rgba: [0.0, 0.0, 0.0, 1.0],
                        duration,
                        fade_in: false,
                    });
                }
                // Opcode 123: Play BGM
                123 => {
                    let bgm_id = parts.get(op_idx + 2).and_then(|s| s.trim().parse::<u32>().ok()).unwrap_or(0);
                    let fade_frames = parts.get(op_idx + 3).and_then(|s| s.trim().parse::<f32>().ok()).unwrap_or(30.0);
                    script.events.push(GameEvent::PlayBgm {
                        bgm_id,
                        fade_duration: fade_frames / 30.0,
                    });
                }
                // Opcode 135: Stop BGM
                135 => {
                    let fade_frames = parts.get(op_idx + 2).and_then(|s| s.trim().parse::<f32>().ok()).unwrap_or(30.0);
                    script.events.push(GameEvent::StopBgm {
                        fade_duration: fade_frames / 30.0,
                    });
                }
                _ => {}
            }
        }

        script
    }
}

/// Script execution sequence runner.
#[derive(Clone, Debug, Default)]
pub struct EventQueue {
    pub events: Vec<GameEvent>,
    pub current_index: usize,
    pub wait_timer: f32,
}

impl EventQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, event: GameEvent) {
        self.events.push(event);
    }

    pub fn push_script(&mut self, script: EventScript) {
        self.events.extend(script.events);
    }

    pub fn is_empty(&self) -> bool {
        self.current_index >= self.events.len()
    }

    pub fn clear(&mut self) {
        self.events.clear();
        self.current_index = 0;
        self.wait_timer = 0.0;
    }

    /// Advance active event execution. Returns current event if actively executing.
    pub fn step(&mut self, dt: f32) -> Option<&GameEvent> {
        if self.current_index >= self.events.len() {
            return None;
        }

        if self.wait_timer > 0.0 {
            self.wait_timer = (self.wait_timer - dt).max(0.0);
            return self.events.get(self.current_index);
        }

        let ev = &self.events[self.current_index];
        match ev {
            GameEvent::Wait { duration } => {
                self.wait_timer = *duration;
                self.current_index += 1;
            }
            _ => {
                self.current_index += 1;
            }
        }

        self.events.get(self.current_index)
    }
}
