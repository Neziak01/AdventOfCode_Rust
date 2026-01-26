// use std::io::{BufRead, BufReader};
// use std::time::Instant;
// use std::{collections::HashMap, env, fs};

// fn main() {
//     let start = Instant::now();

//     let args: Vec<String> = env::args().collect();
//     let file_path = &args[1];

//     let file = fs::File::open(file_path).expect("Unable to open file");
//     let reader = BufReader::new(file);

//     let mut somme: i32 = 0;

//     let mut list_of_lines: Vec<(i32, i32)> = Vec::new();
//     let mut list_of_values: Vec<HashMap<i32, i32>> = Vec::new();

//     // Parcourir chaque ligne du fichier
//     for line in reader.lines() {
//         let line = line.expect("Unable to read line");
//         if let Some((left, right)) = delimiter(&line) {
//             list_of_lines.push((left, right));
//         } else if line.contains(",") {
//             let mut values_pos: HashMap<i32, i32> = HashMap::new();
//             let values: Vec<i32> = line
//                 .split(',')
//                 .map(|s| s.trim().parse::<i32>().expect("Unable to parse value"))
//                 .collect();
//             for (i, value) in values.iter().enumerate() {
//                 values_pos.insert(*value, i as i32);
//             }
//             list_of_values.push(values_pos);
//         }
//     }

//     //println!("{:?}", list_of_lines);
//     println!("{:?}", list_of_values);

//     let mut dictionary: HashMap<i32, Vec<i32>> = HashMap::new();

//     // Créer un dictionnaire pour stocker les valeurs de chaque clé
//     for tuple in list_of_lines {
//         if dictionary.contains_key(&tuple.0) {
//             dictionary.get_mut(&tuple.0).unwrap().push(tuple.1);
//         } else {
//             dictionary.insert(tuple.0, vec![tuple.1]);
//         }
//     }

//     let mut pages: Vec<(&i32, &i32)> = Vec::new();
//     let mut invalid_pages_list: Vec<Vec<(&i32, &i32)>> = Vec::new();

//     for list in &list_of_values {
//         // valeur + position
//         let mut valid = true;

//         for (x, &pos_x) in list {
//             if let Some(rules) = dictionary.get(x) {
//                 for y in rules {
//                     // listes des Y pour un X
//                     if let Some(&pos_y) = list.get(y) {
//                         // s'il existe des Y de la liste présent dans l'upload
//                         if pos_x >= pos_y {
//                             valid = false;
//                         }
//                     }
//                 }
//             }
//             if !valid {
//                 break;
//             }
//         }

//         if valid {
//             pages.clear();
//             pages.extend(list.iter());
//             pages.sort_by_key(|&(_, pos)| pos);
//             let middle_page = pages[pages.len() / 2].0;
//             println!("Page du milieu: {}", middle_page);
//             somme += middle_page;
//         } else {
//             invalid_pages_list.push(list.iter().collect());
//         }
//     }
//     println!("somme: {}", somme);

//     //let duration = start.elapsed();
//     //println!("Temps d'exécution: {:?}", duration);
// }

// fn delimiter(line: &str) -> Option<(i32, i32)> {
//     if line.find("|").is_none() {
//         return None;
//     }
//     let delimiter = line.split("|").collect::<Vec<&str>>();
//     let left = delimiter[0].parse::<i32>().expect("Unable to parse left");
//     let right = delimiter[1].parse::<i32>().expect("Unable to parse right");
//     Some((left, right))
// }

use std::io::{BufRead, BufReader};
use std::{collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = fs::File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut somme_valid = 0;
    let mut somme_invalid = 0;

    let mut list_of_lines: Vec<(i32, i32)> = Vec::new();
    let mut list_of_values: Vec<Vec<i32>> = Vec::new();

    // Lecture des lignes
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        if let Some((left, right)) = delimiter(&line) {
            list_of_lines.push((left, right));
        } else if line.contains(",") {
            let values: Vec<i32> = line
                .split(',')
                .map(|s| s.trim().parse::<i32>().expect("Unable to parse value"))
                .collect();
            list_of_values.push(values);
        }
    }

    // Créer un dictionnaire : X -> Vec<Y>
    let mut dictionary: HashMap<i32, Vec<i32>> = HashMap::new();
    for (x, y) in list_of_lines {
        dictionary.entry(x).or_default().push(y);
    }

    // Parcours des updates
    for update in &list_of_values {
        // Vérification si l'update est valide
        let mut valid = true;
        let positions: HashMap<i32, usize> =
            update.iter().enumerate().map(|(i, &p)| (p, i)).collect();

        for &x in update {
            if let Some(rules) = dictionary.get(&x) {
                for &y in rules {
                    if positions.contains_key(&y) {
                        if positions[&x] >= positions[&y] {
                            valid = false;
                            break;
                        }
                    }
                }
            }
            if !valid {
                break;
            }
        }

        if valid {
            // update valide : somme des pages du milieu
            let middle_page = update[update.len() / 2];
            println!("Update valide, page du milieu: {}", middle_page);
            somme_valid += middle_page;
        } else {
            // update invalide : reconstruire ordre correct
            let mut remaining: Vec<i32> = update.clone();
            let mut final_order: Vec<i32> = Vec::new();

            while !remaining.is_empty() {
                let mut placed = false;

                // Parcours des pages restantes
                for &x in &remaining {
                    // Vérifie si x peut être placé maintenant
                    let mut blocked = false;

                    for &y in &remaining {
                        if x == y {
                            continue;
                        }
                        if let Some(rules_y) = dictionary.get(&y) {
                            if rules_y.contains(&x) {
                                blocked = true;
                                break;
                            }
                        }
                    }

                    if !blocked {
                        // x est plaçable
                        final_order.push(x);
                        remaining.retain(|&p| p != x);
                        placed = true;
                        break;
                    }
                }

                if !placed {
                    panic!("Impossible de placer une page, règle cyclique détectée !");
                }
            }

            let middle_page = final_order[final_order.len() / 2];
            println!(
                "Update corrigé: {:?}, page du milieu: {}",
                final_order, middle_page
            );
            somme_invalid += middle_page;
        }
    }

    println!("Somme pages du milieu (updates valides)  : {}", somme_valid);
    println!(
        "Somme pages du milieu (updates corrigés) : {}",
        somme_invalid
    );
}

// Fonction pour extraire les règles X|Y
fn delimiter(line: &str) -> Option<(i32, i32)> {
    if let Some(pos) = line.find("|") {
        let left = line[..pos].parse::<i32>().expect("Unable to parse left");
        let right = line[pos + 1..]
            .parse::<i32>()
            .expect("Unable to parse right");
        Some((left, right))
    } else {
        None
    }
}
