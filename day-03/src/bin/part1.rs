use regex::Regex;

fn main() {
    let regex = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    let res = include_str!("input.txt")
        .lines()
        .into_iter()
        .map(|line| regex
            .captures_iter(line)
            .map(|c| {
                println!("{c:?}");
                let a: u64 = c.name("a").unwrap().as_str().parse().unwrap();
                let b: u64 = c.name("b").unwrap().as_str().parse().unwrap();
                a * b
            }).sum::<u64>()
        ).sum::<u64>();

    println!("{}", res);
}
