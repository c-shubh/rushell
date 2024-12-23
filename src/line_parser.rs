pub struct LineParser;
impl LineParser {
    pub fn parse(line: String) -> Vec<String> {
        let mut current = String::new();
        let mut ans: Vec<String> = Vec::new();
        for char in line.chars() {
            if !char.is_ascii_whitespace() {
                current.push(char);
            } else if !current.is_empty() {
                ans.push(current.clone());
                current.clear();
            }
        }
        if !current.is_empty() {
            ans.push(current);
        }
        ans
    }
}
