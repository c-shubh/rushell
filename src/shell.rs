use crate::cd_command::CdCommand;
use crate::echo_command::EchoCommand;
use crate::pwd_command::PwdCommand;
use crate::type_command::TypeCommand;
use crate::{exit_command::ExitCommand, line_parser::LineParser};
use std::collections::HashSet;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

pub struct Shell {
    built_in_commands: HashSet<String>,
    env_path: Vec<String>,
    current_dir: PathBuf,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            built_in_commands: Shell::get_built_in_commands(),
            env_path: Shell::get_env_path(),
            current_dir: Shell::get_current_dir(),
        }
    }

    fn get_current_dir() -> PathBuf {
        match env::current_dir() {
            Ok(dir) => dir,
            Err(_) => PathBuf::from("/"),
        }
    }

    fn get_env_path() -> Vec<String> {
        let mut env_path: Vec<String> = Vec::new();
        if let Ok(path) = env::var("PATH") {
            let split_by: &str = match env::consts::OS {
                "windows" => ";",
                _ => ":",
            };
            path.split(split_by)
                .for_each(|p| env_path.push(p.to_string()));
        }
        env_path
    }

    pub fn run(&mut self) {
        self.repl();
    }

    fn repl(&mut self) {
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

    fn execute(&mut self, args: &[String]) -> i32 {
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

    fn get_built_in_commands() -> HashSet<String> {
        HashSet::from(["exit", "echo", "type", "pwd", "cd"].map(str::to_string))
    }

    fn execute_built_in(&mut self, command: &str, args: &[String]) -> i32 {
        match command {
            "exit" => ExitCommand::execute(args),
            "echo" => EchoCommand::execute(args),
            "type" => TypeCommand::execute(args, &self.built_in_commands, &self.env_path),
            "pwd" => PwdCommand::execute(args, &self.current_dir),
            "cd" => CdCommand::execute(args, &mut self.current_dir),
            _ => self.command_not_found(command),
        }
    }

    fn execute_external(&self, command: &str, args: &[String]) -> i32 {
        let cmd = Command::new(command).args(&args[1..]).output();
        if let Ok(output) = cmd {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stdout().flush().unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            io::stderr().flush().unwrap();

            output.status.code().unwrap_or(
                // TODO: what do we return when status code is None
                1,
            )
        } else {
            self.command_not_found(command)
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
