use std::fs;
use std::io::Error;

fn read_config_file() -> Result<String, Error> {
    fs::read_to_string("config.json")
}

fn get_config() -> String {
    let default_config = String::from("{ enable_debug: true }");

    match read_config_file() {
        Ok(config) => config,
        Err(_err) => {
            println!("Failed to read config file: {}", _err);
            default_config
        }
    }
}

fn main() {
    let config = get_config();

    println!("Got a config: {}", config);
}
