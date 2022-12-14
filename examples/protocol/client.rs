use lib_wc::protocols::pingpong::{PingPong, PingPongPacket};
use std::io::Result;
use std::io::{Read, Write};
use std::net::TcpStream;

static ADDR: &str = "localhost:5050";

/// `cargo run --example client`
fn main() -> Result<()> {
    let mut packet = PingPongPacket {
        pingpong: Some(PingPong::Ping(10)),
    };

    // Connect to the server
    let mut stream = TcpStream::connect(ADDR).unwrap();

    loop {
        // Send the packet
        let mut buf = bincode::serialize(&packet).unwrap();
        stream.write(&buf).unwrap();

        // Read the response
        stream.read(&mut buf).unwrap();
        packet = bincode::deserialize(&buf).unwrap();

        // Check if we're done
        packet = match packet.pingpong {
            Some(ping) => {
                println!("{:?}", ping);

                PingPongPacket {
                    pingpong: ping.bounce(),
                }
            }
            None => {
                stream.shutdown(std::net::Shutdown::Both)?;
                break;
            }
        }
    }

    Ok(())
}
