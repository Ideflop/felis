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
use crossterm::{
    cursor::{
        MoveRight,
        MoveUp,
        MoveDown,
        MoveLeft,
    },
    event::{
        Event,
        read,
        KeyEvent,
        KeyCode,
    },
    queue,
};

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

fn main() {
    Command::new("w3m")
            .arg("https://duckduckgo.com/html?q=asdf")
            .arg(">")
            .arg("asdf.html")
            .spawn()
            .expect("error");

    loop {
        match read_char() {
            'h' => MoveLeft(1),
            'j' => MoveDown(1),
            'k' => MoveUp(1),
            'l' => MoveRight(1),
            _ => (),
        }

        //if char == 'q' {
        //    exit(1)
        //} else if char == 'h' {
        //    MoveLeft(1)
        //} else if char == 'j' {
        //    MoveDown(1)
        //} else if char == 'k' {
        //    MoveUp(1)
        //} else if char == 'l' {
        //    MoveRight(1)
        //} else {
        //    
        //}
        
        //println!("{}", char);
        
    }
    
}
