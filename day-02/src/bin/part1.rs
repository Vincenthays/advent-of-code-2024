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
                .windows(2)
                .into_iter()
                .all(|l| l[0] < l[1] && l[1] <= l[0] + 3);
            let is_level_dec = levels
                .windows(2)
                .into_iter()
                .all(|l| l[0] > l[1] && l[0] <= l[1] + 3);

            println!("levels: {:?}", levels);
            println!("is_level_inc: {}", is_level_inc);
            println!("is_level_dec: {}", is_level_dec);

            if is_level_inc || is_level_dec {
                1
            } else {
                0
            }
        })
        .sum();

    println!("{:?}", res);
}
