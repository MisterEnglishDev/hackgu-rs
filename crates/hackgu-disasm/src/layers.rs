//! Game Layer Classification & Master Function Cataloguer.
//! Classifies all 17,237 functions into 7 distinct game subsystems based on
//! RTTI class membership, string cross-references, and call graphs.

use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::disasm::{CodeAnalyzer, FunctionEntry};
use crate::pe::PeFile;
use crate::rtti::RttiClassInfo;
use crate::scanner::StringRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameLayer {
    /// Layer 1: Engine Foundation & System Core (memory pools, files, CRIWARE VFS, thread dispatch, maths)
    CoreSys,
    /// Layer 2: Graphics, Clumps & Animation Engine (bone hierarchies, skinning, morphs, textures, cameras)
    GraphicsAnim,
    /// Layer 3: Event & Cutscene Script VM (ccEvent, dialogue engines, camera tracks, scene sequencing)
    EventScript,
    /// Layer 4: World, Stage & Exploration (Mac Anu, fields, dungeons, collision mesh, Chaos Gate)
    WorldStage,
    /// Layer 5: Actors, NPCs & Player Character (Haseo ccGobjPc, Iyoten, Asta, Silabus, enemy mobs, chims)
    ActorsEntities,
    /// Layer 6: Battle Engine & Combat System (damage formulas, hitboxes, Rengeki triggers, Avatar battles, AI)
    BattleCombat,
    /// Layer 7: User Interface & Altimit OS Desktop (Altimit OS mail/BBS, menus, shop, bike customizer, HUD)
    UiDesktop,
}

