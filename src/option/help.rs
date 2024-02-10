use crate::option::OptionCommand;

pub struct Help {
}

impl OptionCommand for Help {
    fn run(&self) -> () {
        println!("Usage: notes [options] [note text]\n");
        println!("To print all notes, just type 'notes' without any params.\n");
        println!("Switches must come before any note text. Switches are as follows:\n");
        println!("-h --help     Read help.");
        println!("-e --edit     Open notes in editor. From there you are free to edit or delete them.");
    }
    fn short_switch(&self) -> char {
        return 'h';
    }
    fn long_switch(&self) -> String {
        return String::from("help");
    }
}
