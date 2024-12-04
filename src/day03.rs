use regex::Regex;

pub fn part1(input: String) -> u64 {
    let mut sum: u64 = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for (_, [n1, n2]) in re.captures_iter(&input).map(|x| x.extract()) {
        let n1_parsed = n1.parse::<u64>().expect("Could not parse n1");
        let n2_parsed = n2.parse::<u64>().expect("Could not parse n2");
        sum += n1_parsed * n2_parsed;
    }
    sum
}

pub fn part2(input: String) -> u64 {
    let mut sum: u64 = 0;
    let mut enable_mul = true;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();
    for capt_group in re.captures_iter(&input) {
        match capt_group.get(0).unwrap().as_str() {
            "do()" => enable_mul = true,
            "don't()" => enable_mul = false,
            _ if enable_mul => {
                let n1_parsed = capt_group[1].parse::<u64>().expect("Could not parse n1");
                let n2_parsed = capt_group[2].parse::<u64>().expect("Could not parse n2");
                sum += n1_parsed * n2_parsed;
            }
            _ => (),
        }
    }
    sum
}
