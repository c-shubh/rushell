# Rushell

Rushell is a toy shell that's capable of interpreting shell commands, running external programs and builtin commands like cd, pwd, echo and more. It implements shell command parsing, REPLs, builtin commands, and more.

It is an exercise in learning Rust and follows the [Build Your Own Shell](https://app.codecrafters.io/courses/shell/overview) guide as a roadmap for features.

## Building Rushell

1. Clone the repository

   ```bash
   git clone https://github.com/c-shubh/rushell.git
   cd rushell
   ```

2. Build the project

   ```bash
   cargo build
   ```

3. Run Rushell

   ```bash
   cargo run
   ```

## Usage

Once launched, Rushell acts as an interactive shell where you can:

- Run commands like `ls`, `cat`, etc., if they are available in your system's PATH.
- Use builtin commands such as `cd`, `pwd`, and `echo`.

Although Rushell is developed on Windows 11, it strives to be cross-platform and takes care to ensure compatibility with other major operating systems.

## License

This project is licensed under the AGPLv3. See the [LICENSE](./LICENSE.md) file for details.
