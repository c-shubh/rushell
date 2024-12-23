use std::process::exit;

pub struct ExitCommand;

impl ExitCommand {
    pub fn execute(_: &[String]) -> i32 {
        exit(0)
    }
}
