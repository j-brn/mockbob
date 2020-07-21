use std::collections::HashSet;
use std::io::{stdin, BufRead};
use std::iter::FromIterator;

use clipboard::{ClipboardContext, ClipboardProvider};
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
    #[structopt(help = "Use the clipboard as input", long = "from_clipboard")]
    pub from_clipboard: bool,
    #[structopt(help = "Copy the output to the clipboard", long = "to_clipboard")]
    pub to_clipboard: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

    let input = if cli.from_clipboard {
        vec![clipboard.get_contents()?]
    } else if !cli.input.is_empty() {
        cli.input
    } else {
        stdin()
            .lock()
            .lines()
            .filter_map(|line| line.ok())
            .collect::<Vec<String>>()
    };

    let output = input
        .iter()
        .map(|line| mocker.mock(line))
        .collect::<Vec<String>>()
        .join("\n");

    if cli.to_clipboard {
        clipboard.set_contents(output.clone())?;
    }

    println!("{}", output);

    Ok(())
}
