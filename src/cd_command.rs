use std::{
    env,
    path::{absolute, Path, PathBuf},
};

use crate::utils;

pub struct CdCommand;

impl CdCommand {
    pub fn execute(args: &[String]) -> i32 {
        // initially target is args[1] or home dir
        let mut target: PathBuf = Path::new({
            if args.len() == 1 {
                "~"
            } else {
                &args[1]
            }
        })
        .to_path_buf();
        // try to expand tilde (home dir)
        if let Some(home_dir) = utils::home_dir() {
            target = Path::new(
                &args[1].to_string().replace(
                    "~",
                    home_dir
                        .to_str()
                        .expect("Failed to convert home_dir to str"),
                ),
            )
            .to_path_buf();
        }
        // convert to absolute path
        let target: PathBuf = {
            let path = target.as_path();
            match absolute(path) {
                Ok(p) => p,
                Err(_) => return CdCommand::no_such_file_or_directory(path),
            }
        };
        // try setting it as the current dir
        if env::set_current_dir(&target).is_err() {
            return CdCommand::no_such_file_or_directory(target);
        }
        0
    }

    fn no_such_file_or_directory<P: AsRef<Path>>(path: P) -> i32 {
        eprintln!("cd: {}: No such file or directory", path.as_ref().display());
        1
    }
}
