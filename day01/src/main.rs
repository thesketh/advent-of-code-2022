//! Solution to the first advent of code problem.
use std::fs::read_to_string;
use std::path::Path;

type Calories = u64;

/// An Elf, holding some snacks.
#[derive(Debug, PartialEq, Eq)]
struct Elf {
    /// A vec containing the caloric content of each of the elf's snacks.
    calorie_counts: Vec<Calories>,
}

impl Elf {
    /// Calculate the total calories held by the elf.
    fn total_calories(&self) -> Calories {
        self.calorie_counts.iter().sum()
    }
}

/// Load the elves from a string containing the elves and their calorie counts.
fn load_elves(data: String) -> Vec<Elf> {
    let mut elves: Vec<Elf> = Vec::new();

    let mut calorie_counts: Vec<Calories> = Vec::new();
    for row in data.split("\n") {
        if row == "" {
            elves.push(Elf { calorie_counts });
            calorie_counts = Vec::new();
            continue;
        }

        if let Ok(calorie_count) = row.parse::<Calories>() {
            calorie_counts.push(calorie_count);
        }
    }

    if calorie_counts.len() > 0 {
        elves.push(Elf { calorie_counts });
    }
    elves
}

/// Get a descending order vector of the total numbers of calories held by the elves.
fn get_ordered_calorie_counts(elves: &Vec<Elf>) -> Vec<Calories> {
    let total_calorie_counts = elves.iter().map(|elf| elf.total_calories());
    let mut total_calorie_counts_vec: Vec<Calories> = total_calorie_counts.collect();
    total_calorie_counts_vec.sort_by(|a, b| b.cmp(a));

    total_calorie_counts_vec
}

/// Run the advent of code solution.
fn main() {
    let path = Path::new("./input_01.txt");
    let input_string = read_to_string(path).expect("Unable to read input file 'input_01.txt'");

    let elves = load_elves(input_string);
    let ordered_calorie_counts = get_ordered_calorie_counts(&elves);
    println!(
        "The three elves with the most calories are holding {}, {}, and {}.",
        ordered_calorie_counts[0], ordered_calorie_counts[1], ordered_calorie_counts[2]
    );
    println!(
        "The top three elves are holding {} calories in total.",
        &ordered_calorie_counts[..3].iter().sum::<Calories>(),
    )
}

/// Tests for the elf calorie counting functionality.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_calorie_count() {
        let calorie_counts: Vec<Calories> = vec![36, 54, 23];
        let elf = Elf { calorie_counts };
        assert_eq!(elf.total_calories(), 113);
    }

    #[test]
    fn test_elf_parsing() {
        let elf_text = "1000\n2000\n\n1000\n3000\n".to_string();
        let elves = load_elves(elf_text);
        let expected: Vec<Elf> = vec![
            Elf {
                calorie_counts: vec![1000, 2000],
            },
            Elf {
                calorie_counts: vec![1000, 3000],
            },
        ];

        assert_eq!(elves, expected);
    }

    #[test]
    fn test_get_calories() {
        let elves: Vec<Elf> = vec![
            Elf {
                calorie_counts: vec![1000, 2000],
            },
            Elf {
                calorie_counts: vec![10000],
            },
            Elf {
                calorie_counts: vec![1000, 3000],
            },
        ];
        assert_eq!(get_ordered_calorie_counts(&elves), vec![10000, 4000, 3000]);
    }
}
