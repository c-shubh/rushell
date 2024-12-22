use crate::command::Command;
use crate::echo_command::EchoCommand;
use crate::{exit_command::ExitCommand, line_parser::LineParser};
use std::io::{self, Write};

pub struct Shell;

impl Shell {
    pub fn new() -> Shell {
        Shell {}
    }

    pub fn repl(&self) {
        loop {
            self.print_prompt();
            let input = self.read_line();
            let args = LineParser::parse(input);
            if args.is_empty() {
                continue;
            }
            self.execute(&args);
        }
    }

    fn execute(&self, args: &Vec<String>) -> i32 {
        assert!(args.len() > 0);
        let command: &str = args.first().unwrap().as_str();
        match command {
            "exit" => ExitCommand::execute(args),
            "echo" => EchoCommand::execute(args),
            _ => {
                eprintln!("{command}: command not found");
                127
            }
        }
    }

    fn print_prompt(&self) {
        print!("$ ");
        io::stdout().flush().unwrap();
    }

    fn read_line(&self) -> String {
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}
