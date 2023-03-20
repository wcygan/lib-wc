use std::io::{stdin, BufRead, BufReader};
use tokio::select;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(10);
    std::thread::spawn(move || get_input(tx));

    loop {
        select! {
            _ = tokio::signal::ctrl_c() => {
                println!("Done");
                return;
            }
            line = rx.recv() => {
                match line {
                    Some(s) => println!("{s}"),
                    None => {}
                }
            }
        }
    }
}

fn get_input(tx: mpsc::Sender<String>) {
    loop {
        let reader = BufReader::new(stdin());
        let mut lines = reader.lines();
        if let Some(r) = lines.next() {
            if let Ok(s) = r {
                let _ = tx.blocking_send(s);
            }
        }
    }
}
