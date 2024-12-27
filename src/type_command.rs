use std::{collections::HashSet, fs};

pub struct TypeCommand;

impl TypeCommand {
    pub fn execute(
        args: &[String],
        built_in_commands: &HashSet<String>,
        env_path: &[String],
    ) -> i32 {
        let mut return_code: i32 = 0;
        for arg in args.iter().skip(1) {
            if built_in_commands.contains(arg) {
                eprintln!("{} is a shell builtin", arg);
            } else if let Some(file_path) = TypeCommand::check_in_path(env_path, arg) {
                eprintln!("{} is {}", arg, file_path);
            } else {
                eprintln!("{}: not found", arg);
                return_code = 1;
            }
        }
        return_code
    }

    fn check_in_path(env_path: &[String], command: &String) -> Option<String> {
        for path in env_path {
            for item in fs::read_dir(path).unwrap().flatten() {
                let item_path = item.path();
                let file_name = item_path.file_stem().unwrap().to_str().unwrap();
                if file_name == command {
                    return Some(item_path.to_str().unwrap().to_string());
                }
            }
        }
        None
    }
}
