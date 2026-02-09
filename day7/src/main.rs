use std::io::{BufRead, BufReader};
use std::{env, fs, time};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = fs::File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let start = time::Instant::now();

    let mut list: Vec<Vec<(i64, Vec<i64>)>> = Vec::new();

    for line in reader.lines() {
        let mut dictionary: Vec<(i64, Vec<i64>)> = Vec::new();

        let line = line.expect("Unable to read line");
        let (key_part, values_part) = line.split_once(':').unwrap();
        let list_values = values_part
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        dictionary.push((key_part.parse::<i64>().unwrap(), list_values));
        list.push(dictionary);
    }

    let mut total = 0;

    for dict in list {
        for (key, values) in dict {
            let nb_operator = values.len() - 1;
            for mask in 0..(3i64.pow(nb_operator as u32)) {
                // 1 ou 2 déplacé de 5 crans a gauche 100000 (binaire) = 32
                if eval(&values, mask) == key {
                    total += key;
                    break;
                }
            }
        }
    }

    println!("{}", total);
    let end = start.elapsed();
    println!("    Time: {end:.2?}");
}

fn eval(nums: &[i64], mask: i64) -> i64 {
    let mut acc = nums[0];
    let mut m = mask;

    for i in 0..(nums.len() - 1) {
        let op = m % 3;
        m /= 3;
        match op {
            0 => acc += nums[i + 1],
            1 => acc *= nums[i + 1],
            2 => acc = concat(acc, nums[i + 1]),
            _ => unreachable!(),
        }
    }
    acc
}

fn concat(a: i64, b: i64) -> i64 {
    a as i64 * 10i64.pow(b.ilog10() + 1) + b as i64
}
