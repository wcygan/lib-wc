#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn simple_date_format() {
        let simple_date_format = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        assert!(simple_date_format.is_match("2014-01-01"));
        assert!(simple_date_format.is_match("2014-01-02"));
        assert!(simple_date_format.is_match("2014-01-03"));
        assert!(simple_date_format.is_match("2014-01-04"));
        assert!(simple_date_format.is_match("2014-01-05"));
        assert!(simple_date_format.is_match("2014-01-06"));
    }

    #[test]
    fn any_capitalization_of_hello() {
        let any_capitalization_of_hello = Regex::new(r"^[Hh][Ee][Ll][Ll][Oo]$").unwrap();
        assert!(any_capitalization_of_hello.is_match("Hello"));
        assert!(any_capitalization_of_hello.is_match("hello"));
        assert!(any_capitalization_of_hello.is_match("HELLO"));
        assert!(any_capitalization_of_hello.is_match("hElLo"));
        assert!(any_capitalization_of_hello.is_match("HeLlO"));
        assert!(any_capitalization_of_hello.is_match("hELLo"));
    }

    #[test]
    fn any_capitalization_of_hello_occurring_at_least_once_in_any_string() {
        let any_capitalization_of_hello_occurring_at_least_once_in_any_string =
            Regex::new(r"(?i).*hello.*").unwrap();
        assert!(
            any_capitalization_of_hello_occurring_at_least_once_in_any_string.is_match("Hello")
        );
        assert!(
            any_capitalization_of_hello_occurring_at_least_once_in_any_string.is_match("hello")
        );
        assert!(
            any_capitalization_of_hello_occurring_at_least_once_in_any_string.is_match("HELLO")
        );
        assert!(
            any_capitalization_of_hello_occurring_at_least_once_in_any_string.is_match("hElLo")
        );
        assert!(
            any_capitalization_of_hello_occurring_at_least_once_in_any_string.is_match("HeLlO")
        );
        assert!(
            any_capitalization_of_hello_occurring_at_least_once_in_any_string.is_match("hELLo")
        );
        assert!(
            any_capitalization_of_hello_occurring_at_least_once_in_any_string
                .is_match("Hello, world!")
        );
        assert!(
            any_capitalization_of_hello_occurring_at_least_once_in_any_string
                .is_match("hello, world!")
        );
        assert!(
            any_capitalization_of_hello_occurring_at_least_once_in_any_string
                .is_match("HELLO, WORLD!")
        );
        assert!(
            any_capitalization_of_hello_occurring_at_least_once_in_any_string
                .is_match("hElLo, wOrLd!")
        );
        assert!(
            any_capitalization_of_hello_occurring_at_least_once_in_any_string
                .is_match("HeLlO, WoRlD!")
        );
        assert!(
            any_capitalization_of_hello_occurring_at_least_once_in_any_string
                .is_match("hELLo, wORlD!")
        );
    }

    #[test]
    fn iso_8601_datetime() {
        let iso_8601_datetime = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$").unwrap();
        assert!(iso_8601_datetime.is_match("2014-01-01T00:00:00Z"));
        assert!(iso_8601_datetime.is_match("2014-01-02T00:00:00Z"));
        assert!(iso_8601_datetime.is_match("2014-01-03T00:00:00Z"));
        assert!(iso_8601_datetime.is_match("2014-01-04T00:00:00Z"));
        assert!(iso_8601_datetime.is_match("2014-01-05T00:00:00Z"));
        assert!(iso_8601_datetime.is_match("2014-01-06T00:00:00Z"));
    }

    #[test]
    fn email_address() {
        let email_address =
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}$").unwrap();
        assert!(email_address.is_match("a@b.com"));
        assert!(email_address.is_match("foo@bar.com"));
    }

    #[test]
    fn ipv4_address() {
        let ipv4_address = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap();
        assert!(ipv4_address.is_match("0.0.0.0"));
        assert!(ipv4_address.is_match("101.011.110.101"));
        assert!(ipv4_address.is_match("127.0.0.1"));
    }
}
