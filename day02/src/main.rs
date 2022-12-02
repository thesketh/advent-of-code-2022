//! Solution to the second advent of code problem.
use std::cmp::Ordering;
use std::fs::read_to_string;
use std::path::Path;

type Score = u64;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Hand::Rock => match other {
                Hand::Rock => Some(Ordering::Equal),
                Hand::Paper => Some(Ordering::Less),
                Hand::Scissors => Some(Ordering::Greater),
            },
            Hand::Paper => match other {
                Hand::Rock => Some(Ordering::Greater),
                Hand::Paper => Some(Ordering::Equal),
                Hand::Scissors => Some(Ordering::Less),
            },
            Hand::Scissors => match other {
                Hand::Rock => Some(Ordering::Less),
                Hand::Paper => Some(Ordering::Greater),
                Hand::Scissors => Some(Ordering::Equal),
            },
        }
    }
}

impl Hand {
    /// Score a round between the hand and another hand.
    fn score_round(&self, other: &Self) -> Score {
        let base_score: Score = match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };

        let round_score: Score = match self.partial_cmp(other) {
            Some(Ordering::Less) => 0,
            Some(Ordering::Equal) => 3,
            Some(Ordering::Greater) => 6,
            None => 0,
        };

        base_score + round_score
    }
}

/// The action we've been advised to take in our cheat sheet.
enum Action {
    X,
    Y,
    Z,
}

impl Action {
    /// Get a hand based on the assumption that the action refers to a specific hand.
    fn assume_expected_hand(&self) -> Hand {
        match self {
            Action::X => Hand::Rock,
            Action::Y => Hand::Paper,
            Action::Z => Hand::Scissors,
        }
    }

    /// Get a hand based on the assumption that the action refers to a specific result
    /// (e.g. Lose/Draw/Win).
    fn assume_expected_result(&self, opponent_hand: &Hand) -> Hand {
        match (self, opponent_hand) {
            (Action::X, Hand::Rock) => Hand::Scissors,
            (Action::X, Hand::Paper) => Hand::Rock,
            (Action::X, Hand::Scissors) => Hand::Paper,
            (Action::Y, _) => *opponent_hand,
            (Action::Z, Hand::Rock) => Hand::Paper,
            (Action::Z, Hand::Paper) => Hand::Scissors,
            (Action::Z, Hand::Scissors) => Hand::Rock,
        }
    }
}

/// Our opponent's choice.
type OpponentChoice = Hand;
/// A round, consisting of our opponent's choice and an action.
type Round = (OpponentChoice, Action);

/// Load opponents' hands and our actions from the input file text.
fn load_hands(data: String) -> Vec<Round> {
    let mut rounds: Vec<Round> = Vec::new();

    for row in data.split('\n') {
        if row.is_empty() {
            continue;
        }
        let mut chars = row.chars();

        let opponent_hand = match chars.next() {
            Some('A') => Some(Hand::Rock),
            Some('B') => Some(Hand::Paper),
            Some('C') => Some(Hand::Scissors),
            _ => None,
        };

        let action = match chars.nth(1) {
            Some('X') => Some(Action::X),
            Some('Y') => Some(Action::Y),
            Some('Z') => Some(Action::Z),
            _ => None,
        };

        if let Some(opponent_hand) = opponent_hand {
            if let Some(action) = action {
                rounds.push((opponent_hand, action));
            }
        }
    }

    rounds
}

/// Run the advent of code solution.
fn main() {
    let path = Path::new("./input_01.txt");
    let input_string = read_to_string(path).expect("Unable to read input file 'input_01.txt'");

    let rounds = load_hands(input_string);

    let mut assume_hand_score: u64 = 0;
    let mut assume_result_score: u64 = 0;

    for (opponent_hand, action) in rounds {
        let our_hand_from_hand = action.assume_expected_hand();
        assume_hand_score += our_hand_from_hand.score_round(&opponent_hand);

        let our_hand_from_result = action.assume_expected_result(&opponent_hand);
        assume_result_score += our_hand_from_result.score_round(&opponent_hand);
    }

    println!("Assuming the 'hand' cheating strategy, our score is {}", assume_hand_score);
    println!("Assuming the 'result' cheating strategy, our score is {}", assume_result_score);
}

/// Tests for the elf calorie counting functionality.
#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the same hands are equal.
    #[test]
    fn test_hand_equality() {
        assert_eq!(Hand::Rock, Hand::Rock);
        assert_eq!(Hand::Paper, Hand::Paper);
        assert_eq!(Hand::Scissors, Hand::Scissors);
    }

    /// Test that hand ordering is as expected.
    #[test]
    fn test_hand_comparisons() {
        assert!(Hand::Rock > Hand::Scissors);
        assert!(Hand::Rock < Hand::Paper);
        assert!(Hand::Paper > Hand::Rock);
        assert!(Hand::Paper < Hand::Scissors);
        assert!(Hand::Scissors > Hand::Paper);
        assert!(Hand::Scissors < Hand::Rock);
    }

    /// Test that scores between hands are as expected.
    #[test]
    fn test_scoring() {
        assert_eq!(Hand::Rock.score_round(&Hand::Rock), 4);
        assert_eq!(Hand::Rock.score_round(&Hand::Paper), 1);
        assert_eq!(Hand::Rock.score_round(&Hand::Scissors), 7);
        assert_eq!(Hand::Paper.score_round(&Hand::Rock), 8);
        assert_eq!(Hand::Paper.score_round(&Hand::Paper), 5);
        assert_eq!(Hand::Paper.score_round(&Hand::Scissors), 2);
        assert_eq!(Hand::Scissors.score_round(&Hand::Rock), 3);
        assert_eq!(Hand::Scissors.score_round(&Hand::Paper), 9);
        assert_eq!(Hand::Scissors.score_round(&Hand::Scissors), 6);
    }

    #[test]
    fn test_get_hand_from_action_assuming_hand() {
        assert_eq!(Action::X.assume_expected_hand(), Hand::Rock);
        assert_eq!(Action::Y.assume_expected_hand(), Hand::Paper);
        assert_eq!(Action::Z.assume_expected_hand(), Hand::Scissors);
    }

    #[test]
    fn test_get_hand_from_action_assuming_result() {
        assert_eq!(Action::X.assume_expected_result(&Hand::Rock), Hand::Scissors);
        assert_eq!(Action::X.assume_expected_result(&Hand::Paper), Hand::Rock);
        assert_eq!(Action::X.assume_expected_result(&Hand::Scissors), Hand::Paper);
        assert_eq!(Action::Y.assume_expected_result(&Hand::Rock), Hand::Rock);
        assert_eq!(Action::Y.assume_expected_result(&Hand::Paper), Hand::Paper);
        assert_eq!(Action::Y.assume_expected_result(&Hand::Scissors), Hand::Scissors);
        assert_eq!(Action::Z.assume_expected_result(&Hand::Rock), Hand::Paper);
        assert_eq!(Action::Z.assume_expected_result(&Hand::Paper), Hand::Scissors);
        assert_eq!(Action::Z.assume_expected_result(&Hand::Scissors), Hand::Rock);
    }
}
