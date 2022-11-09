use crate::config::{
    check_config,
    create_alias,
};

pub mod config;

use std::{
    fs,
    str::FromStr,
    env::args,   
    process::{
        Command,
        exit,
    }, 
};
use crossterm_cursor::Result;
use toml::value::Value;


fn get_argument() -> String {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        println!("There are no argument giving");
        exit(1)
    }

    match args.get(0).unwrap().as_str() {
        "-a" => create_alias(), // in config.rs
        "-u" => url(args.get(1).unwrap().to_owned()),
        _ => ()
    }

    let mut args = args.iter().peekable();
    let mut args_str = "".to_owned();
    while let Some(arg) = args.next() {
        args_str.push_str(&format!("+{}",arg));
    }
    args_str.remove(0);

    match args_str.as_str().trim() {
        "-a" => create_alias(), // in config.rs
        "-u" => {
            let mut url_adrress  = args_str.clone();
            url_adrress.remove(0);
            url(url_adrress);
            }
        _ => ()
    }
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

fn url(url_adrress: String) {
    let mut cmd = Command::new("w3m")
            .arg(format!("{}", url_adrress))
            .spawn()
            .expect("could not start w3m");
    cmd.wait().expect("failed to finish w3m");
    exit(1)
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


