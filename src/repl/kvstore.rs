use std::collections::HashMap;
use std::io::{self, Write};

pub struct Kvstore {
    store: HashMap<String, String>,
}

impl Kvstore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.store.insert(key, value)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.store.remove(key)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.store.contains_key(key)
    }
}

pub fn repl() {
    let mut kv = Kvstore::new();

    println!(
        "\n
                                            ██████╗ ███████╗██████╗ ██████╗ ██╗   ██╗███████╗██████╗ ██████╗ 
                                            ██╔══██╗██╔════╝██╔══██╗██╔══██╗██║   ██║██╔════╝██╔══██╗██╔══██╗
                                            ██████╔╝█████╗  ██║  ██║██████╔╝██║   ██║███████╗██║  ██║██████╔╝
                                            ██╔══██╗██╔══╝  ██║  ██║██╔══██╗██║   ██║╚════██║██║  ██║██╔══██╗
                                            ██║  ██║███████╗██████╔╝██║  ██║╚██████╔╝███████║██████╔╝██████╔╝
                                            ╚═╝  ╚═╝╚══════╝╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═════╝ ╚═════╝ \n"
    );

    loop {
        let mut input = String::new();

        print!("REDRUSDB ==> ");
        io::stdout().flush().expect("Failed to flush");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.as_slice() {
            ["GET", key] => match kv.get(key) {
                Some(value) => println!("{} = {}", key, value),
                None => println!("{} not found", key),
            },
            ["SET", key, value] => {
                kv.set(key.to_string(), value.to_string());
                println!("Set {} = {}", key, value);
            }
            ["DELETE", key] => match kv.delete(key) {
                Some(value) => println!("Deleted {} = {}", key, value),
                None => println!("{} not found", key),
            },
            ["CONTAINS", key] => {
                if kv.contains(key) {
                    println!("Key {} exists in the store.", key);
                } else {
                    println!("Key {} does not exist in the store.", key);
                }
            }
            ["QUIT"] | ["EXIT"] => {
                println!("........... Goodbye! ...........");
                break;
            }
            _ => println!("Unknown command! Try again."),
        }
    }
}
