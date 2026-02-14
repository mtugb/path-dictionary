use std::{
    collections::HashMap,
    env::current_dir,
    fs::{self},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Set {
        name: String,
        path: Option<String>,
    },
    Get {
        name: String,
    },
    #[clap(visible_alias("ls"))]
    List {},
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct Dictionary {
    items: HashMap<String, PathBuf>,
}

impl Dictionary {
    fn set(&mut self, name: &str, path: PathBuf) -> Option<PathBuf> {
        self.items.insert(name.to_string(), path)
    }
    fn get(&self, name: &String) -> Option<&PathBuf> {
        self.items.get(name)
    }
    fn get_all(&self) -> HashMap<String, PathBuf> {
        self.items.clone()
    }
}

fn get_config_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "mtugb", "pd").unwrap();
    let config_dir = proj_dirs.config_dir(); // Linuxなら ~/.config/pd/
    config_dir.join("pd.toml")
}

fn fetch_config_file() -> Dictionary {
    let config_path = get_config_path();
    let _ = fs::create_dir_all(config_path.parent().unwrap());
    if !config_path.exists() {
        Dictionary::default()
    } else {
        let raw_data = fs::read_to_string(config_path).unwrap();
        toml::from_str::<Dictionary>(raw_data.as_str()).unwrap()
    }
}

fn put_config_file(dictionary: Dictionary) {
    let config_path = get_config_path();
    let raw_data = toml::to_string::<Dictionary>(&dictionary).unwrap();
    let _ = fs::write(config_path, raw_data);
}

fn main() {
    let args = Args::parse();
    let mut dictionary = fetch_config_file().clone();
    match &args.command {
        Some(Commands::Set { name, path }) => {
            // something here
            if let Some(p) = path {
                let pb = dunce::canonicalize(p).unwrap();
                dictionary.set(name, pb);
            } else {
                let pd = dunce::canonicalize(current_dir().unwrap()).unwrap();
                dictionary.set(name, pd);
            }
            put_config_file(dictionary);
        }
        Some(Commands::Get { name }) => {
            if let Some(result_path) = dictionary.get(name) {
                println!("{}", result_path.to_str().unwrap());
            }
        }
        Some(Commands::List {}) => {
            let data = dictionary.get_all();
            for (key, value) in data.into_iter() {
                println!(
                    "[{key}] {}",
                    value
                        .into_os_string()
                        .into_string()
                        .expect("Path Parsing Error")
                );
            }
        }
        _ => {}
    }
}
