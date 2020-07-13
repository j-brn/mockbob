use std::collections::HashSet;
use rand::{thread_rng, Rng};

/// Trait representing a mocker.
pub trait Mocker {
    /// "mocks" the given string. The format can differ based on the Mocker configuration.
    fn mock(&self, input: &str) -> String;
}

#[derive(Copy, Clone, Debug)]
pub enum MockingStrategy {
    /// Randomly decides if the given character should be replaced based on the given probability.
    Random(f64),
    /// Responds with true if the characters index is divisible trough the given step.
    NthChar(usize)
}

impl MockingStrategy {
    pub fn should_mock(self, index: usize, _value: char) -> bool {
        match self {
            MockingStrategy::Random(probability) => thread_rng().gen_bool(probability),
            MockingStrategy::NthChar(nth) => index % nth == 0
        }
    }
}

impl Default for MockingStrategy {
    fn default() -> Self {
        Self::NthChar(2)
    }
}

/// `Mocker` implementation that mocks its input based on the selected `MockingStrategy`
/// `StrategyMocker` also accepts a blacklist to be able to ignore certain characters.
///
/// ## Examples
///
/// ```rust
/// # use mockbob::{Mocker, StrategyMocker};
/// #
/// // The default mocking strategy mocks every character with even index.
/// let mocker = StrategyMocker::default();
/// let input = "mock this for me";
///
/// assert_eq!("MoCk tHiS FoR Me", mocker.mock(input));
/// ```
pub struct StrategyMocker {
    blacklist: HashSet<char>,
    strategy: MockingStrategy
}

impl StrategyMocker {
    pub fn new(strategy: MockingStrategy, blacklist: HashSet<char>) -> Self {
        Self { blacklist, strategy }
    }
}

impl Mocker for StrategyMocker {
    fn mock(&self, input: &str) -> String {
        input
            .to_lowercase()
            .chars()
            .enumerate()
            .map(|(index, char)| {
                if self.strategy.should_mock(index, char) && !self.blacklist.contains(&char){
                    char.to_uppercase().to_string()
                } else {
                    char.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }
}

impl Default for StrategyMocker {
    fn default() -> Self {
        Self::new(MockingStrategy::NthChar(2), HashSet::new())
    }
}