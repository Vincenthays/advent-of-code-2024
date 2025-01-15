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
            let manual = l.split(',')
                .into_iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            for &(a, b) in order.iter() {
                let Some(i) = manual.iter().position(|&x| x == a) else {
                    continue
                };
                let Some(j) = manual.iter().position(|&x| x == b) else {
                    continue
                };
                if i > j {
                    return 0;
                }
            }

            println!("{:?} - {:?}", &manual, manual[manual.len()/2]);

            manual[manual.len()/2]
        })
        .sum::<u32>();

    println!("res: {res}");
}
