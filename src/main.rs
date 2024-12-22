mod command;
mod echo_command;
mod exit_command;
mod line_parser;
mod shell;
use shell::Shell;

fn main() {
    let shell = Shell::new();
    shell.repl();
}
