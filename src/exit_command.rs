use std::process::exit;

pub struct ExitCommand;

impl ExitCommand {
    pub fn execute(_: &Vec<String>) -> i32 {
        exit(0)
    }
}
