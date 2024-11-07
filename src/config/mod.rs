use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Config {
    pub keep_alive: bool,
    pub file_root: String,
    pub default_host: String,
}

impl Config {
    pub fn new() -> Option<Self> {
        let file_result = File::open("config");
        let file = match file_result {
            Ok(f) => f,
            Err(_) => return None,
        };
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
        let mut conf: Config = Config {
            keep_alive: true,
            file_root: "".to_string(),
            default_host: "".to_string(),
        };
        for line in lines {
            let splitted: Vec<&str> = line.split(' ').collect();
            match splitted[0].trim() {
                "keep_alive:" => match splitted[1].trim() {
                    "true" => conf.keep_alive = true,
                    "false" => conf.keep_alive = false,
                    _ => {
                        println!("Invalid keep alive: {}", splitted[1].trim());
                        return None;
                    }
                },
                "file_root:" => conf.file_root = splitted[1].trim().to_string(),
                "default_host:" => conf.default_host = splitted[1].trim().to_string(),
                "" => {}
                _ => {
                    println!("Invalid host: {}", splitted[0].trim());
                    return None;
                }
            }
        }

        Some(conf)
    }
}
