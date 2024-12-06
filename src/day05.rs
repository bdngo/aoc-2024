#[derive(Clone, Debug)]
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
            .filter(|x| is_valid_update(x, &rules_list))
            .map(|x| get_middle_element(&x))
            .sum();
    }
    0
}

fn is_valid_update(update: &Vec<u64>, rules: &Vec<Rule>) -> bool {
    rules.iter().all(|x| {
        if update.contains(&x.before_page) && update.contains(&x.after_page) {
            update.iter().position(|&y| y == x.before_page)
                < update.iter().position(|&y| y == x.after_page)
        } else {
            true
        }
    })
}

fn get_middle_element(vec: &Vec<u64>) -> u64 {
    let middle_index = vec.len() / 2;
    vec[middle_index]
}

pub fn part2(input: String) -> u64 {
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
        let incorrect_updates = update_list.filter(|x| !is_valid_update(x, &rules_list));
        let mut sum = 0;
        for mut bad_update in incorrect_updates.clone() {
            let (mut corrections, mut other_rules) =
                get_corrections(&bad_update, rules_list.clone());
            while !corrections.is_empty() {
                // correct update
                let correction = corrections.pop().unwrap();
                if let (Some(before_idx), Some(after_idx)) = (
                    bad_update.iter().position(|&x| x == correction.before_page),
                    bad_update.iter().position(|&x| x == correction.after_page),
                ) {
                    bad_update.swap(before_idx, after_idx);
                }
                // println!("{:?}", bad_update);

                // add potentially new violating rules
                let mut to_remove = Vec::new();
                for (i, rule) in other_rules.iter().enumerate() {
                    if let (Some(before_idx), Some(after_idx)) = (
                        bad_update.iter().position(|&x| x == rule.before_page),
                        bad_update.iter().position(|&x| x == rule.after_page),
                    ) {
                        // println!("{:?}", (before_idx, after_idx));
                        if before_idx > after_idx {
                            to_remove.push(i);
                            corrections.push(rule.clone());
                        }
                    }
                }
                to_remove.reverse();
                for i in to_remove {
                    other_rules.remove(i);
                }
            }
            sum += get_middle_element(&bad_update);
        }
        return sum;
    }
    0
}

fn get_corrections(update: &Vec<u64>, rules: Vec<Rule>) -> (Vec<Rule>, Vec<Rule>) {
    rules.into_iter().partition(|x| {
        if update.contains(&x.before_page) && update.contains(&x.after_page) {
            update.iter().position(|&y| y == x.before_page)
                > update.iter().position(|&y| y == x.after_page)
        } else {
            false
        }
    })
}
