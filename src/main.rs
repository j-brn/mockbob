use std::env;
use std::io::{stdin, BufRead};

fn mock(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .enumerate()
        .map(|(index, c)| {
            if c.is_alphanumeric() {
                if index % 2 != 0 {
                    let v: Vec<char> = c.to_uppercase().collect();
                    v.into_iter().collect::<String>()
                } else {
                    let v: Vec<char> = c.to_lowercase().collect();
                    v.into_iter().collect::<String>()
                }
            } else {
                c.to_string()
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
