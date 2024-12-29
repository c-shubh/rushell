use crate::cd_command::CdCommand;
use crate::echo_command::EchoCommand;
use crate::exit_command::ExitCommand;
use crate::pwd_command::PwdCommand;
use crate::scanner::Scanner;
use crate::token::TokenType;
use crate::type_command::TypeCommand;
use std::collections::HashSet;
use std::io::{stderr, stdin, stdout, BufRead, BufReader, Write};
use std::process::Command;

pub struct Shell {
    built_in_commands: HashSet<String>,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            built_in_commands: Shell::get_built_in_commands(),
        }
    }

    pub fn main(&self) {
        self.run_prompt();
    }

    fn run_prompt(&self) {
        let input = stdin().lock();
        let mut reader = BufReader::new(input);

        loop {
            print!("$ ");
            stdout().flush().unwrap();
            let mut line: String = String::new();
            if reader.read_line(&mut line).is_err() {
                break;
            }
            line = line.trim().to_string();
            self.run(line);
        }
    }

    fn run(&self, source: String) {
        let mut scanner = Scanner::new(source);
        let scanned_tokens = scanner.scan_tokens();

        match scanned_tokens {
            Ok(scanned_tokens) => {
                let args: Vec<String> = scanned_tokens
                    .iter()
                    .filter(|token| token.type_ != TokenType::Eof)
                    .map(|token| token.lexeme.clone())
                    .collect();
                if args.is_empty() {
                    return;
                }
                self.execute(&args);
            }
            Err(e) => eprintln!("{}", e),
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

    fn get_built_in_commands() -> HashSet<String> {
        HashSet::from(["exit", "echo", "type", "pwd", "cd"].map(str::to_string))
    }

    fn execute_built_in(&self, command: &str, args: &[String]) -> i32 {
        match command {
            "exit" => ExitCommand::execute(args),
            "echo" => EchoCommand::execute(args),
            "type" => TypeCommand::execute(args, &self.built_in_commands),
            "pwd" => PwdCommand::execute(args),
            "cd" => CdCommand::execute(args),
            _ => self.command_not_found(command),
        }
    }

    fn execute_external(&self, command: &str, args: &[String]) -> i32 {
        let cmd = Command::new(command).args(&args[1..]).output();
        if let Ok(output) = cmd {
            stdout().write_all(&output.stdout).unwrap();
            stdout().flush().unwrap();
            stderr().write_all(&output.stderr).unwrap();
            stderr().flush().unwrap();

            output.status.code().unwrap_or(
                // TODO: what do we return when status code is None
                1,
            )
        } else {
            self.command_not_found(command)
        }
    }
}
