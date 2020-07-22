use std::collections::HashSet;
use std::io::{stdin, Read};
use std::iter::FromIterator;

use mockbob_core::{Mocker, MockingStrategy, StrategyMocker};
use structopt::StructOpt;

#[cfg(feature = "clipboard")]
use copypasta_ext::{prelude::*, x11_fork::ClipboardContext};

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "strategy", short = "s", default_value = "random")]
    pub strategy: String,
    #[structopt(
        long = "nth",
        default_value = "2",
        required_if("strategy", "nth_char"),
        help = "overrides the default step of 1 for the nth_char strategy"
    )]
    pub nth: usize,
    #[structopt(
        long = "probability",
        default_value = "0.5",
        required_if("strategy", "random"),
        default_value = "0.5",
        help = "overrides the default probability of 0.5 for the random strategy (0.0 - 1.0)"
    )]
    pub probability: f64,
    #[structopt(long = "blacklist", help = "list of characters to ignore")]
    pub blacklist: Vec<char>,
    #[structopt(
        help = "The actual text to mock. If missing, mockbob-core-cli tries to read from stdin"
    )]
    pub input: Vec<String>,
    #[structopt(help = "Use the clipboard as input", long = "from-clipboard")]
    pub from_clipboard: bool,
    #[structopt(help = "Copy the output to the clipboard", long = "to-clipboard")]
    pub to_clipboard: bool,
    #[structopt(
        help = "don't print to stdout",
        long = "stdout",
        long = "quiet",
        short = "q"
    )]
    pub quiet: bool,
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

    let input = {
        if cli.from_clipboard {
            #[cfg(not(feature = "clipboard"))]
            panic!("mockbob was compiled without clipboard support! To enable it, please compile with the 'clipboard' feature enabled");

            #[cfg(feature = "clipboard")]
            {
                let mut clipboard = ClipboardContext::new()?;
                clipboard.get_contents()?
            }
        } else if !cli.input.is_empty() {
            cli.input.join(" ")
        } else {
            let mut buf = String::new();
            stdin().lock().read_to_string(&mut buf)?;
            buf
        }
    };

    let output = mocker.mock(&input);

    if !cli.quiet {
        println!("{}", &output);
    }

    if cli.to_clipboard {
        #[cfg(not(feature = "clipboard"))]
        panic!("mockbob was compiled without clipboard support! To enable it, please compile with the 'clipboard' feature enabled");

        #[cfg(feature = "clipboard")]
        {
            let mut clipboard = ClipboardContext::new()?;
            clipboard.set_contents(output)?
        }
    }

    Ok(())
}
