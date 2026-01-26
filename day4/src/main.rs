// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let file_path = &args[1];

//     let content = fs::read_to_string(file_path).expect("Unable to read file");

//     let mut matrix: Vec<Vec<char>> = Vec::new();
//     for line in content.lines() {
//         let ligne: Vec<char> = line.chars().collect();
//         matrix.push(ligne);
//     }

//     let mut cpt = 0;
//     let rows = matrix.len();
//     let cols = matrix[0].len();

//     for i in 0..rows {
//         for j in 0..cols {
//             // Vérifier si on peut former "XMAS" à partir de cette position
//             if matrix[i][j] == 'X' {
//                 // Direction diagonale haut-gauche (i-3, j-3)
//                 if i >= 3 && j >= 3 {
//                     if matrix[i - 1][j - 1] == 'M'
//                         && matrix[i - 2][j - 2] == 'A'
//                         && matrix[i - 3][j - 3] == 'S'
//                     {
//                         cpt += 1;
//                     }
//                 }

//                 // Direction verticale haut (i-3, j)
//                 if i >= 3 {
//                     if matrix[i - 1][j] == 'M' && matrix[i - 2][j] == 'A' && matrix[i - 3][j] == 'S'
//                     {
//                         cpt += 1;
//                     }
//                 }

//                 // Direction diagonale haut-droite (i-3, j+3)
//                 if i >= 3 && j + 3 < cols {
//                     if matrix[i - 1][j + 1] == 'M'
//                         && matrix[i - 2][j + 2] == 'A'
//                         && matrix[i - 3][j + 3] == 'S'
//                     {
//                         cpt += 1;
//                     }
//                 }

//                 // Direction horizontale gauche (i, j-3)
//                 if j >= 3 {
//                     if matrix[i][j - 1] == 'M' && matrix[i][j - 2] == 'A' && matrix[i][j - 3] == 'S'
//                     {
//                         cpt += 1;
//                     }
//                 }

//                 // Direction horizontale droite (i, j+3)
//                 if j + 3 < cols {
//                     if matrix[i][j + 1] == 'M' && matrix[i][j + 2] == 'A' && matrix[i][j + 3] == 'S'
//                     {
//                         cpt += 1;
//                     }
//                 }

//                 // Direction diagonale bas-gauche (i+3, j-3)
//                 if i + 3 < rows && j >= 3 {
//                     if matrix[i + 1][j - 1] == 'M'
//                         && matrix[i + 2][j - 2] == 'A'
//                         && matrix[i + 3][j - 3] == 'S'
//                     {
//                         cpt += 1;
//                     }
//                 }

//                 // Direction verticale bas (i+3, j)
//                 if i + 3 < rows {
//                     if matrix[i + 1][j] == 'M' && matrix[i + 2][j] == 'A' && matrix[i + 3][j] == 'S'
//                     {
//                         cpt += 1;
//                     }
//                 }

//                 // Direction diagonale bas-droite (i+3, j+3)
//                 if i + 3 < rows && j + 3 < cols {
//                     if matrix[i + 1][j + 1] == 'M'
//                         && matrix[i + 2][j + 2] == 'A'
//                         && matrix[i + 3][j + 3] == 'S'
//                     {
//                         cpt += 1;
//                     }
//                 }
//             }
//         }
//     }

//     println!("Nombre total d'occurrences de XMAS : {}", cpt);
// }

// 2eme solution plus optimisée

// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let file_path = &args[1];

//     let content = fs::read_to_string(file_path).expect("Unable to read file");

//     let matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
//     let rows = matrix.len();
//     let cols = matrix[0].len();

//     // Huit directions : (delta_row, delta_col)
//     let directions = [
//         (-1, -1), // haut-gauche
//         (-1, 0),  // haut
//         (-1, 1),  // haut-droite
//         (0, -1),  // gauche
//         (0, 1),   // droite
//         (1, -1),  // bas-gauche
//         (1, 0),   // bas
//         (1, 1),   // bas-droite
//     ];

//     let target: Vec<char> = "XMAS".chars().collect();
//     let mut cpt = 0;

