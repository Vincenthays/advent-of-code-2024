fn main() {
    let (order, manuals) = include_str!("input.txt").split_once("\n\n").unwrap();
    let order = order
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('|').unwrap();
            let a = a.parse::<u32>().unwrap();
            let b = b.parse::<u32>().unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();

    let res = manuals
        .lines()
        .map(|l| {
            let mut manual = l.split(',')
                .into_iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let manual_copy = manual.clone();

            manual.sort_by(|a, b| {
                for (i, j) in order.iter() {
                    if a == i && b == j {
                        return std::cmp::Ordering::Less;
                    }
                    if a == j && b == i {
                        return std::cmp::Ordering::Greater;
                    }
                }
                std::cmp::Ordering::Equal
            });

            if manual == manual_copy {
                return 0;
            }

            println!("{manual:?}");

            manual[manual.len()/2]
        })
        .sum::<u32>();

    println!("res: {res}");
}
