use lazy_static::lazy_static;
use regex::Regex;

struct Token {
    type_of: String,
    value: String,
}

fn tokenizer(input: &str) {
    let mut current = 0;
    let mut tokens: Vec<Token> = Vec::new();

    let chars = input.as_bytes();

    while current < input.len() {
        let mut char = chars[current] as char; // works only for ascii

        if char == '(' {
            tokens.push(Token {
                type_of: String::from("paran"),
                value: String::from("("),
            });
            current += 1;
            continue;
        }

        if char == ')' {
            tokens.push(Token {
                type_of: String::from("paran"),
                value: String::from(")"),
            });
            current += 1;
            continue;
        }

        lazy_static! {
            static ref WHITESPACE: Regex = Regex::new(r"\s").unwrap();
        }
        if WHITESPACE.is_match(&char.to_string().to_owned()) {
            current += 1;
            continue;
        }

        lazy_static! {
            static ref NUMBER: Regex = Regex::new(r"[0-9]").unwrap();
        }
        if NUMBER.is_match(&char.to_string().to_owned()) {
            let mut value = String::from("");
            while NUMBER.is_match(&char.to_string().to_owned()) {
                value.push(char);
                current += 1;
                char = chars[current] as char;
            }
            tokens.push(Token {
                type_of: String::from("number"),
                value: value,
            });
            continue;
        }

        panic!("I don't know this token: {}", char);
    }
}

fn main() {
    println!("Hello, world1!");
    tokenizer("() 123");
    println!("Hello, world!");
}
