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
}
