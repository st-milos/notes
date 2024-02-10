use std::env;
use std::ffi;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::io::Write;

pub fn get_store_path() -> ffi::OsString {
    let home: ffi::OsString = match env::var_os("HOME") {
        Some(val) => val,
        None => ffi::OsString::new() 
    };

    let mut store_path: ffi::OsString = ffi::OsString::new();
    store_path.push(home);
    store_path.push("/.notes");

    return store_path;
}

pub fn store_note(note: &String) {
    let store_path = get_store_path(); 
    let mut store = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(store_path)
        .unwrap();

    let mut note_text = note.to_string();
    note_text.push_str("\n");
    store
        .write(note_text.as_bytes())
        .expect("Write failed");
}

pub fn list_notes() -> Result<(), std::io::Error> {
    let store_path = get_store_path();
    println!("{:?}:", store_path);
    let store = OpenOptions::new()
        .read(true)
        .open(store_path)
        .unwrap();
    let reader = BufReader::new(store);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

