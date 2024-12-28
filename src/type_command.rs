use std::{collections::HashSet, env, fs};

pub struct TypeCommand;

impl TypeCommand {
    pub fn execute(args: &[String], built_in_commands: &HashSet<String>) -> i32 {
        let mut return_code: i32 = 0;
        for arg in args.iter().skip(1) {
            if built_in_commands.contains(arg) {
                println!("{} is a shell builtin", arg);
            } else if let Some(file_path) = TypeCommand::check_in_path(arg) {
                println!("{} is {}", arg, file_path);
            } else {
                eprintln!("{}: not found", arg);
                return_code = 1;
            }
        }
        return_code
    }

    fn check_in_path(command: &String) -> Option<String> {
        let split_by = match env::consts::FAMILY {
            "windows" => ";",
            "unix" => ":",
            _ => unimplemented!(),
        };
        let env_value = match env::var("PATH") {
            Ok(path) => path,
            Err(_) => return None,
        };
        for path in env_value.split(split_by) {
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
