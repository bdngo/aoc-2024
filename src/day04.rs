const XMAS: &str = "XMAS";
const SENTINEL_CHAR: char = '~';

pub fn part1(input: String) -> u64 {
    let mut num_xmas = 0;
    let mut word_search: Vec<_> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    // pad word_search
    for _ in 0..XMAS.len() {
        word_search.insert(
            0,
            vec![SENTINEL_CHAR; word_search[0].len() + 2 * XMAS.len()],
        );
        word_search.push(vec![SENTINEL_CHAR; word_search[0].len() + 2 * XMAS.len()]);
    }
    let word_search = word_search
        .into_iter()
        .map(|mut x| {
            for _ in 0..XMAS.len() {
                x.insert(0, SENTINEL_CHAR);
                x.push(SENTINEL_CHAR);
            }
            x
        })
        .collect::<Vec<_>>();
    println!("{:?}", word_search);
    for (row_idx, row) in word_search.iter().enumerate().skip(XMAS.len()) {
        // horizontal forwards & backwards check
        for window in row.windows(XMAS.len()) {
            if &window.iter().collect::<String>() == XMAS
                || &window.iter().rev().collect::<String>() == XMAS
            {
                num_xmas += 1;
            }
        }

        // vertical & diagonal check
        for (char_idx, _) in row.iter().enumerate().skip(XMAS.len()) {
            let mut vertical_window: Vec<char> = Vec::new();
            for i in 0..XMAS.len() {
                vertical_window.push(word_search[row_idx + i][char_idx]);
            }
            if &vertical_window.iter().collect::<String>() == XMAS
                || &vertical_window.iter().rev().collect::<String>() == XMAS
            {
                num_xmas += 1;
            }
            let mut diagonal_down_window: Vec<char> = Vec::new();
            let mut diagonal_up_window: Vec<char> = Vec::new();
            for i in 0..XMAS.len() {
                diagonal_down_window.push(word_search[row_idx + i][char_idx + i]);
                diagonal_up_window.push(word_search[row_idx - i][char_idx + i]);
            }
            if &diagonal_down_window.iter().collect::<String>() == XMAS
                || &diagonal_down_window.iter().rev().collect::<String>() == XMAS
                || &diagonal_up_window.iter().collect::<String>() == XMAS
                || &diagonal_up_window.iter().rev().collect::<String>() == XMAS
            {
                num_xmas += 1;
            }
        }
    }
    num_xmas
}
