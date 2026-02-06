use std::io::{BufRead, BufReader};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = fs::File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut matrice: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line"); // Déballer le Result
        let values: Vec<char> = line.chars().collect(); // Utiliser chars() au lieu de split("")

        matrice.push(values);
    }

    for row in &matrice {
        let line: String = row.iter().collect();
        println!("{}", line);
    }

    let mut visited = vec![vec![false; matrice[0].len()]; matrice.len()];

    let directions: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    //if let Some(row_above) = matrice.get(i.wrapping_sub(1)) {}
    let mut user = (0, 0);
    let mut direction: usize = 0;

    for (i, row) in matrice.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                user = (i as isize, j as isize);
                direction = 0; // '^' = haut
            }
        }
    }

    while let Some(_) = get2d(&matrice, user.0 + directions[0].0, user.1 + directions[0].1) {
        visited[user.0 as usize][user.1 as usize] = true;

        let (di, dj) = directions[direction];
        let ni = user.0 + di;
        let nj = user.1 + dj;

        if let Some(&'#') = get2d(&matrice, ni, nj) {
            direction = (direction + 1) % 4;
        } else {
            user = (ni, nj);
        }
    }
    let mut cpt = 0;
    for row in &visited {
        for cell in row {
            if *cell {
                cpt += 1;
            }
        }
    }
    println!("{}", cpt);
}

fn get2d<T>(matrix: &Vec<Vec<T>>, row: isize, col: isize) -> Option<&T> {
    matrix
        .get(row as usize)
        .and_then(|row_vec| row_vec.get(col as usize))
}
