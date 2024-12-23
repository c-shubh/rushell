pub struct EchoCommand;
impl EchoCommand {
    pub fn execute(args: &Vec<String>) -> i32 {
        for i in 1..(args.len() - 1) {
            print!("{} ", args[i]);
        }
        println!("{}", args[args.len() - 1]);
        0
    }
}
