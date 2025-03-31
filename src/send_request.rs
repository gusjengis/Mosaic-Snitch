use std::error::Error;

use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, lookup_host};
use url::Url;

pub async fn send_request(request_str: &str, url_str: &str) -> Result<String, Box<dyn Error>> {
    let url = Url::parse(url_str)?;
    let host = url.host_str().ok_or("No host in URL")?;
    let port = url.port().unwrap_or_else(|| match url.scheme() {
        "https" => 443,
        _ => 80,
    });

    let addr = format!("{}:{}", host, port);
    let mut addrs = lookup_host(addr).await?;
    let socket_addr = addrs.next().ok_or("Failed to resolve address")?;

    // Connect to the server
    let mut stream = TcpStream::connect(socket_addr).await?;

    // Send the request
    stream.write_all(request_str.as_bytes()).await?;
    stream.flush().await?;

    // Read the response
    let mut response = String::new();
    use tokio::io::AsyncReadExt;
    let mut buffer = [0; 1024];

    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        response.push_str(&String::from_utf8_lossy(&buffer[0..n]));
    }

    Ok(response)
}
