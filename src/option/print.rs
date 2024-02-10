use crate::OptionCommand;
use crate::notes;

pub struct Print {}

impl OptionCommand for Print {
    fn run(&self) -> () {
        let result = notes::list_notes();
        if result.is_err() {
            println!("Error reading notes file");
        }
    }
    fn short_switch(&self) -> char {
        return ' ';
    }
    fn long_switch(&self) -> String {
        return String::from(" ");
    }
}
