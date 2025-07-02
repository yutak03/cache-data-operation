use redis::{Client, Commands, Connection};
use std::io::{self, Write};
use anyhow::Result;

struct CacheClient {
    connection: Connection,
}

impl CacheClient {
    fn connect(url: &str) -> Result<Self> {
        let client = Client::open(url)?;
        let connection = client.get_connection()?;
        Ok(CacheClient { connection })
    }

    fn get(&mut self, key: &str) -> Result<Option<String>> {
        let value: Option<String> = self.connection.get(key)?;
        Ok(value)
    }

    fn set(&mut self, key: &str, value: &str) -> Result<()> {
        let _: () = self.connection.set(key, value)?;
        Ok(())
    }

    fn search_keys(&mut self, pattern: &str) -> Result<Vec<String>> {
        let keys: Vec<String> = self.connection.keys(format!("*{}*", pattern))?;
        Ok(keys)
    }
}

fn main() -> Result<()> {
    println!("Cache Data Operation Tool");
    println!("=========================");
    
    print!("Redis/Valkey URL (default: redis://127.0.0.1/): ");
    io::stdout().flush()?;
    
    let mut url = String::new();
    io::stdin().read_line(&mut url)?;
    let url = url.trim();
    let url = if url.is_empty() { "redis://127.0.0.1/" } else { url };
    
    println!("Connecting to {}...", url);
    let mut client = CacheClient::connect(url)?;
    println!("Connected successfully!");
    
    loop {
        println!("\n--- Menu ---");
        println!("1. Search keys");
        println!("2. Get value by key");
        println!("3. Set key-value");
        println!("4. Exit");
        print!("Select option: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "1" => {
                print!("Enter search pattern: ");
                io::stdout().flush()?;
                let mut pattern = String::new();
                io::stdin().read_line(&mut pattern)?;
                let pattern = pattern.trim();
                
                match client.search_keys(pattern) {
                    Ok(keys) => {
                        if keys.is_empty() {
                            println!("No keys found matching pattern: {}", pattern);
                        } else {
                            println!("Found {} keys:", keys.len());
                            for key in keys {
                                println!("  - {}", key);
                            }
                        }
                    }
                    Err(e) => println!("Error searching keys: {}", e),
                }
            }
            "2" => {
                print!("Enter key name: ");
                io::stdout().flush()?;
                let mut key = String::new();
                io::stdin().read_line(&mut key)?;
                let key = key.trim();
                
                match client.get(key) {
                    Ok(Some(value)) => println!("Value: {}", value),
                    Ok(None) => println!("Key '{}' not found", key),
                    Err(e) => println!("Error getting value: {}", e),
                }
            }
            "3" => {
                print!("Enter key name: ");
                io::stdout().flush()?;
                let mut key = String::new();
                io::stdin().read_line(&mut key)?;
                let key = key.trim();
                
                print!("Enter value: ");
                io::stdout().flush()?;
                let mut value = String::new();
                io::stdin().read_line(&mut value)?;
                let value = value.trim();
                
                match client.set(key, value) {
                    Ok(()) => println!("Successfully set '{}' = '{}'", key, value),
                    Err(e) => println!("Error setting value: {}", e),
                }
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
    
    Ok(())
}
