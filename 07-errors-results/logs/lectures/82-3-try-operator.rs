use std::fs;
use std::io::Error;

fn read_config_file() -> Result<String, Error> {
    fs::read_to_string("config.json")
}

fn get_config() -> Result<String, Error> {
    let config = read_config_file()?;

    Ok(config)
}

fn main() -> Result<(), Error> {
    let config = get_config()?;

    println!("Got a config: {}", config);

    Ok(())
}
