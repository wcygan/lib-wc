#[allow(unused_macros)]
macro_rules! add{
    ($a:expr,$b:expr)=>{
        {
            $a+$b
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_numbers() {
        let x = add!(1, 2);
        assert_eq!(x, 3)
    }

    #[test]
    fn add_strings() {
        let x = add!("hello ".to_owned(), "world");
        assert_eq!(x, "hello world")
    }
}