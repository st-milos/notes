use std::env;
use option::OptionCommand;

pub mod argument_parser;
pub mod notes;
pub mod option;

fn main() {
    let args: Vec<String> = env::args().collect(); 
    let params = argument_parser::parse_config(&args);

    if params.command.is_some() {
        params.command.unwrap().run();
    }
    if params.note.len() > 0 { notes::store_note(&params.note); }
}

