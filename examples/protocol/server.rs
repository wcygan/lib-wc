use std::io::Result;
use std::io::Write;
use std::net::TcpListener;

static ADDR: &str = "localhost:5050";

/// `cargo run --example server`
fn main() -> Result<()> {
    let listener = TcpListener::bind(ADDR)?;
    println!("listening for connections on {}", ADDR);
    loop {
        let (mut stream, addr) = listener.accept()?;
        println!("new connection from: {}", addr);
        stream.write_all(b"hello world")?;
    }
}
