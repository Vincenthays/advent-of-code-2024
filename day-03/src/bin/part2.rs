use regex::Regex;

fn main() {
    let regex = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)|do\(\)|don't\(\)").unwrap();
    let data = include_str!("input.txt");

    let mut sum = 0;
    let mut active = true;

    for c in regex.captures_iter(data) {
        match c.get(0).unwrap().as_str() {
            "do()" => active = true,
            "don't()" => active = false,
            _ => {
                if active {
                    let a: u64 = c.name("a").unwrap().as_str().parse().unwrap();
                    let b: u64 = c.name("b").unwrap().as_str().parse().unwrap();
                    sum += a * b;
                }
            }
        }
    }

    println!("{}", sum);
}
