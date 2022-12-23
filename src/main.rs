use std::{
    env::args,   
    process::{
        Command,
        exit,
    }, 
};
use crossterm_cursor::Result;

use config::create_alias;
use toml_manipulation::Toml;

pub mod config;
pub mod toml_manipulation;


fn get_argument() -> String {
    let mut args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        println!("There are no argument giving");
        exit(1)
    }

    match args.get(0).unwrap().as_str() {
        "-a" => {
            create_alias(); // in config.rs
            args.remove(0);
        },
        "-u" => {
            args.remove(0);
            url(&args[0]);
            },
        _ => ()
    }


    let mut args_item = args.iter().peekable();

    if args_item.peek().is_none() {
        println!("There are no argument giving");
        exit(1)
    }

    let mut args_str = "".to_owned();
    while let Some(arg) = args_item.next() {
        args_str.push_str(&format!("+{}",arg));
    }
    args_str.remove(0);

    args_str.trim().to_string()

}

fn url(url_adrress: &String) {
    let mut cmd = Command::new("w3m")
            .arg(format!("{}", url_adrress))
            .spawn()
            .expect("could not start w3m");
    cmd.wait().expect("failed to finish w3m");
    exit(1)
}

fn main() -> Result<()> {

    let args = get_argument();
    let engine = Toml::get_value("search_engine").unwrap();

    let mut cmd = Command::new("w3m")
            .arg(format!("{}{}", engine, args))
            .spawn()
            .expect("could not start w3m");
    cmd.wait().expect("failed to finish w3m");

    Ok(())
}
