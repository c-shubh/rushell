use std::{env, path::PathBuf};

pub fn home_dir() -> Option<PathBuf> {
    match env::consts::FAMILY {
        "windows" => env::var("USERPROFILE").ok().map(PathBuf::from),
        "unix" => env::var("HOME").ok().map(PathBuf::from),
        _ => None,
    }
}
