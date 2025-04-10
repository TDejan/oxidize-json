// Part of the submodule commands.
use serde_json::{self, Value};
use std::fs;

/// This function reads a JSON file and prints its contents in a pretty format.
pub fn prettify_json(file: &str) {
    match fs::read_to_string(file) {
        Ok(content) => match serde_json::from_str::<Value>(&content) {
            Ok(json) => {
                println!("{}", serde_json::to_string_pretty(&json).unwrap());
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
            }
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}
