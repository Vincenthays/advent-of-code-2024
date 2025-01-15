use regex::Regex;

fn main() {
    let data = include_str!("input.txt");
    let regex = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();

    let res = regex
        .captures_iter(data)
        .map(|c| {
            println!("{c:?}");
            let a: u64 = c.name("a").unwrap().as_str().parse().unwrap();
            let b: u64 = c.name("b").unwrap().as_str().parse().unwrap();
            a * b
        })
        .sum::<u64>();

    println!("{}", res);
}
