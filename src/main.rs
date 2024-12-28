mod cd_command;
mod echo_command;
mod exit_command;
mod pwd_command;
mod scanner;
mod shell;
mod token;
mod type_command;
mod utils;
use shell::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run();
}
