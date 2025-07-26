use std::{env,
    time::Duration,
    thread::sleep,
    path::Path,
    process::exit,
    fs::{self,
        File,
        OpenOptions,
    },
    io::{
        stdin,
        stdout,
        Write,
        Read,
    }, 
};

use crate::toml_manipulation::Toml;

pub fn check_config() -> String {
    
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
        _ = create_config(&felis_config_dir, true)
    }
    let felis_config_file = felis_config.to_owned()+"/config";
    let felis_config_file_exist = Path::new(&felis_config_file).exists();
    if !felis_config_file_exist {
        _ = create_config( &felis_config_dir, false)
    }
    felis_config_file
    
}

fn create_config(felis_config_dir: &String, create_felis_dir: bool) -> std::io::Result<()> {

    let felis_config_file_prep = felis_config_dir.clone().to_owned();
    if create_felis_dir {
        fs::create_dir(felis_config_dir)?;
    }

    let felis_config_file = felis_config_file_prep+"/config";
    let felis_config_file_copy = felis_config_file.clone();
    File::create(felis_config_file)?;


    println!("Config file creation at {}", felis_config_file_copy);

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
    

    print!("Would you like to create an alias for felis ? : [y]es or [n]o : ");
    let mut make_alias = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut make_alias).expect("This wasnt a letter");

    match make_alias.trim() {
        "y" => create_alias(),
        "n" => (),
        _ => panic!("the letter is not y or n"),
    }

    Ok(())
}

pub fn create_alias(){

    println!("felis is going to create an alias into your .bashrc in your home directory");

    let felis_alias = Toml::get_value("alias");
    let already_in_config: bool;
    match felis_alias {
        Ok(value) => {
            println!("The current alias is {}", value);
            already_in_config = true;
        },
        Err(_) => {
            print!("");
            already_in_config = false;
        }
    }

    println!("Wich alias shoud be used ?");
    print!("alias name : ");

    let mut alias = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut alias).expect("An error happend while reading the input");

    write_in_bahsrc(&alias, already_in_config);
    write_in_config(&alias, already_in_config);

    println!("To use the alias : {}, restart your shell", alias.trim());
    sleep(Duration::from_secs(2));
}

fn write_in_config(alias: &String, already_in: bool) {

    match already_in {
        true => {
            let result = Toml::update_value("alias", alias.trim());
            match result {
                Ok(_) => println!("Successfully updated the value"),
                Err(error) => println!("Error updating value: {}", error),
            }
            return
        },
        false => (),
    } 
    
    let path_config = check_config();
    let toml_string = toml::to_string(&Toml::create_toml_table("alias", alias.trim())).expect("could not make toml for the config file");

    let mut config_file = OpenOptions::new()
                                .write(true)
                                .append(true)
                                .open(path_config)
                                .unwrap();

    if let Err(_e) = writeln!(config_file, "{}", toml_string) {
        println!("could not write your alias in the config file ");
        println!("If there was no error about the .bashrc file than you can use the alias");
        sleep(Duration::from_secs(2));
        return
    }

    println!("the alias {} was succefully added in config file", alias.trim());
}

fn write_in_bahsrc(alias: &String, already_in: bool) {

    let path_to_bashrc = env::var("XDG_CONFIG_HOME")
        .or_else(|_| env::var("HOME").map(|home|format!("{}/.bashrc", home))).unwrap();
    if !Path::new(&path_to_bashrc).is_file(){
        println!("Could not find .bashrc file in your home directory. Please create it");
        exit(1)
    }

    match already_in {
        true => {
            let _result = update_alias_bashrc(alias, &path_to_bashrc).unwrap();
            return
        },
        false => (),
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

}

/// Updates the given alias in the specified bashrc file.
///
/// # Arguments
/// * `alias` - The new alias to set.
/// * `path_to_bashrc` - The path to the bashrc file.
///
/// # Returns
/// If the alias was successfully updated, returns `Ok(())`. Otherwise, returns an `Err` containing an error message.
fn update_alias_bashrc(alias : &String, path_to_bashrc: &String) -> Result<(), String> {

    let mut bashrc_file = match OpenOptions::new().read(true).write(true).open(path_to_bashrc) {
        Ok(file) => file,
        Err(error) => return Err(format!("Could not open the file : {}", error)),
    };

    let mut bashrc_content = String::new();
    match bashrc_file.read_to_string(&mut bashrc_content) {
        Ok(_) => (),
        Err(error) => return Err(format!("Error: Could not read bashrc file, {}", error)),
    }

    let old_alias = format!("alias {}=\"felis\"", Toml::get_value("alias").unwrap().trim());
    let new_alias = format!("alias {}=\"felis\"", alias.trim());
    let new_bashrc_content = bashrc_content.replace(&old_alias, &new_alias);

    let mut old_bashrc_file = OpenOptions::new().read(true).write(true).truncate(true).open(path_to_bashrc).unwrap();
    match old_bashrc_file.write_all(new_bashrc_content.as_bytes()) {
        Ok(_) => println!("Successfully changed alias {} to {} in {}", old_alias, new_alias, path_to_bashrc),
        Err(e) => return Err(format!("Error: Could not write in bashrc, {}", e)),
    }

    Ok(())
}
