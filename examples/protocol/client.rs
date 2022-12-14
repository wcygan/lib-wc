use std::io::Read;
use std::io::Result;
use std::net::TcpStream;
static ADDR: &str = "localhost:5050";

/// `cargo run --example client`
fn main() -> Result<()> {
    let mut stream = TcpStream::connect(ADDR).unwrap();
    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;
    println!("{}", buf);
    Ok(())
}
