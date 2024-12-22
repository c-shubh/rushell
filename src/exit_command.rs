use crate::command::Command;
use std::process::exit;

pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(_: &Vec<String>) -> i32 {
        exit(0)
    }
}
