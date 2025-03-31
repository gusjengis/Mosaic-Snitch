use super::*;
use crate::send_request::send_request;

#[tokio::test]
async fn test_send_request() {
    // This is a simple test that just checks if the function runs without errors
    // In a real test, you would use a mock server
    let result = send_request(
        "GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n",
        "http://example.com:80",
    )
    .await;

    // This test will fail if you're not connected to the internet
    // In a real test environment, you would mock the TCP connection
    match result {
        Ok(response) => {
            println!("Request sent successfully\n{}", response);
            assert_eq!(true, true);
        }
        Err(e) => {
            println!("Error sending request: {}", e);
            assert_eq!(true, false);
        }
    };
}
