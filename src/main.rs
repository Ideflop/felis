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
use std::{
    env::args,   
    thread::sleep,
    time::Duration,
    fs::File,
    process::{
        Command,
        exit,
}};
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

fn get_argument() -> String {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        println!("There are no argument giving");
        exit(1)
    }
    let mut args = args.iter().peekable();
    let mut args_str = "".to_owned();
    while let Some(arg) = args.next() {
        args_str.push_str(&format!("+{}",arg));
    }
    args_str.remove(0);
    args_str

}

fn main() -> Result<()> {

    let args = get_argument();
        
    let search = File::create("html/research.txt").expect("failes to open research.txt");
    
    let mut cmd = Command::new("w3m")
            .arg(format!("https://duckduckgo.com/html?q={}",args))
            .stdout(search)
            .spawn()
            .expect("could not start w3m");
    
    cmd.wait().expect("failed to finish w3m");
    
    let cmd = Command::new("w3m")
            .arg(format!("https://duckduckgo.com/html?q={}",args))
            .spawn()
            .expect("could not start w3m");
    
    let pid = cmd.id();

    sleep(Duration::from_millis(2500));
    Command::new("felis-kill-bin")
            .arg(format!("{}", pid))
            .spawn()
            .expect("could not start stop pid");

    let mut cursor = TerminalCursor::new();

    loop {
        let char = read_char(); 

        // the 2 macht char are nearly the same because
        // I don't now how to to make _ empty
        match char {
            'h' => cursor.move_left(2),
            'j' => cursor.move_down(2),
            'k' => cursor.move_up(2),
            'l' => cursor.move_right(2),
            _ => cursor.move_up(1),
        };
        match char {
            'h' => cursor.move_right(1),
            'j' => cursor.move_up(1),
            'k' => cursor.move_down(1),
            'l' => cursor.move_left(1),
            _ => cursor.move_down(1),
        };

        if char == 'q' {
            exit(1)
        }
        
    }
    
}
