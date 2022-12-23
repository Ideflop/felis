use std::{env,
    time::Duration,
    thread::sleep,
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

use crate::toml_manipulation::Toml;

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
    let toml_string = toml::to_string(&Toml::create_toml_table("search_engine", config.as_str())).expect("could not make toml for the config file");
    fs::write(felis_config_file_copy, toml_string).expect("Could not write to file");
    
    // TODO: add the possibility to create an alias 

    Ok(())
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
        println!("could not write in bashrc 1");
        exit(1)
    }

    if let Err(_e) = writeln!(bashrc, "# The next alias was create by felis") {
        println!("could not write in bashrc 2");
        exit(1)
    }

    if let Err(_e) = writeln!(bashrc, "# This alias is for launching felis") {
        println!("could not write in bashrc 3");
        exit(1)
    }

    let alias_format = format!("alias {}=\"felis\"", alias.trim());

    if let Err(_e) = writeln!(bashrc, "{}",alias_format) {
        println!("could not write in bashrc 4");
        exit(1)
    }

    // write the alias to the config file
    let path_config = check_config();
    let toml_string = toml::to_string(&Toml::create_toml_table("alias", alias.trim())).expect("could not make toml for the config file");

    let mut config_file = OpenOptions::new()
                                .write(true)
                                .append(true)
                                .open(path_config)
                                .unwrap();

    if let Err(_e) = writeln!(config_file, "{}", toml_string) {
        println!("could not write your alias in the config file but it's should be in your .bashrc");
        println!("Restart your shell end test if your alias work");
        sleep(Duration::from_secs(2));
        return
    }

    println!("the alias {} was succefully added in .bashrc", alias.trim());
    println!("To use the alias restart your shell");
    sleep(Duration::from_secs(2));
}

// TODO :
// find_alias open the config file (get_value in Toml) and return the alias 
// create_alias display the alias, ask the alias and give it to write*
// write_in_config write in the config file
// write_in_bashrc write in the bashrc
