use anyhow::Result;
use cdot::CacheClient;

#[test]
#[ignore = "requires redis instance"]
fn test_connection() -> Result<()> {
    let _client = CacheClient::connect("redis://127.0.0.1/")?;
    Ok(())
}

#[test]
#[ignore = "requires redis instance"]
fn test_set_and_get() -> Result<()> {
    let mut client = CacheClient::connect("redis://127.0.0.1/")?;

    let key = "test_key";
    let value = "test_value";

    client.set(key, value)?;
    let retrieved = client.get(key)?;

    assert_eq!(retrieved, Some(value.to_string()));

    client.delete(key)?;
    Ok(())
}

#[test]
#[ignore = "requires redis instance"]
fn test_set_with_ttl() -> Result<()> {
    let mut client = CacheClient::connect("redis://127.0.0.1/")?;

    let key = "test_ttl_key";
    let value = "test_ttl_value";

    client.set_with_ttl(key, value, 60)?;
    let retrieved = client.get(key)?;

    assert_eq!(retrieved, Some(value.to_string()));

    client.delete(key)?;
    Ok(())
}

#[test]
#[ignore = "requires redis instance"]
fn test_search_keys() -> Result<()> {
    let mut client = CacheClient::connect("redis://127.0.0.1/")?;

    client.set("search_test_1", "value1")?;
    client.set("search_test_2", "value2")?;
    client.set("other_key", "value3")?;

    let search_results = client.search_keys("search_test")?;
    assert_eq!(search_results.len(), 2);

    client.delete("search_test_1")?;
    client.delete("search_test_2")?;
    client.delete("other_key")?;

    Ok(())
}

#[test]
#[ignore = "requires redis instance"]
fn test_get_nonexistent_key() -> Result<()> {
    let mut client = CacheClient::connect("redis://127.0.0.1/")?;

    let result = client.get("nonexistent_key_12345")?;
    assert_eq!(result, None);

    Ok(())
}

#[test]
#[ignore = "requires redis instance"]
fn test_delete() -> Result<()> {
    let mut client = CacheClient::connect("redis://127.0.0.1/")?;

    let key = "delete_test_key";
    client.set(key, "value")?;

    let exists = client.get(key)?;
    assert!(exists.is_some());

    client.delete(key)?;

    let after_delete = client.get(key)?;
    assert_eq!(after_delete, None);

    Ok(())
}
