use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// Setup the TCP server details here.
const SERVER_IP: &'static str = "127.0.0.1";
const SERVER_PORT: u32 = 8080;

// This is a very basic TCP Server for Rust.

// To manually test this on Linux:
// echo "hello world" | nc 127.0.0.1 8080

// Taken from this example here
// https://codezup.com/creating-tcp-server-with-rust-hands-on/

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let peer = stream.peer_addr()?;
    println!("new connection: {peer}");

    let mut buf = [0u8; 1024];
    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            return Ok(());
        }
        stream.write_all(&buf[..n])?;
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", SERVER_IP, SERVER_PORT))?;
    println!("listening on {}:{}", SERVER_IP, SERVER_PORT);

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            if let Err(e) = handle_client(stream) {
                eprintln!("connection error: {e}");
            }
        });
    }

    Ok(())
}