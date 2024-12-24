use std::path::PathBuf;

pub struct PwdCommand;

impl PwdCommand {
    pub fn execute(_args: &[String], current_dir: &PathBuf) -> i32 {
        println!("{}", current_dir.display());
        0
    }
}
