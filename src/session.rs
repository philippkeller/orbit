//! Last-session snapshot: queue, playback position, and library navigation.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::config;
use crate::library::{self, LibEntry, Library};
use crate::model::Track;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Session {
    #[serde(default)]
    pub library_cwd: Option<PathBuf>,
    #[serde(default)]
    pub library_filter: String,
    #[serde(default)]
    pub library_selection: Option<PathBuf>,
    #[serde(default)]
    pub queue_tracks: Vec<PathBuf>,
    #[serde(default)]
    pub queue_order: Vec<usize>,
    #[serde(default)]
    pub queue_order_pos: usize,
    #[serde(default)]
    pub now_playing: Option<PathBuf>,
    #[serde(default)]
    pub position_ms: u64,
    #[serde(default)]
    pub paused: bool,
    #[serde(default)]
    pub focus: SessionFocus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum SessionFocus {
    #[default]
    Library,
    Buckets,
    Queue,
}

pub fn load() -> Option<Session> {
    fs::read_to_string(config::session_file())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
}

pub fn save(session: &Session) {
    if let Ok(json) = serde_json::to_string_pretty(session) {
        let path = config::session_file();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        fs::write(path, json).ok();
    }
}

pub fn resolve_track(path: &Path, library: &Library) -> Option<Track> {
    library
        .tracks
        .iter()
        .find(|t| t.path == path)
        .cloned()
        .or_else(|| library::read_track(path))
}

pub fn find_library_row(library: &Library, path: &Path) -> Option<usize> {
    for (row, entry) in library.entries.iter().enumerate() {
        match entry {
            LibEntry::Track(i) => {
                if library.track(*i).map(|t| t.path.as_path()) == Some(path) {
                    return Some(row);
                }
            }
            LibEntry::Folder { path: folder, .. } if folder == path => return Some(row),
            LibEntry::Parent => {
                if library.cwd() == Some(path) {
                    return Some(row);
                }
            }
            _ => {}
        }
    }
    None
}
