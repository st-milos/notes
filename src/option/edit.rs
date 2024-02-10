use std::env;
use std::process;
use crate::notes;
use crate::option::OptionCommand;

pub struct Edit {}

impl OptionCommand for Edit {
    fn run(&self) -> () {
        let editor: String;
        if env::var("EDITOR").is_err() {
            editor = String::from("vi");
        } else {
            editor = env::var("EDITOR").unwrap();
        }
        process::Command::new(editor)
            .arg(notes::get_store_path())
            .status()
            .expect("Opening notes in editor failed");
    }
    fn short_switch(&self) -> char {
        return 'e';
    }
    fn long_switch(&self) -> String {
        return String::from("edit");
    }
}

