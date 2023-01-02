use std::io::Write;

use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::Tuple,
    IResult,
};

fn main() {
    println!("hex color code evaluation loop");
    println!("  1. Enter hex color codes of the form #ffffff");
    println!("  2. Type `exit` to quit");
    loop {
        let input = get_input();

        if input == "exit" {
            break;
        }

        match hex_color(&input) {
            Ok((_, color)) => println!("parsed color: {:?}", color),
            Err(_) => println!("cannot parse color: {:?}", input),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;

    Ok((input, Color { red, green, blue }))
}

fn prompt() {
    print!("hex> ");
    std::io::stdout().flush().unwrap();
}

fn get_input() -> String {
    prompt();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[test]
fn parse_color() {
    assert_eq!(
        hex_color("#2F14DF"),
        Ok((
            "",
            Color {
                red: 47,
                green: 20,
                blue: 223,
            }
        ))
    );
}
