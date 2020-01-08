use std::env;
use std::io::{stdin, BufRead};

fn mock(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .enumerate()
        .map(|(index, char)| {
            if index % 2 != 0 {
                char.to_ascii_uppercase()
            } else {
                char
            }
        })
        .collect()
}

fn main() {
    if env::args().count() <= 1 {
        stdin()
            .lock()
            .lines()
            .map(|line| mock(&line.unwrap()))
            .for_each(|mocked| println!("{}", mocked))
    } else {
        println!(
            "{}",
            mock(&env::args().skip(1).collect::<Vec<String>>().join(" "))
        );
    }
}
