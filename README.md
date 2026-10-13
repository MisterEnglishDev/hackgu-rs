# hackgu-rs

An open-source Rust reimplementation of **.hack//G.U. Last Recode** (Vol.1 so far), built from the ground up to enable modern modding, high frame rates, native rendering, and game preservation.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust 2021](https://img.shields.io/badge/Rust-2021%20Edition-orange.svg)](https://www.rust-lang.org/)

---

## ⚠️ Important Legal & Bring-Your-Own-Game (BYOG) Policy

* **Zero Copyrighted Assets or Binaries Distributed**: This repository contains **NO** game assets, copyrighted textures, 3D models, audio files, movies, dialogue scripts, or original proprietary executables.
* **Bring Your Own Game**: You must supply your own legitimately owned copy of *.hack//G.U. Last Recode* (Steam / PC). The setup script reads it from your install; nothing from the game is in this repository.
* **Non-Commercial**: This is an independent, non-commercial research and preservation project not affiliated with Bandai Namco Entertainment Inc. or CyberConnect2 Co., Ltd. See [LEGAL.md](LEGAL.md) for full intellectual property and trademark disclaimers.

---

## Quick Start: Getting Started

### 1. Prerequisites
- **[Rust & Cargo](https://rustup.rs/)** (Rust 1.75+ recommended, 2021 edition)
- **[Python 3.8+](https://www.python.org/)** (asset extraction; the script installs the `capstone` and `unicorn` packages it needs)
- **[FFmpeg](https://ffmpeg.org/)** on your PATH (`ffmpeg` and `ffprobe`, used to play the game's movies)
- A legitimate copy of **.hack//G.U. Last Recode** (Steam / PC).

### 2. Extract Game Assets (One-Click Setup)

Run the included extraction script to unpack game assets from your copy into the local workspace:

* **Option A — Drag & Drop**:
  Drag your Steam `hackGU` game folder (or any `.cpk` inside its `cpk` folder) onto **`Extract_Game_Assets.bat`** in Windows Explorer.
* **Option B — Auto-Detection**:
  Double-click **`Extract_Game_Assets.bat`** (or run `python tools/extract_assets.py`). The tool will automatically scan standard Steam library locations for your install and prompt to extract.
* **Option C — Command Line**:
  ```bash
  python tools/extract_assets.py "F:\SteamLibrary\steamapps\common\hackGU"
  ```

The script unpacks the Vol.1 and common archives into `target/`, remembers your install folder in `game_dir.txt`
(sound and movies are read from the install while playing), and generates the data the port computes from the game's
own code (event locations, story dungeon layouts) from your copy's `hackGU_vol1.dll`. It needs about 3 GB of space.

#### PS2 discs
**Not supported yet.** The port is built on the Last Recode files and game code; the PS2 releases use older file
formats and different code. The extractor can copy the files off a PS2 `.iso`, but the game cannot run from them.

### 3. Build & Run

Double-click **`play_game.bat`**. It builds the game, then starts it.

The **first build takes several minutes** (5-15 on most PCs) and keeps the CPU busy while Rust compiles everything;
cargo shows its progress in the window. Later launches only rebuild what changed and start in seconds.
```cmd
play_game.bat
```
Or build and run with Cargo:
```bash
cargo run --release -p hackgu-render
```

---

## Workspace Architecture

The project is structured as a modular Cargo workspace:

| Crate | Purpose |
| :--- | :--- |
| `crates/hackgu-render` | Modern `wgpu` real-time renderer, swapchain management, camera systems, and game loop |
| `crates/hackgu-game` | High-level game state, event scripting (`ev.rs`), dialogue playback, and actor positioning |
| `crates/hackgu-world` | World assembly, locator parsing (`DMY_*`), stage geometry, and collision meshes (`HitMesh`) |
| `crates/hackgu-ccs` | CyberConnect2 `.ccs` (CCSF) container parser, model geometry decoder, texture decoders, and OBJ export |
| `crates/hackgu-cpk` | CRI CPK archive reader, extractor, XOR table decryptor, and CRILAYLA decompressor |
| `crates/hackgu-vfs` | Virtual File System with on-the-fly CPK mounting and priority-based directory mod overlay shadowing |
| `crates/hackgu-engine` | Core math, transforms, quaternion conjugation, and engine utilities |
| `crates/survey` | Static analysis tooling for game installs: magic bytes, entropy, engine hints, PE analysis |

---

## CLI Tools & Utilities

### Inspecting and Extracting CPK Archives (`hackgu-cpk`)
```bash
# Archive metadata and compression stats
cargo run --release -p hackgu-cpk -- info "<path to .cpk>"

# List all files inside the archive
cargo run --release -p hackgu-cpk -- list "<path to .cpk>" [--filter <name>]

# Extract archive
cargo run --release -p hackgu-cpk -- extract "<path to .cpk>" --out <dest_dir>
```

### Inspecting & Exporting CyberConnect2 Assets (`hackgu-ccs`)
```bash
# Container header and object hierarchy breakdown
cargo run --release -p hackgu-ccs -- info <file.ccs>

# List declared subfiles and embedded textures/models
cargo run --release -p hackgu-ccs -- list-objects <file.ccs>

# Export all textures to 32-bit BMP or DDS
cargo run --release -p hackgu-ccs -- dump-textures <file.ccs> --out <dir> [--dds]

# Export 32-bit 3D models to Wavefront OBJ with UVs and normals
cargo run --release -p hackgu-ccs -- dump-models <file.ccs> --out <dir>
```

---

## Supporting the Project (Patreon & Sponsorships)

If you find this research and preservation effort valuable and would like to support ongoing development:

* **[Patreon](https://www.patreon.com/c/MisterEnglishDev)** *(Community Tip Jar)*
* **Zero Paywalled Content**: All code, tools, documentation, and features are completely free and open-source on GitHub. Patreon contributions serve solely as voluntary support to offset compute/AI tooling subscriptions, reverse-engineering software, test hardware, and development time.

---

## Roadmap

- [x] **1. Survey & Architecture**: Reverse engineered CPK archives, middleware, and entry points.
- [x] **2. CPK Archive Unpacker**: Pure-Rust parser for CRI CPK containers, XOR `@UTF` decryption, and CRILAYLA decompressor.
- [x] **3. CCS Container Parser**: CyberConnect2 `CCSF` container parser for textures, clumps, models, materials, animations.
- [x] **4. Layered Virtual File System (`hackgu-vfs`)**: Priority-based virtual layering with loose mod directory shadowing.
- [x] **5. Texture & Model Decoders**: DXT1/DXT5/RGBA32 textures, rigid models, deformable skinned models, triangle strip reconstruction.
- [x] **6. HitMesh Collision System**: Surface flags, walls, floor height checks, follow-camera boom clipping.
- [x] **7. Town Assembly & Real-Time Rendering**: Mac Anu prop layout, reverse-Z depth, vertex colors, alpha decal and additive blend modes.
- [x] **8. Event & Dialogue Cutscene Engine**: The game's own event scripts drive cameras, cast, motions, voices and music.
- [x] **9. Text & Localization**: `messageInfo.bin` parsed; English text with the game's markup and button glyphs.
- [x] **10. Opening to the first dungeon**: Title, ALTIMIT desktop, Mac Anu, Chaos Gate, the tutorial field, and the
  Peaceful Leading Freedom dungeon built by the game's own layout code (both floors and its story scenes).
- [ ] **11. Combat**: The tutorial fights work with approximate numbers; the real battle formulas and dungeon enemies are next.
- [ ] **12. The rest of Vol.1**, then Vols. 2 and 3.

---

## License

This project is licensed under the [MIT License](LICENSE).
See [LEGAL.md](LEGAL.md) for intellectual property and trademark acknowledgments.
