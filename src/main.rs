mod echo_command;
mod exit_command;
mod line_parser;
mod pwd_command;
mod shell;
mod type_command;
use shell::Shell;

fn main() {
    let shell = Shell::new();
    shell.run();
}
