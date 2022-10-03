const SHOW_NOTES: u8 = 1;
const ADD_NOTE: u8 = 2;


fn main() {
    let mut notes = Vec::new();
    loop {
        show_menu();
        let selected = read_menu_input();

        match selected {
            SHOW_NOTES => show_notes(&notes),
            ADD_NOTE => notes.push(add_note()),
            _ => break
        }
    }
}

fn show_menu() {
    println!();
    println!();
    println!("**** PROGRAM MENU ****");
    println!("1) Show notes");
    println!("2) Add note");
    println!("0) Exit");
}

fn read_menu_input() -> u8 {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let read_result = stdin.read_line(&mut buf);
    read_result.unwrap();
    buf.trim().parse().unwrap()
}

fn show_notes(notes: &Vec<String>) {
    println!();
    println!("Notes list:");
    for note in notes {
        println!("{}", note)
    }
    
}

fn add_note() -> String {
    println!();
    println!("Enter new note:");
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let read_result = stdin.read_line(&mut buf);
    read_result.unwrap();
    buf.trim().to_string()
}