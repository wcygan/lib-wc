#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn spawn_a_thread() {
        let t = thread::spawn(|| {
            println!("Hello from a thread!");
        });

        t.join().unwrap()
    }

    #[test]
    fn scoped_threads() {
        // thread::scope spawns "scoped" threads that cannot outlive the scope of the
        // closures that we pass to it.
        // scopes allow us to safely borrow data from the parent thread
        let numbers = vec![1, 2, 3];

        thread::scope(|s| {
            s.spawn(|| {
                println!("length: {}", numbers.len());
            });

            s.spawn(|| {
                for n in &numbers {
                    println!("{n}");
                }
            });
        });
    }

    #[test]
    fn arc_shadowing() {
        let x = Arc::new(5);

        let t = thread::spawn({
            // create a value "x" in a new scope
            // the value "x" shadows the value "x" in the parent scope
            let x = x.clone();
            move || {
                println!("{}", x);
            }
        });

        t.join().unwrap();

        println!("{}", x);
    }

    #[test]
    fn foo() {}
}
