extern crate structopt;
extern crate serde_json;
extern crate dirs;

use std::{ fs, env, io };
use std::path;
use structopt::StructOpt;
use std::io::ErrorKind;
use std::collections::HashMap;


#[derive(Debug, StructOpt)]
#[structopt(name = "to somewhere", about = "Create and use your folder aliases")]
struct Cli {
    #[structopt(short = "s", long = "save")]
    save: bool,
    alias: String,
}

type Config = HashMap<String, String>;

fn get_config_path() -> path::PathBuf {
    let home_dir = dirs::home_dir()
        .expect("Unable to read config file");

    home_dir.join(".to.json")
}

fn save_config (config: &Config) -> serde_json::Result<()> {
    let config_path = get_config_path();
    let raw_config = serde_json::to_string(&config)?;

    fs::write(config_path, raw_config)
        .expect("Unable to write file");

    Ok(())
}

fn create_config_file(config_path: &std::path::PathBuf) -> String {
    let default_value = "{}";
    fs::write(config_path, default_value)
        .expect("Unable to write file");

    String::from(default_value)
}

fn read_raw_config () -> String {
    let config_path = get_config_path();
    let config = fs::read_to_string(&config_path);

    match config {
        Ok(content) => return content,
        Err(ref error) if error.kind() == ErrorKind::NotFound => create_config_file(&config_path),
        Err(error) => panic!("couldn't open {:?}", error),
    }
}

fn read_config () -> serde_json::Result<Config> {
    let raw_config = read_raw_config();
    let config: Config = serde_json::from_str(&raw_config)?;

    Ok(config)
}

fn save_alias (alias: String) -> io::Result<()> {
    let mut config = read_config()?;
    println!("save {}", alias);

    match config.get(&alias) {
        Some(folder_path) => panic!("Alias {} already use ({})", alias, folder_path),
        None => {
            if let Some(current_dir) = env::current_dir()?.to_str() {
                config.insert(alias, String::from(current_dir));
                save_config(&config)?;
            }
        }
    }

    Ok(())
}

fn go_to(alias: String) -> io::Result<()> {
    let config = read_config()?;

    if let Some(folder_path) = config.get(&alias) {
        println!("{}", folder_path);
        return Ok(())
    }

    panic!("Unknow alias");
}

fn main() {
    let opt = Cli::from_args();
    let Cli { save, alias } = opt;

    if save {
        save_alias(alias);
    } else {
        go_to(alias);
    }
}
