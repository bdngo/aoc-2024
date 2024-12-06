use std::{cmp::Ordering, collections::HashMap};

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

type RuleGraph = HashMap<u64, Vec<u64>>;

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
        let rule_graph = create_rule_graph(&rules_list);
        let incorrect_updates = update_list.filter(|x| !is_valid_update(x, &rules_list));
        return incorrect_updates
            .map(|mut bad_update| {
                bad_update.sort_by(|y, z| {
                    if let Some(rule_edges) = rule_graph.get(y) {
                        if rule_edges.contains(z) {
                            return Ordering::Less;
                        }
                    } else if let Some(rule_edges) = rule_graph.get(z) {
                        if rule_edges.contains(y) {
                            return Ordering::Greater;
                        }
                    }
                    Ordering::Equal
                });
                bad_update
            })
            .map(|x| get_middle_element(&x))
            .sum();
    }
    0
}

fn create_rule_graph(rules_list: &Vec<Rule>) -> RuleGraph {
    let mut rule_graph = RuleGraph::new();
    for Rule {
        before_page,
        after_page,
    } in rules_list
    {
        rule_graph
            .entry(*before_page)
            .or_insert(Vec::new())
            .push(*after_page);
    }
    rule_graph
}
