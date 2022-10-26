use std::{env,
    path::Path,
    fs,
    fs::File,
};


pub fn check_config(){
    
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
    let felis_exist = Path::new(&felis_config_dir).exists();
    if !felis_exist {
        _ = create_config(felis_config_dir)
    }
    
}

fn create_config(felis_config_dir:String) -> std::io::Result<()> {
    // we create felis dir and config file
    let felis_config_file_prep = felis_config_dir.clone().to_owned();
    fs::create_dir(felis_config_dir)?;

    let felis_config_file = felis_config_file_prep+"/config";
    File::create(felis_config_file)?;
    Ok(())

}
