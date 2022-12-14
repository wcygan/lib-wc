use lib_wc::protocols::pingpong::PingPongPacket;
use std::io::Write;
use std::io::{Read, Result};
use std::net::{TcpListener, TcpStream};

static ADDR: &str = "localhost:5050";

/// `cargo run --example server`
fn main() -> Result<()> {
    // Create a listener
    let listener = TcpListener::bind(ADDR)?;
    println!("listening for connections on {}", ADDR);

    // Listen for connections
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    println!("new client: {}", stream.peer_addr()?);

    loop {
        // Read the packet
        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap();
        let mut packet: PingPongPacket = bincode::deserialize(&buf).unwrap();

        // Check if we're done
        packet = match packet.pingpong {
            Some(ping) => {
                println!("{:?}", ping);

                PingPongPacket {
                    pingpong: ping.bounce(),
                }
            }
            None => {
                break;
            }
        };

        // Send the packet
        stream.write_all(&bincode::serialize(&packet).unwrap())?;
    }

    Ok(())
}
