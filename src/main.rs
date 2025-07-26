use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::{env, fs};
use std::{path::Path, process::Command};

use anyhow::{anyhow, bail, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

const SEARCH_ENGINES: [(&str, &str); 2] = [
    ("google", "https://www.google.com/search?q="),
    ("duckduckgo", "https://duckduckgo.com/html?q="),
];

#[derive(Parser, Debug)]
#[command(version, about = "Command line interface to browse the web using W3m", long_about = None)]
struct Cli {
    #[arg(short, long, help = "Url from the website")]
    url: Option<String>,
    #[arg(help = "Web search")]
    web_search: Option<Vec<String>>,
}

//TODO:
//check w3m installed
//add -s to specfie custom search engine (bypass conf file)
fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Some(url_adress) = cli.url {
        let mut cmd = Command::new("w3m")
            .arg(&url_adress)
            .spawn()
            .map_err(|e| anyhow!("Could not spawn W3m because of: {}", e))?;

        cmd.wait()?;
    } else if let Some(search_words) = cli.web_search {
        if search_words.is_empty() {
            bail!("No words could be found")
        }

        let config = get_config()?;
        let search_engine = SEARCH_ENGINES
            .into_iter()
            .filter(|(k, _)| *k == config.search_engine)
            .map(|(_, v)| v)
            .collect::<String>();

        let mut search = String::new();
        for word in search_words {
            search.push_str(&format!("+{word}"));
        }

        println!("{search}");
        println!("{search_engine}");
        let mut cmd = Command::new("w3m")
            .arg(format!("{search_engine}{search}"))
            .spawn()
            .map_err(|e| anyhow!("Could not spawn W3m because of: {}", e))?;
        cmd.wait()?;
    } else {
        bail!("You need to specify an Url of words for a web search")
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Config {
    search_engine: String,
}

fn get_config() -> Result<Config> {
    let mut config_path = env::var("XDG_CONFIG_HOME")
        .or_else(|_| env::var("HOME").map(|home| format!("{home}/.config")))
        .map_err(|e| {
            anyhow!(
                "XDG_CONFIG_HOME and HOME variable are not set cannot find config file: {}",
                e
            )
        })?;

    if !Path::new(&config_path).exists() {
        bail!("Couldn't find config path")
    }

    config_path.push_str("/felis");
    if !Path::new(&config_path).exists() {
        fs::create_dir(&config_path)?;
    }

    config_path.push_str("/config");
    if !Path::new(&config_path).exists() {
        File::create(&config_path)?;

        create_config(&config_path)?;
    }

    let content = fs::read(&config_path)?;
    Ok(toml::from_slice(&content)?)
}

fn create_config(config_path: &String) -> Result<()> {
    println!("Wich search engine would you use ?");
    let engine_list = SEARCH_ENGINES
        .into_iter()
        .map(|(k, _)| k)
        .collect::<Vec<&str>>();

    loop {
        println!("Available search engie: {}", engine_list.join(", "));
        print!("Your choice: ");
        let mut user_choice = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut user_choice)
            .expect("This wasnt a letter");

        if engine_list.iter().any(|x| *x == user_choice.trim()) {
            let config = Config {
                search_engine: user_choice.trim().to_string(),
            };
            fs::write(config_path, toml::to_string(&config)?)?;
            return Ok(());
        }
    }
}
