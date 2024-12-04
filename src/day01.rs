use std::collections::HashMap;

struct InputDay01 {
    left: Vec<u64>,
    right: Vec<u64>,
}

pub fn part1(input: String) -> u64 {
    let mut input_lists = InputDay01 {
        left: Vec::new(),
        right: Vec::new(),
    };
    for line in input.lines() {
        let row: Vec<u64> = line
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        if let [left_int, right_int] = row.as_slice() {
            input_lists.left.push(*left_int);
            input_lists.right.push(*right_int);
        }
    }
    input_lists.left.sort();
    input_lists.right.sort();
    input_lists
        .left
        .into_iter()
        .zip(input_lists.right.into_iter())
        .map(|(x, y)| x.abs_diff(y))
        .sum()
}

pub fn part2(input: String) -> u64 {
    let mut input_lists = InputDay01 {
        left: Vec::new(),
        right: Vec::new(),
    };
    for line in input.lines() {
        let row: Vec<u64> = line
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        if let [left_int, right_int] = row.as_slice() {
            input_lists.left.push(*left_int);
            input_lists.right.push(*right_int);
        }
    }
    let mut right_occurrences: HashMap<u64, u64> = HashMap::new();
    for right_key in input_lists.right {
        *right_occurrences.entry(right_key).or_insert(0) += 1;
    }
    input_lists
        .left
        .into_iter()
        .map(|x| x * right_occurrences.get(&x).unwrap_or(&0))
        .sum()
}
