use httparse::{Request};
use reqwest::{Client, Response};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener as AsyncTcpListener;
use tokio::net::TcpStream as AsyncTcpStream;

async fn handle_request(mut stream: AsyncTcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request:\n{}", request);
    
    // let headers = Request::from(&request[..]).headers();
    // let mut headers = [httparse::EMPTY_HEADER; 4];
    // let mut headers_str = String::new();
    // for (name, value) in headers.iter() {
    //     headers_str += &format!("{}: {}\r\n", name.as_str(), value.to_str().unwrap());
    // }

    // Imprimir las cabeceras de la petición
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut headers_str = String::new();
    let mut req = httparse::Request::new(&mut headers);
    req.parse(&buffer).unwrap();
    for header in req.headers.iter() {
        println!("{}: {}", header.name, String::from_utf8_lossy(header.value));
    }
    
    
    let url = "http://example.com";
    let resp = reqwest::get(url).await.unwrap();
    let mut content = String::new();
    // resp.into_body().read_to_string(&mut content).await.unwrap();
    
    let response = format!("HTTP/1.1 200 OK\r\n{}\r\n{}", headers_str, content);
    stream.write(response.as_bytes()).await.unwrap();
}

#[tokio::main]
async fn main() {
    let listener = AsyncTcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on 127.0.0.1:8080");
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_request(stream).await;
        });
    }
}
