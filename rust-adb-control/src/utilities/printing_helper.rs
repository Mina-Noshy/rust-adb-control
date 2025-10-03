use colored::*;

pub struct Printer;

impl Printer {
    pub fn print_success(&self, message: &str) {
        println!("{}", message.green().bold());
    }
    // pub fn print_error(&self, message: &str) {
    //     eprintln!("{}", message.red().bold());
    // }
    // pub fn print_warning(&self, message: &str) {
    //     println!("{}", message.yellow().bold());
    // }
    // pub fn print_info(&self, message: &str) {
    //     println!("{}", message.blue().bold());
    // }
}

pub const PRINTER: Printer = Printer;
