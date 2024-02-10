use crate::option;
use option::OptionCommand;

pub struct Params {
    pub note: String,
    pub list_notes: bool,
    pub command: Option<&'static dyn OptionCommand>,
}

pub fn parse_config(args: &[String]) -> Params {
    let mut params = Params {
        note: String::new(),
        list_notes: false,
        command: None, // 
    };
    //if args.len() == 1 { return params; }
    let mut note_text: String = String::new();
    for (index, word) in args.iter().enumerate() {
        if index == 0 { continue; }
        if index == 1 {
            if word.starts_with("-") && 
                word.as_str().chars().count() == 2 && 
                    try_add_option_from_short(&mut params, word) { continue; }
            if word.starts_with("--") &&
                word.as_str().chars().count() >= 3 &&
                    try_add_option_from_long(&mut params, word) { continue; }
        }
        if index > 1 { note_text.push_str(" "); }
        note_text.push_str(&*word);
    }

    params.note = note_text;
    if params.command.is_none() && params.note.is_empty() { // Default to first command if no args
        params.command = Some(*option::COMMANDS.first().unwrap());
    }

    return params;
}

fn try_add_option_from_short(params: &mut Params, word: &str) -> bool {
    for cmd in option::COMMANDS.iter() {
        let switch = word.chars().last().unwrap();
        if cmd.short_switch() == switch {
            params.command = Some(*cmd);
            return true;
        }
    }

    return false;
}

fn try_add_option_from_long(params: &mut Params, word: &str) -> bool {
    for cmd in option::COMMANDS.iter() {
        let mut switch = word.to_string();
        switch.remove(0);
        switch.remove(0);
        if cmd.long_switch() == switch {
            params.command = Some(*cmd);
            return true;
        }
    }

    return false;
}

