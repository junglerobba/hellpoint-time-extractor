#![feature(is_some_with)]

use serde::{Deserialize, Serialize};
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct HellpointSave {
    pub name: String,
    #[serde(rename = "totalTime")]
    pub total_time: usize,
    pub player: Player,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Player {
    pub stats: Vec<isize>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let name = args.get(1);

    let dir = get_save_dir().unwrap();
    let paths = fs::read_dir(dir).unwrap();

    let mut paths: Vec<DirEntry> = paths
        .filter_map(|path| path.ok())
        .filter(|file| file.path().extension().is_some_and(|ext| *ext == "hp"))
        .collect();

    paths.sort_by(|a, b| {
        let a = fs::metadata(a.path()).unwrap().modified().unwrap();
        let b = fs::metadata(b.path()).unwrap().modified().unwrap();
        b.cmp(&a)
    });

    let saves: Vec<HellpointSave> = paths
        .iter()
        .map(|path| {
            let data = fs::read_to_string(path.path()).unwrap();
            let json: HellpointSave = serde_json::from_str(&data).unwrap();
            json
        })
        .collect();

    let save = name
        .map_or_else(
            || saves.first(),
            |name| saves.iter().find(|save| save.name == *name),
        )
        .expect("No save found");

    let total_time = save.total_time;
    let hours = total_time / 3600;
    let minutes = (total_time % 3600) / 60;
    let seconds = total_time - (hours * 3600) - (minutes * 60);

    println!(
        "{} (Level {}) {}:{}:{}",
        save.name,
        get_player_level(&save.player),
        pad_zeroes(hours, 2),
        pad_zeroes(minutes, 2),
        pad_zeroes(seconds, 2)
    );
}

fn get_save_dir() -> Option<PathBuf> {
    let home = dirs::home_dir().unwrap();
    if cfg!(windows) {
        let path = home.join(Path::new("AppData\\LocalLow\\Cradle Games\\Hellpoint"));
        if path.exists() {
            return Some(path);
        } else {
            return None;
        }
    }

    let config_dir = std::env::var("XDG_CONFIG_HOME").map_or_else(
        |_| home.join(Path::new(".config")),
        |config| Path::new(&config).to_path_buf(),
    );

    let linux = config_dir.join(Path::new("unity3d/Cradle Games/Hellpoint"));
    let flatpak =
        home.join(".var/app/com.valvesoftware.Steam/config/unity3d/Cradle Games/Hellpoint");

    if linux.exists() {
        Some(linux)
    } else if flatpak.exists() {
        Some(flatpak)
    } else {
        None
    }
}

fn get_player_level(player: &Player) -> isize {
    player.stats.iter().fold(-7, |acc, x| acc + x)
}

fn pad_zeroes(time: usize, length: usize) -> String {
    let str_length = time.to_string().chars().count();
    if str_length >= length {
        return format!("{}", time);
    }
    let count = length - str_length;
    let zeroes = "0".repeat(count);
    format!("{}{}", zeroes, time)
}
