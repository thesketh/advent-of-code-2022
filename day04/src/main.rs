//! Solution to the fourth advent of code problem.
use std::fs::read_to_string;
use std::path::Path;

/// A range of sectors that the elves need to clean.
struct CleaningRange {
    start: usize,
    end: usize,
}

impl CleaningRange {
    /// Whether the cleaning range duplicates _any_
    /// work done in the other range.
    fn overlaps(&self, other: &CleaningRange) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    /// Whether the cleaning range fully contains the other.
    fn contains(&self, other: &CleaningRange) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    /// Whether the cleaning range completely replicates
    /// work done in the other range.
    fn replicates(&self, other: &CleaningRange) -> bool {
        self.contains(other) || other.contains(self)
    }
}

/// Parse an input string containg comma-separated cleaning ranges (which are themselves two
/// hyphen-separated integers).
fn parse_range_pairs(input_string: String) -> Vec<(CleaningRange, CleaningRange)> {
    let mut range_pairs: Vec<(CleaningRange, CleaningRange)> = Vec::new();

    for line in input_string.split('\n') {
        if line.is_empty() {
            continue;
        };

        let ranges: Vec<Vec<usize>> = line
            .split(',')
            .map(|range| range.split('-').map(|string| string.parse::<usize>().unwrap()).collect())
            .collect();

        let first_range = CleaningRange { start: ranges[0][0], end: ranges[0][1] };
        let second_range = CleaningRange { start: ranges[1][0], end: ranges[1][1] };
        range_pairs.push((first_range, second_range));
    }
    range_pairs
}

fn main() {
    let path = Path::new("./input_01.txt");
    let input_string = read_to_string(path).expect("Unable to read input file 'input_01.txt'");
    let range_pairs = parse_range_pairs(input_string);

    let replicating_pairs: usize =
        range_pairs.iter().map(|(range, other)| range.replicates(other) as usize).sum();
    println!("{} pairs of ranges completely replicate the work of the other", replicating_pairs);

    let overlapping_pairs: usize =
        range_pairs.iter().map(|(range, other)| range.overlaps(other) as usize).sum();
    println!("{} pairs of ranges replicate some work of the other", overlapping_pairs);
}
