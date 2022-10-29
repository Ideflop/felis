use crate::config::check_config;

pub mod config;

use std::{
    fs,
    env::args,   
    process::{
        Command,
        exit,
    }, str::FromStr
};
use crossterm_cursor::Result;
use toml::value::Value;


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

fn get_search_engine(config_path : String)-> String {
    let config_read = fs::read_to_string(config_path).expect("could not read the config file of felis");
    let content = Value::from_str(&config_read).unwrap();
    let engine_choice = content.get("search_engine").unwrap().to_string();
    let engine_vec: Vec<&str> =  engine_choice.split('"').collect();
    let engine = engine_vec[1].to_owned();
    engine

}

fn main() -> Result<()> {

    let config_path = check_config(); // in config.rs
    
    let args = get_argument();
    let engine = get_search_engine(config_path);

    let mut cmd = Command::new("w3m")
            .arg(format!("{}{}", engine, args))
            .spawn()
            .expect("could not start w3m");
    cmd.wait().expect("failed to finish w3m");

    Ok(())
}
