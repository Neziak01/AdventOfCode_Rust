use std::{fs::read_to_string, time};

#[derive(PartialEq, Eq)]
enum Part {
    One,
    Two,
}

fn main() {
    let input = read_to_string("data.txt").unwrap();
    let start = time::Instant::now();

    let equations: Vec<(u64, Vec<u64>)> = input
        .lines()
        .filter_map(|l| l.split_once(':'))
        .map(|(target, nums)| {
            let target = target.parse().unwrap();
            let nums = nums.split_whitespace().flat_map(|s| s.parse()).collect();
            (target, nums)
        })
        .collect();

    let p1 = solve_equations(&equations, Part::One);
    let p2 = solve_equations(&equations, Part::Two);

    let end = start.elapsed();
    println!("2024 Day 07:");
    println!("  Part 1: {p1}");
    println!("  Part 2: {p2}");
    println!("    Time: {end:.2?}");
}

fn solve_equations(equations: &[(u64, Vec<u64>)], part: Part) -> u64 {
    equations
        .iter()
        .map(|(target, nums)| {
            let mut results: Vec<u64> = vec![nums[0]];
            for num in &nums[1..] {
                let mut this_results = Vec::new();
                for result in results {
                    this_results.extend(
                        [
                            Some(result + num),
                            Some(result * num),
                            match part {
                                Part::One => None,
                                Part::Two => format!("{}{}", result, num).parse().ok(),
                            },
                        ]
                        .iter()
                        .filter(|x| x.is_some_and(|x| x <= *target))
                        .flatten(),
                    );
                }
                results = this_results;
            }
            *results.iter().find(|&x| x == target).unwrap_or(&0)
        })
        .sum()
}
