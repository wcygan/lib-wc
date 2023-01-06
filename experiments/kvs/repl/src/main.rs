use kvs::KVStore;

fn main() {
    let mut kv = KVStore::open().unwrap();

    // A REPL that reads from stdin and writes to stdout
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "exit" => break,
            _ => {
                let mut parts = input.split_whitespace();
                let command = parts.next().unwrap();
                let key = parts.next().unwrap();

                // convert &str to [u8]
                let key_bytes = key.as_bytes();

                match command {
                    "get" => {
                        match kv.get(key_bytes) {
                            Ok(Some(value)) => {
                                // convert &[u8] to &str
                                let value_str = std::str::from_utf8(&value).unwrap();
                                println!("value: {}", value_str);
                            }
                            Ok(None) => println!("Key not found"),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    "insert" => {
                        let value = parts.next().unwrap();
                        match kv.insert(key_bytes, value.as_bytes()) {
                            Ok(()) => println!("Key set"),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    "delete" => match kv.delete(key_bytes) {
                        Ok(()) => println!("Key removed"),
                        Err(e) => println!("Error: {}", e),
                    },
                    "update" => {
                        let value = parts.next().unwrap();
                        match kv.update(key_bytes, value.as_bytes()) {
                            Ok(()) => println!("Key updated"),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    _ => println!("Unknown command"),
                }
            }
        }
    }
}
