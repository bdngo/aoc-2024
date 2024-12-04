use std::str::FromStr;

const XMAS: &str = "XMAS";

pub fn part1(input: String) -> u64 {
    let mut num_xmas = 0;
    let word_search: Vec<_> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    for (row_idx, row) in word_search.iter().enumerate() {
        // horizontal forwards & backwards check
        for window in row.windows(XMAS.len()) {
            if &window.iter().collect::<String>() == XMAS
                || &window.iter().rev().collect::<String>() == XMAS
            {
                num_xmas += 1;
            }
        }
        // vertical check
        for (char_idx, _) in row.iter().enumerate() {
            let mut vertical_window: Vec<String> = Vec::new();
            vertical_window.push(row[row_idx..row_idx + XMAS.len()][char_idx].to_string());
            if vertical_window == XMAS {
                num_xmas += 1;
            }
        }
    }
    num_xmas
}
