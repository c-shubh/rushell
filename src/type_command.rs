use std::collections::HashSet;

pub struct TypeCommand;

impl TypeCommand {
    pub fn execute(args: &Vec<String>, built_in_commands: &HashSet<String>) -> i32 {
        let mut return_code: i32 = 0;
        for i in 1..args.len() {
            if built_in_commands.contains(&args[i]) {
                println!("{} is a shell builtin", args[i]);
            } else {
                println!("{}: not found", args[i]);
                return_code = 1;
            }
        }
        return return_code;
    }
}
