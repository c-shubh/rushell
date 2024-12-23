use crate::echo_command::EchoCommand;
use crate::type_command::TypeCommand;
use crate::{exit_command::ExitCommand, line_parser::LineParser};
use std::collections::HashSet;
use std::io::{self, Write};

pub struct Shell {
    built_in_commands: HashSet<String>,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            built_in_commands: HashSet::from(["exit", "echo", "type"].map(str::to_string)),
        }
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
            "type" => TypeCommand::execute(args, &self.built_in_commands),
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
