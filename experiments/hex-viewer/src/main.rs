use std::{env::args, fs::File, io::Read, sync::Arc};

const BYTES_PERLINE: usize = 16;

fn main() {
    let filename = Arc::new(args().nth(1).expect("No filename provided"));
    let mut file = File::open(filename.as_str())
        .unwrap_or_else(|_| panic!("Could not open file '{}'", filename));

    let mut pos = 0;
    let mut buffer = [0; BYTES_PERLINE];

    while file.read_exact(&mut buffer).is_ok() {
        print!("[0x{:08x}] ", pos);

        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }
        println!();
        pos += BYTES_PERLINE;
    }
}
