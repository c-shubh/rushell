use std::{
    env,
    path::{absolute, Path, PathBuf},
};

pub struct CdCommand;

impl CdCommand {
    pub fn execute(args: &[String], current_dir: &mut PathBuf) -> i32 {
        if args.len() == 1 {
            todo!("set current_dir to user's home dir here")
        }

        let target: PathBuf = {
            let path = Path::new(&args[1]);
            match absolute(path) {
                Ok(p) => p,
                Err(_) => return CdCommand::no_such_file_or_directory(path.to_path_buf()),
            }
        };
        if env::set_current_dir(&target).is_err() {
            return CdCommand::no_such_file_or_directory(target.to_path_buf());
        }
        current_dir.clear();
        current_dir.push(target);
        0
    }

    fn no_such_file_or_directory<P: AsRef<Path>>(path: P) -> i32 {
        println!("cd: {}: No such file or directory", path.as_ref().display());
        return 1;
    }
}
