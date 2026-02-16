use std::{collections::HashMap, collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("data.txt").unwrap();

    let mut list: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    let carte: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    for row in &carte {
        let line: String = row.iter().collect();
        println!("{}", line);
    }

    for (x, row) in carte.iter().enumerate() {
        for (y, &char) in row.iter().enumerate() {
            if char != '.' {
                list.entry(char).or_default().push((x as isize, y as isize));
            }
        }
    }
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for (_char, nodes) in &list {
        for node in nodes {
            antinodes.insert(*node);
        }
        for x in 0..nodes.len() {
            for y in x + 1..nodes.len() {
                let (x1, y1) = nodes[x];
                let (x2, y2) = nodes[y];

                let dx = x2 - x1; // (2,1) - (1,3) = (1,-2)
                // (1,3) - (2,1) = (-1,2)
                let dy = y2 - y1;

                let mut p1 = (x1 - dx, y1 - dy);
                let mut p2 = (x2 + dx, y2 + dy);

                while let Some(_) = carte
                    .get(p1.0 as usize)
                    .and_then(|row| row.get(p1.1 as usize))
                {
                    antinodes.insert(p1);
                    p1 = (p1.0 - dx, p1.1 - dy);
                }

                while let Some(_) = carte
                    .get(p2.0 as usize)
                    .and_then(|row| row.get(p2.1 as usize))
                {
                    antinodes.insert(p2);
                    p2 = (p2.0 + dx, p2.1 + dy);
                }
            }
        }
    }
    print!("{}", antinodes.len())
}
