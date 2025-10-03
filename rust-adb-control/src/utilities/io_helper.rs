pub struct Input;

impl Input {
    pub fn read_line(&self, msg: &str) -> String {
        println!("{}", msg);
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input!");

        input.trim().to_string()
    }
}
