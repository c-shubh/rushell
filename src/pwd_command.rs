use std::path::Path;

pub struct PwdCommand;

impl PwdCommand {
    pub fn execute(_args: &[String], current_dir: &Path) -> i32 {
        println!("{}", current_dir.display());
        0
    }
}
