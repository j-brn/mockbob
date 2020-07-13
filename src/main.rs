use std::collections::HashSet;
use std::io::{stdin, BufRead};
use std::iter::FromIterator;

use mockbob_core::{Mocker, MockingStrategy, StrategyMocker};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "strategy", short = "s", default_value = "nth_char")]
    pub strategy: String,
    #[structopt(
        long = "nth",
        short = "n",
        default_value = "2",
        required_if("strategy", "nth_char"),
        help = "overrides the default step of 1 for the nth_char strategy"
    )]
    pub nth: usize,
    #[structopt(
        long = "probability",
        short = "p",
        default_value = "0.5",
        required_if("strategy", "random"),
        default_value = "0.5",
        help = "overrides the default probability of 0.5 for the random strategy (0.0 - 1.0)"
    )]
    pub probability: f64,
    #[structopt(long = "blacklist", short = "b", help = "list of characters to ignore")]
    pub blacklist: Vec<char>,
    #[structopt(
        help = "The actual text to mock. If missing, mockbob-core-cli tries to read from stdin"
    )]
    pub input: Vec<String>,
}

fn main() {
    let cli = Cli::from_args();

    let mocker = {
        let strategy = match cli.strategy.as_ref() {
            "nth_char" => MockingStrategy::NthChar(cli.nth),
            "random" => {
                // use default when a invalid value was submitted
                let probability = if cli.probability >= 0.0 && cli.probability <= 1.0 {
                    cli.probability
                } else {
                    0.5
                };

                MockingStrategy::Random(probability)
            }
            _ => MockingStrategy::default(),
        };

        let blacklist = HashSet::from_iter(cli.blacklist);

        StrategyMocker::new(strategy, blacklist)
    };

    if !cli.input.is_empty() {
        let input = cli.input.join(" ");
        println!("{}", mocker.mock(&input));
    } else {
        stdin()
            .lock()
            .lines()
            .map(|line| mocker.mock(&line.unwrap()))
            .for_each(|mocked| println!("{}", mocked));
    }
}
