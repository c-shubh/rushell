use std::env;

pub struct PwdCommand;

impl PwdCommand {
    pub fn execute(_args: &[String]) -> i32 {
        if let Ok(dir) = env::current_dir() {
            eprintln!("{}", dir.display());
            return 0;
        }
        1
    }
}
