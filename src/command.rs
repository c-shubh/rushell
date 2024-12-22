pub trait Command {
    fn execute(args: &Vec<String>) -> i32;
}
