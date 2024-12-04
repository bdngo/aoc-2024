pub fn part1(input: String) -> u64 {
    let report_matrix: Vec<Vec<i64>> = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect()
        })
        .collect();
    report_matrix
        .into_iter()
        .map(difference)
        .filter(|x| strict_monotonicity(x.to_vec()) && bounded_difference(x.to_vec()))
        .count() as u64
}
pub fn part2(input: String) -> u64 {
    let report_matrix: Vec<Vec<i64>> = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect()
        })
        .collect();
    report_matrix
        .into_iter()
        .map(apply_problem_dampener)
        .filter(|x| *x)
        .count() as u64
}

fn apply_problem_dampener(report: Vec<i64>) -> bool {
    let report_iter = report.iter();
    // check if already safe
    let new_diffs = difference(report.clone());
    if strict_monotonicity(new_diffs.clone()) && bounded_difference(new_diffs.clone()) {
        return true;
    } else {
        for (level_idx, _) in report_iter.enumerate() {
            let mut report_clone = report.clone();
            report_clone.remove(level_idx);
            let new_diffs = difference(report_clone);
            if strict_monotonicity(new_diffs.clone()) && bounded_difference(new_diffs.clone()) {
                return true;
            }
        }
        false
    }
}

fn difference(nums: Vec<i64>) -> Vec<i64> {
    let nums_iter = nums.iter();
    let nums_iter_tail = nums.iter().skip(1);
    nums_iter.zip(nums_iter_tail).map(|(x, y)| y - x).collect()
}

fn strict_monotonicity(diffs: Vec<i64>) -> bool {
    let diffs_iter = diffs.into_iter();
    diffs_iter.clone().all(|x| x < 0) || diffs_iter.clone().all(|x| x > 0)
}

fn bounded_difference(diffs: Vec<i64>) -> bool {
    diffs.into_iter().all(|x| 1 <= x.abs() && x.abs() <= 3)
}
