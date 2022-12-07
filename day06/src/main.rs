//! Solution to the sixth advent of code problem.
use std::collections::{HashSet, LinkedList};
use std::fs::read_to_string;
use std::path::Path;

type PacketMarkerEnd = usize;
type MessageMarkerEnd = usize;

/// Identify the packet and message end characters from the message string.
fn identify_markers(message: String) -> Option<(PacketMarkerEnd, MessageMarkerEnd)> {
    let mut packet_marker_end: Option<usize> = None;
    let mut message_marker_end: Option<usize> = None;

    let mut characters: LinkedList<char> = LinkedList::new();
    for (index, character) in message.chars().enumerate() {
        characters.push_back(character);
        if characters.len() > 14 {
            characters.pop_front();
        };

        let mut character_set: HashSet<&char> = HashSet::new();
        let mut last_n_unique = 0;
        for seen_character in characters.iter().rev() {
            character_set.insert(seen_character);

            if character_set.len() != (last_n_unique + 1) {
                break;
            };
            last_n_unique += 1;
        }

        if packet_marker_end.is_none() && last_n_unique == 4 {
            packet_marker_end = Some(index + 1);
        };
        if message_marker_end.is_none() && last_n_unique == 14 {
            message_marker_end = Some(index + 1);
            break;
        };
    }

    if let Some(packet_marker_end) = packet_marker_end {
        if let Some(message_marker_end) = message_marker_end {
            return Some((packet_marker_end, message_marker_end));
        };
    };
    None
}

fn main() {
    let path = Path::new("./input_01.txt");
    let input_string = read_to_string(path).expect("Unable to read input file 'input_01.txt'");
    if let Some((packet_end, message_end)) = identify_markers(input_string) {
        println!(
            "The packet marker ends at character {}; the message marker ends at character {}",
            packet_end, message_end
        );
    } else {
        println!("Packet or message marker missing from message.");
    }
}
