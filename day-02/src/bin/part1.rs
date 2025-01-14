fn main() {
    let res: u32 = include_str!("input.txt")
        .lines()
        .into_iter()
        .map(|l| {
            let levels = l
                .split_whitespace()
                .into_iter()
                .map(|l| l.parse().unwrap())
                .collect::<Vec<u32>>();

            let is_level_inc = levels
                .iter()
                .zip(levels.iter().skip(1))
                .all(|(&a, &b)| a < b && a <= b + 3);

            let is_level_dec = levels
                .iter()
                .zip(levels.iter().skip(1))
                .all(|(&a, &b)| a > b && a <= b + 3);

            println!("levels: {:?}", levels);
            println!("is_level_inc: {}", is_level_inc);
            println!("is_level_dec: {}", is_level_dec);

            match is_level_inc || is_level_dec {
                true => 1,
                false => 2,
            }
        })
        .sum();

    println!("{:?}", res);
}
