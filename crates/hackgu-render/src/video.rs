//! # CRI Sofdec USM Movie Player & Cutscene Orchestrator
//! Plays authentic full-motion video cutscenes (.usm) from .hack//G.U. Last Recode
//! with native video decoding, 5.1/stereo audio, and full-screen presentation.

use std::path::PathBuf;
use std::process::{Child, Command};

pub struct MovieController {
    pub current_process: Option<Child>,
    pub current_movie: Option<String>,
}

impl MovieController {
    pub fn new() -> Self {
        Self {
            current_process: None,
            current_movie: None,
        }
    }

    /// Resolve absolute path to a .usm video file across game installations and directories.
    pub fn resolve_movie(movie_name: &str) -> Option<PathBuf> {
        let candidate_dirs = [
            PathBuf::from(crate::gamedir::game_path("cmn/movie/vol1/ev")),
            PathBuf::from(crate::gamedir::game_path("cmn/movie/vol1")),
            PathBuf::from(crate::gamedir::game_path("cmn/movie/vol1/online/US")),
            PathBuf::from(crate::gamedir::game_path("cmn/movie/vol1/news/US")),
            PathBuf::from(r"cmn\movie\vol1\ev"),
            PathBuf::from(r"cmn\movie\vol1"),
            PathBuf::from(r"target\movies"),
            PathBuf::from(r"movies"),
        ];

        for dir in &candidate_dirs {
            let p = dir.join(movie_name);
            if p.exists() && p.is_file() {
                return Some(p);
            }
        }
        None
    }

    /// Play an authentic .usm cutscene video using the native ffplay engine.
    pub fn play(&mut self, movie_name: &str) -> Result<(), String> {
        // Stop any currently running movie first
        self.stop();

        let path = Self::resolve_movie(movie_name)
            .ok_or_else(|| format!("Movie file '{}' not found in game media archives", movie_name))?;

        println!("🎬 Launching authentic full-motion cutscene: {}", path.display());

        let mut cmd = Command::new("ffplay");
        cmd.arg("-autoexit")
            .arg("-fs")
            .arg("-noborder")
            .arg("-window_title")
            .arg(format!(".hack//G.U. Vol. 1 - {}", movie_name))
            .arg("-af")
            .arg("aformat=channel_layouts=stereo,volume=0.82")
            .arg(&path);

        match cmd.spawn() {
            Ok(child) => {
                self.current_process = Some(child);
                self.current_movie = Some(movie_name.to_string());
                Ok(())
            }
            Err(e) => {
                let err_msg = format!("Failed to launch ffplay player for '{}': {}", movie_name, e);
                eprintln!("{}", err_msg);
                Err(err_msg)
            }
        }
    }

    /// Check if a cutscene movie is currently playing.
    pub fn is_playing(&mut self) -> bool {
        if let Some(ref mut child) = self.current_process {
            match child.try_wait() {
                Ok(Some(_)) => {
                    self.current_process = None;
                    self.current_movie = None;
                    false
                }
                Ok(None) => true,
                Err(_) => {
                    self.current_process = None;
                    self.current_movie = None;
                    false
                }
            }
        } else {
            false
        }
    }

    /// Poll if movie finished playing this tick.
    pub fn poll_finished(&mut self) -> bool {
        if let Some(ref mut child) = self.current_process {
            match child.try_wait() {
                Ok(Some(status)) => {
                    println!("🎬 Cutscene {:?} finished with status {:?}", self.current_movie, status);
                    self.current_process = None;
                    let _ = self.current_movie.take();
                    true
                }
                Ok(None) => false,
                Err(_) => {
                    self.current_process = None;
                    let _ = self.current_movie.take();
                    true
                }
            }
        } else {
            false
        }
    }

    /// Abort / skip currently playing movie immediately.
    pub fn stop(&mut self) {
        if let Some(mut child) = self.current_process.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        self.current_movie = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_prologue_movies_resolve() {
        let movies = [
            ("OP10000.usm", "Vol. 1 Opening Cinematic"),
            ("MM10145.usm", "Ovan's Intervention (Savior)"),
            ("MM10210.usm", "Six Months Later: The Terror of Death"),
            ("MM10220.usm", "Confrontation with Iyoten & Asta in Mac Anu"),
            ("MM10230.usm", "The Cathedral of Hulle Granz (Tri-Edge Appears)"),
            ("MM10240.usm", "Battle with Tri-Edge & The Data Drain"),
        ];

        for (file_name, desc) in &movies {
            let path = MovieController::resolve_movie(file_name)
                .unwrap_or_else(|| panic!("Prologue movie '{}' ({}) must resolve to disk", file_name, desc));
            assert!(path.exists() && path.is_file(), "Movie file must exist: {}", path.display());
            let meta = std::fs::metadata(&path).unwrap();
            assert!(meta.len() > 10_000_000, "Movie file '{}' should be a valid media file (>10MB)", file_name);
            println!("Verified authentic cutscene {}: {} ({:.1} MB)", file_name, desc, meta.len() as f64 / (1024.0 * 1024.0));
        }
    }
}

