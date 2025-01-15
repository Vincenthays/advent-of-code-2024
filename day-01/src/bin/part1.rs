fn main() {
    let (mut col1, mut col2): (Vec<_>, Vec<_>) = include_str!("input.txt")
        .lines()
        .into_iter()
        .map(|l| {
            let mut s = l.split_whitespace();
            let col1: u32 = s.next().unwrap().parse().unwrap();
            let col2: u32 = s.next().unwrap().parse().unwrap();
            (col1, col2)
        })
        .unzip();

    col1.sort();
    col2.sort();

    let res: u32 = col1
        .into_iter()
        .zip(col2.into_iter())
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .sum();

    println!("{:?}", res);
}
