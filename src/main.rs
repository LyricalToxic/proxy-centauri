mod cmd;
mod http;

use clap::Parser;
use std::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;

use cmd::cli::Cli;
use http::http_parser::{HTTPParser, HTTPRequestParser};

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
        let http_parser = HTTPRequestParser::new(&buf);
        let http_request = http_parser.parse();
        println!("Request: {}", http_request);
        break;
    }
    loop {
        socket.writable().await?;
        match socket.try_write("HTTP/1.1 200 OK\n\n<h1>Hello traveller</h1>".as_bytes()) {
            Ok(n) => {
                println!("HERE");
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    println!("Socket shutdown {:?}", socket.peer_addr());
    Ok(())
}

async fn send_hello() -> io::Result<()> {
    println!("Prepare to send hello");
    let mut sender = TcpStream::connect("127.0.0.1:7272").await?;
    let message = String::from(
        "
        GET / HTTP/1.1
        Content-Type: application/json
        User-Agent: PostmanRuntime/7.39.0
        Accept: */*
        Host: 127.0.0.1:8081
        Accept-Encoding: gzip, deflate, br
        Connection: keep-alive
        Content-Length: 3

        END
        "
        .trim(),
    );
    let message_bytes = message.as_bytes();
    sender.write(message_bytes).await?;
    println!("Hello was sent");
    sender.write(String::from("").as_ref()).await?;
    Ok(())
}

async fn handle_connection(port: u32) -> io::Result<()> {
    let tcp_listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    loop {
        let (client_stream, addr) = tcp_listener.accept().await?;
        println!("Address: {}", addr);
        spawn(async move { process_tcp_client_stream(client_stream).await });
        if TEST {
            break;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();
    // spawn(async move { send_hello().await });
    handle_connection(cli.port).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_say_hello() {
        spawn(async move { send_hello().await });
        handle_connection(7272).await.expect("PANIC");

        assert!(true)
    }

    #[test]
    fn test_split() {
        let http_request_raw = String::from(
            "
            GET / HTTP/1.1
            Content-Type: application/json
            User-Agent: PostmanRuntime/7.39.0
            Accept: */*
            Host: 127.0.0.1:8081
            Accept-Encoding: gzip, deflate, br
            Connection: keep-alive
            Content-Length: 3

            END
        ",
        );
        let split_lines = http_request_raw.lines().collect::<Vec<&str>>();
        let slice = &split_lines[2..];
        println!("{:?}", slice);

        assert!(true)
    }
}
