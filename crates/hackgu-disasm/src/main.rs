pub mod pe;
pub mod scanner;
pub mod disasm;
pub mod rtti;
pub mod codegen;
pub mod layers;

use std::fs::File;
use std::io::Write;
use pe::PeFile;
use scanner::SymbolScanner;
use disasm::CodeAnalyzer;
use rtti::RttiParser;
use codegen::RustCodeGen;
use layers::LayerManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let default_dll = "F:\\SteamLibrary\\steamapps\\common\\hackGU\\hackGU_vol1.dll";
    let target_path = if args.len() > 1 {
        &args[1]
    } else {
        default_dll
    };

    println!("=== .hack//G.U. Binary Decompiler & Reverse-Engineering Tool ===");
    println!("Analyzing target: {}", target_path);

    let data = std::fs::read(target_path)?;
    let pe = PeFile::parse(&data)?;

    println!("PE Image Base: 0x{:016x}", pe.image_base);
    println!("Entry Point RVA: 0x{:08x}", pe.entry_point);
    println!("Sections ({}):", pe.sections.len());
    for s in &pe.sections {
        println!(
            "  [{:<8}] VAddr: 0x{:08x} VSize: 0x{:08x} RawSize: 0x{:08x} Char: 0x{:08x}",
            s.name, s.virtual_address, s.virtual_size, s.size_of_raw_data, s.characteristics
        );
    }

    println!("\nExported Functions ({}):", pe.exports.len());
    for e in pe.exports.iter().take(20) {
        println!("  #{} {:<30} @ RVA 0x{:08x}", e.ordinal, e.name.as_deref().unwrap_or(""), e.rva);
    }

    println!("\nImported DLLs & Functions ({}):", pe.imports.len());
    let mut import_counts = std::collections::BTreeMap::new();
    for imp in &pe.imports {
        *import_counts.entry(&imp.dll_name).or_insert(0) += 1;
    }
    for (dll, count) in &import_counts {
        println!("  {:<25} ({} functions)", dll, count);
    }

    // 1. Scan and Reconstruct RTTI Class Hierarchy and VTables
    println!("\n[1/4] Parsing MSVC x64 RTTI Class Hierarchy & VTables...");
    let rtti_classes = RttiParser::parse_classes(&pe);
    println!("Successfully resolved {} C++ classes with inheritance & vtables:", rtti_classes.len());
    let mut vtable_count = 0;
    for (name, cls) in &rtti_classes {
        if cls.vftable_rva.is_some() {
            vtable_count += 1;
        }
        if name.contains("Gobj") || name.contains("ccGeom") || name.contains("ccClump") || name.contains("ccEvent") {
            let base_str = if cls.base_classes.is_empty() {
                String::new()
            } else {
                format!(" : {}", cls.base_classes.join(", "))
            };
            let vt_str = if let Some(va) = cls.vftable_rva {
                format!(" [vftable 0x{:08x}, {} virtual methods]", va, cls.virtual_methods.len())
            } else {
                String::new()
            };
            println!("  class {}{}{}", name, base_str, vt_str);
        }
    }
    println!("  Total classes with primary vtables: {}", vtable_count);

    // 2. Scan for engine symbols, source file paths, and format strings
    println!("\n[2/4] Scanning Engine Strings & Source Symbols...");
    let strings = SymbolScanner::scan_strings(&pe, 5);
    let symbol_strings: Vec<_> = strings.iter().filter(|s| s.is_symbol_like).collect();
    println!("Found {} engine-relevant symbols and strings:", symbol_strings.len());
    for s in symbol_strings.iter().take(25) {
        println!("  0x{:08x}: \"{}\"", s.rva, s.value);
    }

    // 3. Scan code section (.text) for function boundaries & disassemble virtual methods
    println!("\n[3/4] Analyzing .text Executable Code & Disassembling Engine Functions...");
    let analyzer = pe.get_section(".text").map(|(sec, code)| {
        CodeAnalyzer::new(code, sec.virtual_address, pe.image_base)
    });

    if let Some(ref ana) = analyzer {
        let funcs = ana.find_functions();
        println!("Identified {} function entrypoints in .text:", funcs.len());

        // Decompile key player/actor virtual methods using iced-x86
        if let Some(pc_cls) = rtti_classes.get(".?AVccGobjPc@@").or_else(|| rtti_classes.get("ccGobjPc")) {
            println!("\n  === Disassembling Player Character (ccGobjPc) Virtual Methods ===");
            for (m_idx, &m_va) in pc_cls.virtual_methods.iter().take(5).enumerate() {
                let m_rva = (m_va - pe.image_base) as u32;
                if let Some(decomp) = ana.disassemble_function(m_rva, 512) {
                    println!("  [Method #{}] RVA 0x{:08X} ({} instructions, {} basic blocks, stack: {} bytes):",
                        m_idx, m_rva, decomp.instructions.len(), decomp.basic_blocks.len(), decomp.stack_size);
                    println!("    Field accesses on 'this' (RCX):");
                    for fa in decomp.field_accesses.iter().take(6) {
                        let op = if fa.is_write { "Write" } else { "Read" };
                        let kind = if fa.is_float { "float" } else { "int/ptr" };
                        println!("      +0x{:04X} [{} bytes, {}, {}]", fa.offset, fa.size, op, kind);
                    }
                    let lines = CodeAnalyzer::format_instructions(&decomp.instructions[..decomp.instructions.len().min(6)]);
                    for l in lines {
                        println!("      {}", l);
                    }
                }
            }
        }
    }

    // 4. Write full decompilation report
    let out_report = "decomp_vol1_report.txt";
    let mut file = File::create(out_report)?;
    writeln!(file, "=== .hack//G.U. Decompilation Analysis: {} ===", target_path)?;
    writeln!(file, "Image Base: 0x{:016x}", pe.image_base)?;
    writeln!(file, "\n--- C++ Classes & Inheritance ({}) ---", rtti_classes.len())?;
    for (name, cls) in &rtti_classes {
        let base_str = if cls.base_classes.is_empty() {
            String::new()
        } else {
            format!(" : {}", cls.base_classes.join(", "))
        };
        let vt_str = if let Some(va) = cls.vftable_rva {
            format!(" [vftable 0x{:08x}, {} virtual methods]", va, cls.virtual_methods.len())
        } else {
            String::new()
        };
        writeln!(file, "class {}{}{}", name, base_str, vt_str)?;
        for (m_idx, &m_va) in cls.virtual_methods.iter().enumerate() {
            writeln!(file, "    [{:>2}] 0x{:016x} (RVA: 0x{:08x})", m_idx, m_va, m_va - pe.image_base)?;
        }
    }
    writeln!(file, "\n--- Engine Strings ({}) ---", symbol_strings.len())?;
    for s in &symbol_strings {
        writeln!(file, "0x{:08x}: {}", s.rva, s.value)?;
    }
    println!("\nWrote detailed decompilation report to: {}", out_report);

    // 5. Generate pure Rust source files from decompiled CC2 engine classes with recovered fields
    println!("\n[4/5] Generating Rust Engine Crate from Decompiled CC2 Classes & Recovered Fields...");
    let rust_out = std::path::Path::new("crates/hackgu-engine/src");
    RustCodeGen::generate_rust_crate(&rtti_classes, analyzer.as_ref(), pe.image_base, rust_out)?;
    println!("Successfully generated Rust CC2 engine crate at: {}", rust_out.display());

    // 6. Partition all 17k functions into 7 game layers and export master database
    if let Some(ref ana) = analyzer {
        println!("\n[5/5] Partitioning & Cataloguing all 17,237 Functions into 7 Game Subsystem Layers...");
        let funcs = ana.find_functions();
        let classified = LayerManager::classify_all(&pe, &funcs, &rtti_classes, &strings, ana);
        let layers_dir = std::path::Path::new("decomp/layers");
        LayerManager::export_layered_database(&classified, layers_dir)?;
        println!("Successfully exported 7-layer game manifest and function catalog to: {}", layers_dir.display());
    }

    Ok(())
}
