#[allow(unused_macros)]
macro_rules! add {
    ($a:expr,$b:expr) => {{
        $a + $b
    }};
}

#[allow(unused_macros)]
macro_rules! hello {
    ($a:expr) => {{
        String::from("hello ") + $a
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_numbers() {
        let result = add!(1, 2);
        assert_eq!(result, 3)
    }

    #[test]
    fn add_strings() {
        let result = add!("hello ".to_owned(), "world");
        assert_eq!(result, "hello world")
    }

    #[test]
    fn hello_world() {
        let result = hello!("world");
        assert_eq!(result, "hello world")
    }

    #[test]
    fn hello_number() {
        let result = hello!("123");
        assert_eq!(result, "hello 123")
    }
}
