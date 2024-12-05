#[derive(Clone)]
struct Rule {
    before_page: u64,
    after_page: u64,
}

pub fn part1(input: String) -> u64 {
    if let [rules, updates] = &input.split("\n\n").collect::<Vec<_>>()[..] {
        let rules_list: Vec<Rule> = rules
            .split_whitespace()
            .map(|x| {
                if let [before_page, after_page] = x.split('|').collect::<Vec<_>>()[..] {
                    Rule {
                        before_page: before_page.parse().unwrap(),
                        after_page: after_page.parse().unwrap(),
                    }
                } else {
                    Rule {
                        before_page: 0,
                        after_page: 0,
                    }
                }
            })
            .collect();
        let update_list = updates.split_whitespace().map(|x| {
            x.split(',')
                .filter_map(|y| y.parse::<u64>().ok())
                .collect::<Vec<u64>>()
        });
        return update_list
            .filter(|x| is_valid_update(x.to_vec(), rules_list.clone()))
            .map(|x| get_middle_element(&x))
            .sum();
    }
    0
}

fn is_valid_update(update: Vec<u64>, rules: Vec<Rule>) -> bool {
    rules.iter().all(|x| {
        update.iter().position(|&y| y == x.before_page)
            < update.iter().position(|&y| y == x.after_page)
    })
}

fn get_middle_element(vec: &Vec<u64>) -> u64 {
    let middle_index = vec.len() / 2;
    vec[middle_index]
}

pub fn part2(input: String) -> u64 {
    0
}
