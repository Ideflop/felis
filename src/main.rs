use std::io::{stdout, Write};
use std::process::Command;

use crossterm::event::{
    poll, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::{
    cursor::{position, Show, MoveTo},
    event::{
        read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, Event, KeyCode, KeyEvent,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
    queue,
};


//fn search() {
//    Command::new("wget")
//            .arg("-O")
//            .arg("html/html.txt")
//            .arg("https://duckduckgo.com/html?q=macron")
//            .output()
//            .expect("could not find the something needed to perform the search");
//}

pub fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = read()
        {
            return Ok(c);
        }
    }
}

fn run() -> Result<()> {
    loop {
        // Blocking read
        let event = read()?;
        let mut stdout = stdout();

        println!("Event: {:?}\r", event);

        if event == Event::Key(KeyCode::Char('c').into()) {
            println!("Cursor position: {:?}\r", position());
        }

        match read_char() {
            Ok('j') => {
                queue!(stdout, MoveTo(10,10));
                println!("Event {:?}", event);
            }

            Ok('k') => {
                queue!(stdout, MoveTo(11,11));
                println!("Event {:?}", event);
            } 
            Ok(_) | Err(_) => break
        };
        stdout.flush();

    }
    Ok(())
}

fn main() -> Result<()> {
    print!("\x1B[2J");

    let string = "Hello everybody";
    let mut stdout = stdout();

    println!("{}",string);
    enable_raw_mode()?;

    execute!(
        stdout,
        Show,
        EnableBracketedPaste,
        EnableFocusChange,
        EnableMouseCapture,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
        )
    )?;

    if let Err(e) = run() {
        println!("Error: {:?}\r", e);
    }

    execute!(
        stdout,
        DisableBracketedPaste,
        PopKeyboardEnhancementFlags,
        DisableFocusChange,
        DisableMouseCapture
    )?;
    
    Command::new("clear").output().expect("lol erreur");
    disable_raw_mode()

}
