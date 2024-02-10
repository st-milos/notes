// use std::collections::HashMap;
pub mod print;
pub mod help;
pub mod edit;

pub static COMMANDS: &'static [&dyn OptionCommand] = &[
    &print::Print{},
    &help::Help{},
    &edit::Edit{}
];

pub trait OptionCommand: Sync {
    fn run(&self) -> ();
    fn short_switch(&self) -> char;
    fn long_switch(&self) -> String;
}

