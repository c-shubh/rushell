use crate::echo_command::EchoCommand;
use crate::type_command::TypeCommand;
use crate::{exit_command::ExitCommand, line_parser::LineParser};
use std::collections::HashSet;
use std::env;
use std::io::{self, Write};
use std::process::Command;

pub struct Shell {
    built_in_commands: HashSet<String>,
    env_path: Vec<String>,
}

impl Shell {
    pub fn new() -> Shell {
        let built_in_commands = HashSet::from(["exit", "echo", "type"].map(str::to_string));
        let mut env_path: Vec<String> = Vec::new();
        if let Ok(path) = env::var("PATH") {
            let split_by: &str = match env::consts::OS {
                "windows" => ";",
                _ => ":",
            };
            path.split(split_by)
                .for_each(|p| env_path.push(p.to_string()));
        }
        Shell {
            built_in_commands,
            env_path,
        }
    }

    pub fn run(&self) {
        self.repl();
    }

    fn repl(&self) {
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

    fn execute(&self, args: &[String]) -> i32 {
        assert!(!args.is_empty());
        let command: &str = args.first().unwrap().as_str();

        if self.built_in_commands.contains(command) {
            self.execute_built_in(command, args)
        } else {
            self.execute_external(command, args)
        }
    }

    fn command_not_found(&self, command: &str) -> i32 {
        eprintln!("{command}: command not found");
        127
    }

    fn execute_built_in(&self, command: &str, args: &[String]) -> i32 {
        return match command {
            "exit" => ExitCommand::execute(args),
            "echo" => EchoCommand::execute(args),
            "type" => TypeCommand::execute(args, &self.built_in_commands, &self.env_path),
            _ => self.command_not_found(&command),
        };
    }

    fn execute_external(&self, command: &str, args: &[String]) -> i32 {
        let cmd = Command::new(command).args(&args[1..]).output();
        if let Ok(output) = cmd {
            io::stdout().write(&output.stdout).unwrap();
            io::stdout().flush().unwrap();
            io::stderr().write(&output.stderr).unwrap();
            io::stderr().flush().unwrap();

            return match output.status.code() {
                Some(code) => code,
                None => {
                    // TODO: what do we return here?
                    1
                }
            };
        } else {
            return self.command_not_found(command);
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
