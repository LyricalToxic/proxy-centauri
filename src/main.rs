use std::io;

use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;

use crate::http_parser::HTTPResponseParser;

mod http_parser;

static TEST: bool = false;

async fn process_tcp_client_stream(mut socket: TcpStream) -> io::Result<()> {
    loop {
        socket.readable().await?;
        let mut buf = [0; 4096];
        match socket.try_read(&mut buf) {
            Ok(0) => {
                println!("Socket has been closed: {:?}", socket.peer_addr());
                break;
            }
            Ok(n) => {
                println!("Read N bytes: {n}")
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
        let http_parser = HTTPResponseParser::new(Box::from(buf)).unwrap();
        println!("Read data: {}", http_parser.get_text());
        println!("Headers: {}", http_parser.get_headers());
        break;
    }

    socket.shutdown().await?;
    println!("Socket shutdown {:?}", socket.peer_addr());
    Ok(())
}

async fn send_hello() -> io::Result<()> {
    println!("Prepare to send hello");
    let mut sender = TcpStream::connect("127.0.0.1:8081").await?;
    let message = String::from("
GET / HTTP/1.1
Content-Type: application/json
User-Agent: PostmanRuntime/7.39.0
Accept: */*
Host: 127.0.0.1:8081
Accept-Encoding: gzip, deflate, br
Connection: keep-alive
Content-Length: 3

END
");
    let message_bytes = message.as_bytes();
    sender.write(message_bytes).await?;
    println!("Hello was sent");
    sender.write(String::from("").as_ref()).await?;
    Ok(())
}

async fn handle_connection() -> io::Result<()> {
    let tcp_listener: TcpListener = TcpListener::bind("127.0.0.1:8081").await?;
    loop {
        let (client_stream, addr) = tcp_listener.accept().await?;
        println!("Address: {}", addr);
        spawn(async move {
            process_tcp_client_stream(client_stream).await
        });
        if TEST {
            break;
        }
    }
    Ok(())
}


#[tokio::main]
async fn main() -> io::Result<()> {
    spawn(async move {
        send_hello().await
    });
    handle_connection().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_say_hello() {
        spawn(async move {
            send_hello().await
        });
        handle_connection().await;

        assert!(true)
    }

    #[test]
    fn test_split() {
        let http_request_raw = String::from("
GET / HTTP/1.1
Content-Type: application/json
User-Agent: PostmanRuntime/7.39.0
Accept: */*
Host: 127.0.0.1:8081
Accept-Encoding: gzip, deflate, br
Connection: keep-alive
Content-Length: 3

END
");
        let split_lines = http_request_raw.lines().collect::<Vec<&str>>();
        let slice =&split_lines[2..];
        println!("{:?}", slice);

        assert!(true)
    }
}