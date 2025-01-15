fn main() {
    let table = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
}

fn all_direction(table: &Vec<Vec<char>>, i: usize, j: usize) {
    let mut words = [vec![]; 8];

    if let Some(x) = table.get(i-1).and_then(|row| row.get(j-1)) {
        words[0].push(x.clone());
    }
    if let Some(x) = table.get(i-1).and_then(|row| row.get(j)) {
        words[1].push(x.clone());
    }
    if let Some(x) = table.get(i-1).and_then(|row| row.get(j+1)) {
        words[2].push(x.clone());
    }
    if let Some(x) = table.get(i).and_then(|row| row.get(j+1)) {
        words[3].push(x.clone());
    }
    if let Some(x) = table.get(i+1).and_then(|row| row.get(j+1)) {
        words[4].push(x.clone());
    }
    if let Some(x) = table.get(i+1).and_then(|row| row.get(j)) {
        words[5].push(x.clone());
    }
    if let Some(x) = table.get(i+1).and_then(|row| row.get(j-1)) {
        words[6].push(x.clone());
    }
    if let Some(x) = table.get(i).and_then(|row| row.get(j-1)) {
        words[7].push(x.clone());
    }
}
