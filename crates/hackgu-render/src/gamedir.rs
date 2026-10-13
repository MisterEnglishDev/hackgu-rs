//! Where the player's own copy of .hack//G.U. Last Recode is installed. Sounds, movies and the game DLL are read from
//! there at run time (they are not copied into the project). Set by the asset extractor (game_dir.txt in the project
//! folder), overridable with the HACKGU_GAME_DIR environment variable; otherwise common Steam folders are tried.

use std::path::PathBuf;
use std::sync::OnceLock;

const STEAM_GUESSES: [&str; 6] = [
    r"C:\Program Files (x86)\Steam\steamapps\common\hackGU",
    r"C:\SteamLibrary\steamapps\common\hackGU",
    r"D:\SteamLibrary\steamapps\common\hackGU",
    r"E:\SteamLibrary\steamapps\common\hackGU",
    r"F:\SteamLibrary\steamapps\common\hackGU",
    r"G:\SteamLibrary\steamapps\common\hackGU",
];

fn is_install(p: &std::path::Path) -> bool {
    p.join("cmn").is_dir() || p.join("hackGU_vol1.dll").is_file()
}

/// The game install folder (the one containing hackGU.exe, cpk\ and cmn\).
pub fn game_dir() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        if let Ok(v) = std::env::var("HACKGU_GAME_DIR") {
            return PathBuf::from(v);
        }
        if let Ok(s) = std::fs::read_to_string("game_dir.txt") {
            let p = PathBuf::from(s.trim());
            if is_install(&p) {
                return p;
            }
        }
        for g in STEAM_GUESSES {
            let p = PathBuf::from(g);
            if is_install(&p) {
                return p;
            }
        }
        eprintln!("game install not found: run Extract_Game_Assets.bat or set HACKGU_GAME_DIR (sound and movies are off)");
        PathBuf::from("game_not_found")
    })
    .clone()
}

/// A path inside the game install, e.g. `game_path("cmn/movie/vol1")`.
pub fn game_path(rel: &str) -> PathBuf {
    game_dir().join(rel)
}
