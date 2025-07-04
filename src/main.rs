use anyhow::Result;
use cdot::CacheClient;
use std::io::{self, Write};

fn main() -> Result<()> {
    // Interactive URL input
    print!("Enter Redis URL (default: redis://127.0.0.1/): ");
    io::stdout().flush()?;
    
    let mut url_input = String::new();
    io::stdin().read_line(&mut url_input)?;
    let url = url_input.trim();
    
    let redis_url = if url.is_empty() {
        "redis://127.0.0.1/"
    } else {
        url
    };

    println!("Connecting to {}...", redis_url);
    let mut client = CacheClient::connect(redis_url)?;
    println!("Connected successfully!");

    run_interactive_mode(&mut client)?;

    Ok(())
}


fn run_interactive_mode(client: &mut CacheClient) -> Result<()> {
    println!("Cache Data Operation Tool");
    println!("=========================");

    loop {
        println!("\n--- Menu ---");
        println!("1. Search keys");
        println!("2. Get value by key");
        println!("3. Set key-value");
        println!("4. Delete key");
        println!("5. Exit");
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

                print!("Enter TTL in seconds (press Enter for no expiration): ");
                io::stdout().flush()?;
                let mut ttl_input = String::new();
                io::stdin().read_line(&mut ttl_input)?;
                let ttl_input = ttl_input.trim();

                if ttl_input.is_empty() {
                    match client.set(key, value) {
                        Ok(()) => println!("Successfully set '{}' = '{}'", key, value),
                        Err(e) => println!("Error setting value: {}", e),
                    }
                } else {
                    match ttl_input.parse::<u64>() {
                        Ok(ttl_seconds) => match client.set_with_ttl(key, value, ttl_seconds) {
                            Ok(()) => println!(
                                "Successfully set '{}' = '{}' with TTL of {} seconds",
                                key, value, ttl_seconds
                            ),
                            Err(e) => println!("Error setting value with TTL: {}", e),
                        },
                        Err(_) => println!("Invalid TTL value. Please enter a valid number."),
                    }
                }
            }
            "4" => {
                print!("Enter key name to delete: ");
                io::stdout().flush()?;
                let mut key = String::new();
                io::stdin().read_line(&mut key)?;
                let key = key.trim();

                match client.delete(key) {
                    Ok(()) => println!("Successfully deleted key '{}'", key),
                    Err(e) => println!("Error deleting key: {}", e),
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }

    Ok(())
}
