use std::fs;

pub fn day_1() {
    let contents = fs::read_to_string("input/day_1")
        .expect("Something went wrong reading the file");
    let data =
        contents.split("\n\n")
            .map(|s|
                s.split("\n")
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<i32>().unwrap())
                    .sum::<i32>()
            );

    // Part 1
    let max = data.clone().max().unwrap();
    println!("{}", max);

    // Part 2
    let mut sorted = data.collect::<Vec<i32>>();
    sorted.sort_by(|a, b| b.cmp(a));
    let top_3 = sorted.iter().take(3).sum::<i32>();
    println!("{}", top_3)
}
