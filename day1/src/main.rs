use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// fn minimum(liste: &[i32]) -> (i32, usize) {
//     let mut mini = liste[0];
//     let mut index_min = 0;
//     for (index, &element) in liste.iter().enumerate() {
//         if element < mini { 
//             mini = element;
//             index_min = index;
//         }
//     }
//     (mini, index_min)
// }

// fn main() -> io::Result<()> {
//     // Vecteurs pour stocker les nombres de chaque colonne
//     let mut first_column: Vec<i32> = Vec::new();
//     let mut second_column: Vec<i32> = Vec::new();
//     let mut res_column: Vec<i32> = Vec::new();

//     // Lire le fichier
//     let path = Path::new("src/input.txt");
//     let file = File::open(path)?;
//     let reader = io::BufReader::new(file);

//     // Parcourir chaque ligne
//     for line in reader.lines() {
//         let line = line?;
//         let parts: Vec<&str> = line.split_whitespace().collect();
        
//         if parts.len() >= 2 {
//             if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
//                 first_column.push(num1);
//                 second_column.push(num2);
//             }
//         }
//     }

//     while !first_column.is_empty() {
//         let (min1, index1) = minimum(&first_column);
//         let (min2, index2) = minimum(&second_column);
//         let res = min1 - min2;
//         if res < 0 {
//             res_column.push(res * -1);
//         } else {
//             res_column.push(res);
//         }
//         first_column.remove(index1);
//         second_column.remove(index2);
//     }

//     let final_res: i32 = res_column.iter().sum();
//     println!("Résultat final: {}", final_res);

//     Ok(())
// }



////// Second exercice //////

fn main() -> io::Result<()> {
    let mut first_column: Vec<i32> = Vec::new();
    let mut second_column: Vec<i32> = Vec::new();
    let mut res_column: Vec<i32> = Vec::new();

    let path = Path::new("src/input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() >= 2 {
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                first_column.push(num1);
                second_column.push(num2);
            }
        }
    }

    for &i in &first_column {
        let mut cpt = 0;
        for &j in &second_column {
            if i == j {
                cpt += 1;
            }
        }
        let res: i32 = (i * cpt) as i32;
        res_column.push(res);
    }
    
    let final_res: i32 = res_column.iter().sum();
    println!("Résultat final: {}", final_res);

    Ok(())
}