//fn search() {
//    Command::new("wget")
//            .arg("-O")
//            .arg("html/html.txt")
//            .arg("https://duckduckgo.com/html?q=macron")
//            .output()
//            .expect("could not find the something needed to perform the search");
//}
    //Command::new("wget")
    //        .arg("-O")
    //        .arg("wget.html")
    //        .arg("https://duckduckgo.com/html?q=macron")
    //        .output()
    //        .expect("error");

    //Command::new("w3m")
    //        .arg("wget.html")
    //        .output()
    //        .expect("error");
use std::process::{
    Command,
    exit,
};
use crossterm::
    event::{
        Event,
        read,
        KeyEvent,
        KeyCode,
    };

use crossterm_cursor::{Result, TerminalCursor};

pub fn read_char() -> char {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = read()
        {
            return c;
        }
    }
}

fn main() -> Result<()> {
    Command::new("w3m")
            .arg("https://duckduckgo.com/html?q=asdf")
            .arg(">")
            .arg("asdf.html")
            .spawn()
            .expect("error");

    let mut cursor = TerminalCursor::new();

    loop {
        let char = read_char();
        
        let hello = match char {
            'h' => cursor.move_left(1),
            'j' => cursor.move_down(1),
            'k' => cursor.move_up(1),
            'l' => cursor.move_right(1),
            _ => cursor.move_up(0),
        };


        if read_char() == 'q' {
            exit(1)
        }
        //println!("{}", char);
        
    }
    
}
