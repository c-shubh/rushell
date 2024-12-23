use std::collections::HashSet;

pub struct TypeCommand;

impl TypeCommand {
    pub fn execute(args: &[String], built_in_commands: &HashSet<String>) -> i32 {
        let mut return_code: i32 = 0;
        for arg in args.iter().skip(1) {
            if built_in_commands.contains(arg) {
                println!("{} is a shell builtin", arg);
            } else {
                println!("{}: not found", arg);
                return_code = 1;
            }
        }
        return_code
    }
}
