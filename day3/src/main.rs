// use regex::Regex;
// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let file_path = &args[1];

//     let content = fs::read_to_string(file_path).expect("it should be able to read file");

//     let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
//     let re_do = Regex::new(r"do\(\)").unwrap();
//     let re_dont = Regex::new(r"don't\(\)").unwrap();

//     let matches: Vec<regex::Captures> = re.captures_iter(&content).collect();
//     let do_matches: Vec<regex::Captures> = re_do.captures_iter(&content).collect();
//     let dont_matches: Vec<regex::Captures> = re_dont.captures_iter(&content).collect();

//     // Concaténer tous les vecteurs de matches
//     let mut all_matches = matches;
//     all_matches.extend(do_matches);
//     all_matches.extend(dont_matches);

//     // Trier les matches par position de début (start)
//     all_matches.sort_by(|a, b| {
//         let a_start = a.get(0).unwrap().start();
//         let b_start = b.get(0).unwrap().start();
//         a_start.cmp(&b_start)
//     });

//     let mut sum = 0;

//     let mut actif = true;

//     for captures in all_matches {
//         if let (Some(left_match), Some(right_match)) = (captures.get(1), captures.get(2)) {
//             if actif {
//                 if let (Ok(left_num), Ok(right_num)) = (
//                     left_match.as_str().parse::<i32>(),
//                     right_match.as_str().parse::<i32>(),
//                 ) {
//                     let product = left_num * right_num;
//                     sum += product;
//                 }
//             }
//         } else if captures.get(0).unwrap().as_str() == "do()" {
//             actif = true;
//         } else if captures.get(0).unwrap().as_str() == "don't()" {
//             actif = false;
//         }
//     }

//     println!("{}", sum);
// }

use regex::Regex;
use std::env;
use std::fs;

#[derive(Debug)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path).expect("Unable to read file");

    // Un seul regex avec alternatives nommées
    let re =
        Regex::new(r"(?:mul\((?P<l>\d{1,3}),(?P<r>\d{1,3})\))|(?P<do>do\(\))|(?P<dont>don't\(\))")
            .unwrap();

    // Transformer directement en enum
    let mut instructions: Vec<(usize, Instruction)> = re
        .captures_iter(&content)
        .filter_map(|cap| {
            let m = cap.get(0)?; // match global pour la position

            if let (Some(l), Some(r)) = (cap.name("l"), cap.name("r")) {
                // le "l" et "r" sont les noms des groupes venant du regex <l> et <r>
                let left = l.as_str().parse().ok()?;
                let right = r.as_str().parse().ok()?;
                Some((m.start(), Instruction::Mul(left, right)))
            } else if cap.name("do").is_some() {
                Some((m.start(), Instruction::Do))
            } else if cap.name("dont").is_some() {
                Some((m.start(), Instruction::Dont))
            } else {
                None
            }
        })
        .collect();
    println!("{:?}", instructions);
    // Trier par position du match
    instructions.sort_by_key(|(pos, _)| *pos);

    let mut actif = true;
    let mut sum = 0;

    for (_, instr) in instructions {
        match instr {
            Instruction::Mul(a, b) if actif => sum += a * b,
            Instruction::Do => actif = true,
            Instruction::Dont => actif = false,
            _ => {}
        }
    }

    println!("{}", sum);
}
