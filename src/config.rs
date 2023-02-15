use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, File},
    io::Write,
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub key: String,
    pub wait: u64,
}

impl Config {
    pub fn load() -> Config {
        let data = match read_to_string("./config.json") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("failed to read config!");
                eprintln!("{:?}", e);
                eprintln!("creating new config file at ./config.json");
                Config::new().unwrap();
                std::process::exit(1);
            }
        };

        let json: Config = serde_json::from_str(&data).unwrap();

        println!("Key: {}, Wait: {}", key, wait);
        Config {
            key: json.key,
            wait: json.wait,
        }
    }
    fn new() -> Result<()> {
        let mut config = File::create("./config.json")?;

        let default = r#"{
    "key": "APIKEYHERE",
    "update": 900
}"#;

        config.write(default.as_bytes())?;

        Ok(())
    }
}
