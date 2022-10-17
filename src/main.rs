use std::{
    env::args,   
    process::{
        Command,
        exit,
    }
};
use crossterm_cursor::{Result};

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
        
    let mut cmd = Command::new("w3m")
            .arg(format!("https://duckduckgo.com/html?q={}",args))
            .spawn()
            .expect("could not start w3m");
    cmd.wait().expect("failed to finish w3m");
    Ok(())
}
