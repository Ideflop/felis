use std::{env,
    path::Path,
    process::exit,
    fs::{self,
        File,
        OpenOptions
    },
    io::{
        stdin,
        stdout, Write,
    }, 
};
use toml::{map::Map, Value};


pub fn check_config() -> String{
    
    // check if config dir exist
    let config_home_result = env::var("XDG_CONFIG_HOME")
        .or_else(|_| env::var("HOME").map(|home|format!("{}/.config", home))).unwrap();
    let _config_exist = match Path::new(&config_home_result).exists() {
       true  => (),
       false => panic!("no config dir or no home variable set with XDG")
    };

    // check if felis dir exist inside the config file
    let felis_config_dir = env::var("XDG_CONFIG_HOME")
        .or_else(|_| env::var("HOME").map(|home|format!("{}/.config/felis", home))).unwrap();
    let felis_config = felis_config_dir.clone();
    let felis_exist = Path::new(&felis_config_dir).exists();
    if !felis_exist {
        _ = create_config(felis_config_dir)
    }
    let felis_config_file = felis_config.to_owned()+"/config";
    felis_config_file
    
}

fn create_config(felis_config_dir:String) -> std::io::Result<()> {
    // we create felis dir and config file
    let felis_config_file_prep = felis_config_dir.clone().to_owned();
    fs::create_dir(felis_config_dir)?;

    let felis_config_file = felis_config_file_prep+"/config";
    let felis_config_file_copy = felis_config_file.clone();
    File::create(felis_config_file)?;


    println!("Config file creation");

    println!("Wich search engine would you use ?");
    let mut search_engine = String::new();
    print!("d for duckduckgo or g for google : ");
    let _ = stdout().flush();
    stdin().read_line(&mut search_engine).expect("This wasnt a letter");

    let config = match search_engine.trim() { 
        "d"  => String::from("https://duckduckgo.com/html?q="),
        "g"  => String::from("https://www.google.com/search?q="),
        _ => panic!("the letter is not d or g"),
    }; 

    // write to file as toml
    let toml_string = toml::to_string(&to_toml(config)).expect("could not make toml for the config file");
    fs::write(felis_config_file_copy, toml_string).expect("Could not write to file");
    
    Ok(())
}

fn to_toml(config:String) -> Value{
    let mut engine = Map::new();
    engine.insert("search_engine".into(), Value::String(config));
    Value::Table(engine)
}

pub fn create_alias(){
    println!("felis is going to create an alias into your .bashrc in your home directory");
    println!("Wich alias shoud be used ?");
    print!("alias name : ");

    let mut alias = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut alias).expect("An error happend while reading the input");
    
    let path_to_bashrc = env::var("XDG_CONFIG_HOME")
        .or_else(|_| env::var("HOME").map(|home|format!("{}/.bashrc", home))).unwrap();
    if !Path::new(&path_to_bashrc).is_file(){
        println!("Could not find .bashrc file in your home directory. Please create it");
        exit(1)
    }

    let mut bashrc = OpenOptions::new()
                                .write(true)
                                .append(true)
                                .open(path_to_bashrc)
                                .unwrap();

    if let Err(_e) = writeln!(bashrc, "") {
        println!("could not write in bashrc");
        exit(1)
    }

    if let Err(_e) = writeln!(bashrc, "# The next alias was create by felis") {
        println!("could not write in bashrc");
        exit(1)
    }

    if let Err(_e) = writeln!(bashrc, "# This alias is for launching felis") {
        println!("could not write in bashrc");
        exit(1)
    }

    let alias_format = format!("alias {}=\"felis\"", alias.trim());

    if let Err(_e) = writeln!(bashrc, "{}",alias_format) {
        println!("could not write in bashrc");
        exit(1)
    }

    println!("the alias {} was succefully added in .bashrc", alias.trim());
    println!("To use the alias restart your shell");
    exit(1)
}
