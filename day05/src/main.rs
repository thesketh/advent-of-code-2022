//! Solution to the fifth advent of code problem.
use std::collections::{HashMap, LinkedList};
use std::fs::read_to_string;
use std::path::Path;

/// A move instruction.
#[derive(Debug)]
struct Instruction {
    n_crates: u32,
    move_from: u32,
    move_to: u32,
}

impl Instruction {
    fn apply(&self, stacks: &mut HashMap<u32, LinkedList<String>>) {
        for _ in 0..self.n_crates {
            let source_stack = stacks.get_mut(&self.move_from).unwrap();
            let some_crate = source_stack.pop_back().unwrap();

            let dest_stack = stacks.get_mut(&self.move_to).unwrap();
            dest_stack.push_back(some_crate);
        }
    }

    fn apply_move_multi(&self, stacks: &mut HashMap<u32, LinkedList<String>>) {
        let mut to_move = Vec::with_capacity(self.n_crates.try_into().unwrap());

        let source_stack = stacks.get_mut(&self.move_from).unwrap();
        for _ in 0..self.n_crates {
            let some_crate = source_stack.pop_back().unwrap();
            to_move.push(some_crate);
        }

        let dest_stack = stacks.get_mut(&self.move_to).unwrap();
        for some_crate in to_move.into_iter().rev() {
            dest_stack.push_back(some_crate);
        }
    }
}

fn parse_stacks_instructions(
    input_string: String,
) -> (HashMap<u32, LinkedList<String>>, Vec<Instruction>) {
    let mut lines = input_string.split('\n');
    let mut stack_lines: Vec<String> = Vec::new();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        stack_lines.push(line.to_string());
    }

    let stack_numbers: Vec<u32> = stack_lines
        .pop()
        .unwrap()
        .chars()
        .filter(|character| character.is_numeric())
        .map(|character| character.to_digit(10).unwrap())
        .collect();
    let mut stacks: Vec<LinkedList<String>> = vec![LinkedList::new(); stack_numbers.len()];

    for line in stack_lines {
        let mut start_index: usize = 0;
        let mut end_index: usize = 3;

        for stack in &mut stacks {
            if start_index > line.len() || end_index > line.len() {
                break;
            };
            let some_crate = line[start_index..end_index].to_string();
            if some_crate != "   " {
                stack.push_front(some_crate);
            };
            start_index += 4;
            end_index += 4;
        }
    }
    let stack_map: HashMap<u32, LinkedList<String>> =
        stack_numbers.into_iter().zip(stacks.into_iter()).collect();

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let line_split: Vec<&str> = line.split(' ').collect();
        let n_crates = line_split[1].parse::<u32>().unwrap();
        let move_from = line_split[3].parse::<u32>().unwrap();
        let move_to = line_split[5].parse::<u32>().unwrap();

        instructions.push(Instruction { n_crates, move_from, move_to });
    }

    (stack_map, instructions)
}

fn main() {
    let path = Path::new("./input_01.txt");
    let input_string = read_to_string(path).expect("Unable to read input file 'input_01.txt'");

    let (mut stack_map, instructions) = parse_stacks_instructions(input_string);
    let mut multistack_map = stack_map.clone();

    let mut ordered_keys: Vec<u32> = stack_map.keys().cloned().collect();
    ordered_keys.sort();

    for instruction in &instructions {
        instruction.apply(&mut stack_map);
        instruction.apply_move_multi(&mut multistack_map);
    }

    let mut containers: Vec<char> = Vec::new();
    for key in &ordered_keys {
        let container = stack_map.get_mut(key).unwrap().pop_back().unwrap();
        containers.push(container.chars().nth(1).unwrap());
    }
    let mut container_string: String = containers.iter().cloned().collect();
    println!("Containers in singly-moved stacks: {}", container_string);

    containers.clear();
    for key in &ordered_keys {
        let container = multistack_map.get_mut(key).unwrap().pop_back().unwrap();
        containers.push(container.chars().nth(1).unwrap());
    }
    container_string = containers.iter().cloned().collect();
    println!("Containers in multi-moved stacks: {}", &container_string);
}
