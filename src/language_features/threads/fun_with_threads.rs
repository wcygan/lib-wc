#[cfg(test)]
mod tests {

    #[test]
    fn spawn_a_thread() {
        let t = std::thread::spawn(|| {
            println!("Hello from a thread!");
        });

        t.join().unwrap()
    }
}