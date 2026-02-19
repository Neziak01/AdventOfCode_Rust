use std::fs::read_to_string;

fn main() {
    let input: Vec<u8> = read_to_string("data.txt")
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut final_list: Vec<i32> = Vec::new();
    for (i, elem) in input.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*elem {
                final_list.push((i / 2) as i32);
            }
        } else {
            for _ in 0..*elem {
                final_list.push(-1);
            }
        }
    }

    let mut count: i128 = 0;
    let mut left = 0;
    let mut right = final_list.len() - 1;

    while left < right {
        while left < right && final_list[left] != -1 {
            left += 1;
        }

        while left < right && final_list[right] == -1 {
            right -= 1;
        }

        if left < right {
            final_list[left] = final_list[right];
            final_list[right] = -1;
        }
    }

    for (index, elem) in final_list.iter().enumerate() {
        if *elem != -1 {
            count += *elem as i128 * index as i128;
        }
    }

    println!("{:?}", count);
}
