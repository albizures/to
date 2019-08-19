extern crate structopt;

use std::{ fs };
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "to somewhere", about = "Create and use your folder aliases")]
struct Cli {
    #[structopt(short = "s", long = "save")]
    save: bool,
    alias: String,
}

// TODO use json instead of string
fn read_config () -> String {
    let config = fs::read_to_string("~/.to.json");

    match config {
        Ok(content) => return content,
        _ => (),
    }

    // TODO create config file if it doesn;t exist
    String::from("")
}

fn save_alias (alias: String) {
    println!("save {}", alias);
}

fn go_to(alias: String) {
    println!("go to {}", alias);
}

fn main() {
    let opt = Cli::from_args();

    let Cli { save, alias } = opt;

    let config = read_config();

    println!("{:?}", config);

    if save {
        save_alias(alias);
    } else {
        go_to(alias);
    }
}
