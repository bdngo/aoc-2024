use bitvec::{prelude::BitVec, view::BitView};
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Equation {
    total: u64,
    operands: Vec<u64>,
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
    Concat,
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
            for i in 0usize..1 << equation.operands.len() - 1 {
                let operator_bitvec: BitVec = i.view_bits().to_bitvec();
                let operator_list = operator_bitvec
                    .into_iter()
                    .map(|b| if b { Operator::Multiply } else { Operator::Add })
                    .collect();
                if evaluate_equation(equation, operator_list) == equation.total {
                    return true;
                }
            }
            false
        })
        .map(|x| x.total)
        .sum()
}

fn evaluate_equation(equation: &Equation, operators: Vec<Operator>) -> u64 {
    let init_elem = VecDeque::from(equation.operands.clone())
        .pop_front()
        .unwrap_or_default();
    equation
        .operands
        .iter()
        .skip(1)
        .zip(operators)
        .fold(init_elem, |x, (y, z)| match z {
            Operator::Multiply => x * y,
            Operator::Add => x + y,
            Operator::Concat => x * 10u64.pow(y.ilog10() + 1) + y,
        })
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
            let mut possible_operators = (0..equation.operands.len() - 1)
                .map(|_| {
                    (0..3).map(|x| match x {
                        0 => Operator::Add,
                        1 => Operator::Multiply,
                        2 => Operator::Concat,
                        _ => unreachable!(),
                    })
                })
                .multi_cartesian_product();
            // println!("{:?}", evaluate_equation(equation, vec![Operator::Concat]));
            possible_operators
                .any(|operations| evaluate_equation(equation, operations) == equation.total)
        })
        .map(|x| x.total)
        .sum()
}