//     for i in 0..rows {
//         for j in 0..cols {
//             // On ne cherche que depuis une case contenant 'X'
//             if matrix[i][j] == 'X' {
//                 for &(dr, dc) in &directions {
//                     // Vérifier si tous les caractères de "XMAS" correspondent dans cette direction
//                     let matches = (0..target.len()).all(|k| {
//                         let r = i as isize + dr * k as isize;
//                         let c = j as isize + dc * k as isize;
//                         r >= 0
//                             && c >= 0
//                             && (r as usize) < rows
//                             && (c as usize) < cols
//                             && matrix[r as usize][c as usize] == target[k]
//                     });
//                     if matches {
//                         cpt += 1;
//                     }
//                 }
//             }
//         }
//     }

//     println!("Nombre total d'occurrences de XMAS : {}", cpt);
// }

// niveau 2 PAS RESOLU

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path).expect("Unable to read file");

    let matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut count = 0;

    // Parcourir chaque position dans la matrice
    for i in 0..rows {
        for j in 0..cols {
            // Chercher des X-MAS à partir de chaque position
            count += count_xmas_at_position(&matrix, i, j, rows, cols);
        }
    }

    println!("Nombre total d'occurrences de X-MAS : {}", count);
}

fn count_xmas_at_position(
    matrix: &Vec<Vec<char>>,
    center_i: usize,
    center_j: usize,
    rows: usize,
    cols: usize,
) -> i32 {
    let mut count = 0;

    // Vérifier si la position centrale peut être le centre d'un X-MAS
    // Le centre doit être 'A' selon le pattern
    if matrix[center_i][center_j] != 'A' {
        return 0;
    }

    // Directions pour les deux branches du X
    // Chaque branche doit former "MAS" (dans n'importe quelle direction)
    let directions = [
        // Branche 1: diagonale haut-gauche, Branche 2: diagonale bas-droite
        ((-1, -1), (1, 1)),
        // Branche 1: diagonale haut-droite, Branche 2: diagonale bas-gauche
        ((-1, 1), (1, -1)),
        // Branche 1: haut, Branche 2: bas
        ((-1, 0), (1, 0)),
        // Branche 1: gauche, Branche 2: droite
        ((0, -1), (0, 1)),
    ];

    for &(dir1, dir2) in &directions {
        // Vérifier si on peut former un X-MAS avec ces directions
        if can_form_xmas(matrix, center_i, center_j, dir1, dir2, rows, cols) {
            count += 1;
        }
    }

    count
}

fn can_form_xmas(
    matrix: &Vec<Vec<char>>,
    center_i: usize,
    center_j: usize,
    dir1: (i32, i32),
    dir2: (i32, i32),
    rows: usize,
    cols: usize,
) -> bool {
    // Vérifier que les positions pour les deux branches sont valides
    // Les M et S sont à distance 1 du centre A
    let pos1 = (center_i as i32 + dir1.0, center_j as i32 + dir1.1);
    let pos2 = (center_i as i32 + dir2.0, center_j as i32 + dir2.1);

    // Vérifier que toutes les positions sont dans les limites
    if !is_valid_position(pos1, rows, cols) || !is_valid_position(pos2, rows, cols) {
        return false;
    }

    // Convertir en usize pour accéder à la matrice
    let (i1, j1) = (pos1.0 as usize, pos1.1 as usize);
    let (i2, j2) = (pos2.0 as usize, pos2.1 as usize);

    // Vérifier que les deux branches forment "MAS" (dans n'importe quelle direction)
    // Chaque branche peut être "MAS" ou "SAM"
    let branch1_mas = matrix[i1][j1] == 'M' && matrix[i2][j2] == 'S';
    let branch1_sam = matrix[i1][j1] == 'S' && matrix[i2][j2] == 'M';

    // Au moins une branche doit être valide
    branch1_mas || branch1_sam
}

fn is_valid_position(pos: (i32, i32), rows: usize, cols: usize) -> bool {
    pos.0 >= 0 && pos.0 < rows as i32 && pos.1 >= 0 && pos.1 < cols as i32
}
