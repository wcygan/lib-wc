use std::arch::asm;

/// This example uses the `asm!` macro to do some basic operations.
#[cfg(all(target_arch = "aarch64"))]
fn main() {
    let ten = double(five() as u64);
    println!("5 * 2 = {}", ten);
}

#[cfg(all(target_arch = "aarch64"))]
fn five() -> u8 {
    let x: u8;

    unsafe {
        asm!("mov {0:x}, 5", out(reg) x);
    }

    x
}

#[cfg(all(target_arch = "aarch64"))]
fn double(i: u64) -> u64 {
    let mut o: u64;

    unsafe {
        asm!(
        "add x1, x0, x0",
        in("x0") i,
        out("x1") o,
        );
    }

    o
}

#[cfg(all(test, target_arch = "aarch64"))]
mod tests {
    use super::*;

    #[test]
    fn five_works() {
        assert_eq!(five(), 5);
    }

    #[test]
    fn double_works() {
        assert_eq!(double(5), 10);
    }
}
