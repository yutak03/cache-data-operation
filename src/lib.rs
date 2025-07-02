use anyhow::Result;
use redis::{Client, Commands, Connection};

pub struct CacheClient {
    connection: Connection,
}

impl CacheClient {
    pub fn connect(url: &str) -> Result<Self> {
        let client = Client::open(url)?;
        let connection = client.get_connection()?;
        Ok(CacheClient { connection })
    }

    pub fn get(&mut self, key: &str) -> Result<Option<String>> {
        let value: Option<String> = self.connection.get(key)?;
        Ok(value)
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        let _: () = self.connection.set(key, value)?;
        Ok(())
    }

    pub fn set_with_ttl(&mut self, key: &str, value: &str, ttl_seconds: u64) -> Result<()> {
        let _: () = self.connection.set_ex(key, value, ttl_seconds)?;
        Ok(())
    }

    pub fn search_keys(&mut self, pattern: &str) -> Result<Vec<String>> {
        let keys: Vec<String> = self.connection.keys(format!("*{}*", pattern))?;
        Ok(keys)
    }

    pub fn delete(&mut self, key: &str) -> Result<()> {
        let _: () = self.connection.del(key)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pattern_formatting() {
        let pattern = "test";
        let formatted = format!("*{}*", pattern);
        assert_eq!(formatted, "*test*");
    }
}