impl GameLayer {
    pub fn name(&self) -> &'static str {
        match self {
            GameLayer::CoreSys => "Layer 1: Core System & Foundation",
            GameLayer::GraphicsAnim => "Layer 2: Graphics, Clumps & Animation",
            GameLayer::EventScript => "Layer 3: Event & Cutscene Script VM",
            GameLayer::WorldStage => "Layer 4: World, Stage & Exploration",
            GameLayer::ActorsEntities => "Layer 5: Actors, Entities & Player",
            GameLayer::BattleCombat => "Layer 6: Battle Engine & Combat System",
            GameLayer::UiDesktop => "Layer 7: User Interface & Altimit Desktop",
        }
    }

    pub fn slug(&self) -> &'static str {
        match self {
            GameLayer::CoreSys => "layer1_core_sys",
            GameLayer::GraphicsAnim => "layer2_graphics_anim",
            GameLayer::EventScript => "layer3_event_script",
            GameLayer::WorldStage => "layer4_world_stage",
            GameLayer::ActorsEntities => "layer5_actors_entities",
            GameLayer::BattleCombat => "layer6_battle_combat",
            GameLayer::UiDesktop => "layer7_ui_desktop",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassifiedFunction {
    pub rva: u32,
    pub size: u32,
    pub layer: GameLayer,
    pub class_name: Option<String>,
    pub virtual_method_index: Option<usize>,
    pub instruction_count: usize,
    pub basic_block_count: usize,
    pub string_xrefs: Vec<String>,
    pub calls_out: Vec<u32>,
}

pub struct LayerManager;

impl LayerManager {
    /// Classify all functions in PE into the 7 game layers
    pub fn classify_all(
        pe: &PeFile,
        funcs: &[FunctionEntry],
        rtti_classes: &BTreeMap<String, RttiClassInfo>,
        strings: &[StringRef],
        analyzer: &CodeAnalyzer,
    ) -> Vec<ClassifiedFunction> {
        // 1. Map virtual method RVAs -> (class_name, method_index)
        let mut rva_to_class_method: HashMap<u32, (String, usize)> = HashMap::new();
        for cls in rtti_classes.values() {
            let display_name = if !cls.demangled_name.is_empty() {
                &cls.demangled_name
            } else {
                &cls.name
            };
            for (m_idx, &m_va) in cls.virtual_methods.iter().enumerate() {
                let m_rva = (m_va.saturating_sub(pe.image_base)) as u32;
                rva_to_class_method.insert(m_rva, (display_name.clone(), m_idx));
            }
        }

        // 2. Map string RVAs -> string value
        let string_map: BTreeMap<u32, &String> = strings.iter().map(|s| (s.rva, &s.value)).collect();

        // 3. Classify every function entry
        let mut classified = Vec::with_capacity(funcs.len());

        for f in funcs {
            let decomp = analyzer.disassemble_function(f.rva, f.size.max(32) as usize);
            let insn_count = decomp.as_ref().map(|d| d.instructions.len()).unwrap_or(0);
            let bb_count = decomp.as_ref().map(|d| d.basic_blocks.len()).unwrap_or(1);
            let calls_out: Vec<u32> = decomp.as_ref().map(|d| d.calls.iter().map(|&va| (va.saturating_sub(pe.image_base)) as u32).collect()).unwrap_or_default();

            // Find string cross references in function body
            let mut string_xrefs = Vec::new();
            if let Some(ref d) = decomp {
                for insn in &d.instructions {
                    let rip_target = insn.ip_rel_memory_address() as u32;
                    if rip_target > 0 {
                        if let Some(&s) = string_map.get(&rip_target) {
                            if !string_xrefs.contains(s) {
                                string_xrefs.push(s.clone());
                            }
                        }
                    }
                }
            }

            // Determine Layer
            let (class_info, layer) = if let Some((cls_name, m_idx)) = rva_to_class_method.get(&f.rva) {
                let l = Self::layer_from_class_name(cls_name);
                (Some((cls_name.clone(), *m_idx)), l)
            } else {
                let l = Self::layer_from_strings(&string_xrefs, f.rva);
                (None, l)
            };

            classified.push(ClassifiedFunction {
                rva: f.rva,
                size: f.size,
                layer,
                class_name: class_info.as_ref().map(|(c, _)| c.clone()),
                virtual_method_index: class_info.as_ref().map(|(_, m)| *m),
                instruction_count: insn_count,
                basic_block_count: bb_count,
                string_xrefs,
                calls_out,
            });
        }

        classified
    }

    /// Determine layer from CC2 class name
    fn layer_from_class_name(name: &str) -> GameLayer {
        if name.starts_with("ccGobjPc") || name.starts_with("ccGobjNpc") || name.starts_with("ccGobjEnemy") || name.starts_with("ccGobjChim") || name.starts_with("ccGobjGim") || name.starts_with("ccGobjSpc") {
            GameLayer::ActorsEntities
        } else if name.starts_with("ccAvatarBattle") || name.starts_with("ccArenaBattle") || name.starts_with("ccBattle") || name.starts_with("ccGobjBattle") {
            GameLayer::BattleCombat
        } else if name.starts_with("ccEvent") || name.starts_with("ccEvMachine") || name.starts_with("ccEventTransactor") {
            GameLayer::EventScript
        } else if name.contains("Menu") || name.contains("Wnd") || name.contains("Disp") || name.contains("Gauge") || name.contains("Desktop") || name.contains("Web") || name.contains("Bike") {
            GameLayer::UiDesktop
        } else if name.starts_with("ccCoord") || name.starts_with("ccGeom") || name.starts_with("ccClump") || name.starts_with("ccAnm") || name.starts_with("ccCamera") {
            GameLayer::GraphicsAnim
        } else if name.contains("Stage") || name.contains("Town") || name.contains("Dungeon") || name.contains("Area") || name.contains("Field") {
            GameLayer::WorldStage
        } else {
            GameLayer::CoreSys
        }
    }

    /// Determine layer from referenced strings
    fn layer_from_strings(strings: &[String], rva: u32) -> GameLayer {
        for s in strings {
            let lower = s.to_lowercase();
            if lower.contains("event") || lower.contains("ev_") || lower.contains("pm10") || lower.contains("mm10") {
                return GameLayer::EventScript;
            }
            if lower.contains("town") || lower.contains("stage") || lower.contains("dungeon") || lower.contains("field") || lower.contains("chaos") {
                return GameLayer::WorldStage;
            }
            if lower.contains("battle") || lower.contains("rengeki") || lower.contains("combo") || lower.contains("damage") || lower.contains("attack") {
                return GameLayer::BattleCombat;
            }
            if lower.contains("menu") || lower.contains("window") || lower.contains("button") || lower.contains("cursor") || lower.contains("desktop") || lower.contains("mail") {
                return GameLayer::UiDesktop;
            }
            if lower.contains("anime") || lower.contains("clump") || lower.contains("bone") || lower.contains("model") || lower.contains("mesh") {
                return GameLayer::GraphicsAnim;
            }
            if lower.contains("player") || lower.contains("enemy") || lower.contains("npc") || lower.contains("haseo") {
                return GameLayer::ActorsEntities;
            }
        }

        // Heuristic based on code section layout in hackGU_vol1.dll:
        if rva < 0x00100000 {
            GameLayer::CoreSys
        } else if rva < 0x00250000 {
            GameLayer::GraphicsAnim
        } else if rva < 0x00350000 {
            GameLayer::EventScript
        } else if rva < 0x00450000 {
            GameLayer::WorldStage
        } else if rva < 0x00550000 {
            GameLayer::ActorsEntities
        } else if rva < 0x00650000 {
            GameLayer::BattleCombat
        } else {
            GameLayer::UiDesktop
        }
    }

    /// Export comprehensive layered decompilation reports and JSON manifest
    pub fn export_layered_database(
        funcs: &[ClassifiedFunction],
        out_dir: &Path,
    ) -> std::io::Result<()> {
        std::fs::create_dir_all(out_dir)?;

        let mut layer_map: BTreeMap<GameLayer, Vec<&ClassifiedFunction>> = BTreeMap::new();
        for f in funcs {
            layer_map.entry(f.layer).or_default().push(f);
        }

        println!("\n=== Layered Game Subsystem Decompilation Manifest ===");
        for (&layer, list) in &layer_map {
            let total_bytes: u32 = list.iter().map(|f| f.size).sum();
            let total_insns: usize = list.iter().map(|f| f.instruction_count).sum();
            println!(
                "  [{}] -> {:>5} functions ({:>6.1} KB, {:>7} instructions)",
                layer.name(),
                list.len(),
                total_bytes as f32 / 1024.0,
                total_insns,
            );

            // Write individual layer report
            let layer_file = out_dir.join(format!("{}.txt", layer.slug()));
            let mut f = File::create(layer_file)?;
            writeln!(f, "=== .hack//G.U. Decompilation: {} ===", layer.name())?;
            writeln!(f, "Total Functions: {} | Total Size: {:.1} KB\n", list.len(), total_bytes as f32 / 1024.0)?;

            for func in list {
                let class_str = if let Some(ref c) = func.class_name {
                    format!(" [{}]::method_{:02}", c, func.virtual_method_index.unwrap_or(0))
                } else {
                    String::new()
                };
                writeln!(
                    f,
                    "Func 0x{:08X} (size: {:>4} bytes, insns: {:>3}, BBs: {:>2}){}",
                    func.rva, func.size, func.instruction_count, func.basic_block_count, class_str
                )?;
                if !func.string_xrefs.is_empty() {
                    writeln!(f, "    XRefs: {:?}", func.string_xrefs)?;
                }
            }
        }

        // Export JSON master index
        let json_path = out_dir.join("master_functions_manifest.json");
        let mut json_file = File::create(json_path)?;
        writeln!(json_file, "[")?;
        for (i, func) in funcs.iter().enumerate() {
            let comma = if i + 1 < funcs.len() { "," } else { "" };
            let class_escaped = func.class_name.as_deref().unwrap_or("").replace("\"", "\\\"");
            writeln!(
                json_file,
                "  {{\"rva\": \"0x{:08X}\", \"size\": {}, \"layer\": \"{}\", \"class\": \"{}\", \"method_idx\": {}, \"insn_count\": {}}}{}",
                func.rva, func.size, func.layer.slug(), class_escaped, func.virtual_method_index.map(|m| m as i32).unwrap_or(-1), func.instruction_count, comma
            )?;
        }
        writeln!(json_file, "]")?;

        Ok(())
    }
}
