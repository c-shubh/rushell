pub struct EchoCommand;
impl EchoCommand {
    pub fn execute(args: &[String]) -> i32 {
        for arg in args.iter().take(args.len() - 1).skip(1) {
            eprintln!("{} ", arg);
        }
        if args.len() > 1 {
            eprintln!("{}", args[args.len() - 1]);
        }
        0
    }
}
