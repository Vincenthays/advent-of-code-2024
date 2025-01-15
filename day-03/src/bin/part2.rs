use regex::Regex;

fn main() {
    let data = include_str!("input.txt");
    let regex = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let mut is_active = true;

    for c in regex.captures_iter(data) {
        match (c.get(0).unwrap().as_str(), is_active) {
            ("do()", _) => is_active = true,
            ("don't()", _) => is_active = false,
            (_, true) => {
                let a: u64 = c.name("a").unwrap().as_str().parse().unwrap();
                let b: u64 = c.name("b").unwrap().as_str().parse().unwrap();
                sum += a * b;
            }
            _ => {}
        }
    }

    println!("{}", sum);
}
