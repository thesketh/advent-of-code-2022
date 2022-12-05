//! Solution to the third advent of code problem.
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Item(char);

impl Item {
    fn score(&self) -> u64 {
        if !self.0.is_ascii_alphabetic() {
            return 0;
        }

        let ord_base: u64 = if self.0.is_ascii_lowercase() { 96 } else { 38 };
        (self.0 as u64) - ord_base
    }
}

/// Load a newline-separated sequence of items in each elf's rucksack, where the items in the
/// first compartment occupy the first half of the line and the items in the second occupy the
/// second half.
///
/// Return a Result with a tuple containing:
///  - a Vec of HashSets containing Items which overlap between both compartments for each elf.
///  - a Vec of Items which overlap between each trio of elves (thus, their badges).
fn load_compartment_overlaps_badges(
    data: String,
) -> Result<(Vec<HashSet<Item>>, Vec<Item>), &'static str> {
    let mut overlaps: Vec<HashSet<Item>> = Vec::new();
    let mut badges: Vec<Item> = Vec::new();
    let mut elf_group: Vec<HashSet<Item>> = Vec::with_capacity(3);

    for all_contents in data.split('\n') {
        if all_contents.is_empty() {
            continue;
        }

        let split_point = all_contents.len() / 2;
        let mut items = all_contents.chars().map(Item);
        let contents = items.by_ref();

        let first_compartment: HashSet<Item> = HashSet::from_iter(contents.take(split_point));
        let second_compartment: HashSet<Item> = HashSet::from_iter(contents.take(split_point));
        let whole_bag = first_compartment.union(&second_compartment).copied().collect();
        let overlap = first_compartment.intersection(&second_compartment).copied().collect();

        overlaps.push(overlap);
        elf_group.push(whole_bag);

        if elf_group.len() == 3 {
            let elf_overlap = &(&elf_group[0] & &elf_group[1]) & &elf_group[2];
            if elf_overlap.len() != 1 {
                return Err("Expected elf overlap to contain only one item for each group");
            }
            if let Some(badge) = elf_overlap.into_iter().next() {
                badges.push(badge);
            }
            elf_group.clear();
        }
    }

    Ok((overlaps, badges))
}

/// Score the overlaps.
fn score_overlaps(overlaps: &[HashSet<Item>]) -> u64 {
    overlaps.iter().map(|overlap| overlap.iter().map(Item::score).sum::<u64>()).sum()
}

/// Score the badges.
fn score_badges(badges: &[Item]) -> u64 {
    badges.iter().map(Item::score).sum()
}

/// Run the advent of code solution.
fn main() {
    let path = Path::new("./input_01.txt");
    let input_string = read_to_string(path).expect("Unable to read input file 'input_01.txt'");

    let (overlaps, badges) =
        load_compartment_overlaps_badges(input_string).expect("Unable to parse overlaps/badges");
    let overlap_score = score_overlaps(&overlaps);
    let badge_score: u64 = score_badges(&badges);

    println!("The combined score of the overlapping items is {}", overlap_score);
    println!("The combined score of the badges is {}", badge_score);
}

/// Tests for the item overlap calculation.
#[cfg(test)]
mod test {
    use super::*;

    /// Test that item scoring works as expected.
    #[test]
    fn test_item_scoring() {
        assert_eq!(Item('a').score(), 1);
        assert_eq!(Item('A').score(), 27);
        assert_eq!(Item(' ').score(), 0);
    }
}
