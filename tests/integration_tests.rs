extern crate clickhouse_client;

#[test]
fn test_client_creation() {
    let client = clickhouse_client::client::http_client("localhost", 8123);

    assert!(client.is_ok());
}