use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Equation {
    total: u64,
    operands: Vec<u64>,
}

pub fn part1(input: String) -> u64 {
    let equations: Vec<Equation> = input
        .split('\n')
        .filter_map(|x| {
            if let [total, operands] = x.split(": ").collect::<Vec<_>>()[..] {
                Some(Equation {
                    total: total.parse().unwrap_or_default(),
                    operands: operands
                        .split_whitespace()
                        .map(|x| x.parse().unwrap_or_default())
                        .collect(),
                })
            } else {
                None
            }
        })
        .collect();
    equations
        .iter()
        .filter(|&equation| {
            let mut mut_equation = equation.clone();
            mut_equation.operands.reverse();
            evaluate_equation_binary(mut_equation)
        })
        .map(|x| x.total)
        .sum()
}

fn evaluate_equation_binary(mut equation: Equation) -> bool {
    if let Some(leftmost) = equation.operands.pop() {
        match leftmost.cmp(&equation.total) {
            Ordering::Equal => return true,
            Ordering::Greater => return false,
            Ordering::Less => {
                if let Some(second_elem) = equation.operands.pop() {
                    let mut test_mul = equation.operands.clone();
                    let mut test_add = equation.operands.clone();
                    test_mul.push(leftmost + second_elem);
                    test_add.push(leftmost * second_elem);
                    return evaluate_equation_binary(Equation {
                        total: equation.total,
                        operands: test_add,
                    }) || evaluate_equation_binary(Equation {
                        total: equation.total,
                        operands: test_mul,
                    });
                } else {
                    return false;
                }
            }
        }
    } else {
        false
    }
}

pub fn part2(input: String) -> u64 {
    let equations: Vec<Equation> = input
        .split('\n')
        .filter_map(|x| {
            if let [total, operands] = x.split(": ").collect::<Vec<_>>()[..] {
                Some(Equation {
                    total: total.parse().unwrap_or_default(),
                    operands: operands
                        .split_whitespace()
                        .map(|x| x.parse().unwrap_or_default())
                        .collect(),
                })
            } else {
                None
            }
        })
        .collect();
    equations
        .iter()
        .filter(|&equation| {
            let mut mut_equation = equation.clone();
            mut_equation.operands.reverse();
            evaluate_equation_ternary(mut_equation)
        })
        .map(|x| x.total)
        .sum()
}

fn evaluate_equation_ternary(mut equation: Equation) -> bool {
    if let Some(leftmost) = equation.operands.pop() {
        match leftmost.cmp(&equation.total) {
            Ordering::Equal => return true,
            Ordering::Greater => return false,
            Ordering::Less => {
                if let Some(second_elem) = equation.operands.pop() {
                    let mut test_mul = equation.operands.clone();
                    let mut test_add = equation.operands.clone();
                    let mut test_concat = equation.operands.clone();
                    test_mul.push(leftmost + second_elem);
                    test_add.push(leftmost * second_elem);
                    test_concat.push(leftmost * 10u64.pow(second_elem.ilog10() + 1) + second_elem);
                    return evaluate_equation_ternary(Equation {
                        total: equation.total,
                        operands: test_add,
                    }) || evaluate_equation_ternary(Equation {
                        total: equation.total,
                        operands: test_mul,
                    }) || evaluate_equation_ternary(Equation {
                        total: equation.total,
                        operands: test_concat,
                    });
                } else {
                    return false;
                }
            }
        }
    } else {
        false
    }
}
