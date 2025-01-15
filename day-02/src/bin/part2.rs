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

            let level_inc = levels
                .iter()
                .zip(levels.iter().skip(1))
                .filter_map(|(&a, &b)| match a < b && b <= a + 3 {
                    true => Some(a),
                    false => None,
                })
                .collect::<Vec<_>>();

            let level_dec= levels
                .iter()
                .zip(levels.iter().skip(1))
                .filter_map(|(&a, &b)| match b < a && a <= b + 3 {
                    true => Some(a),
                    false => None,
                })
                .collect::<Vec<_>>();

            println!("levels: {:?}", levels);
            println!("is_level_inc: {:?}", level_inc);
            println!("is_level_inc: {:?}", level_dec);

            0
        })
        .sum();

    println!("{:?}", res);
}
