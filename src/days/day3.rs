use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day3_test.txt"), 157);
    dbg!(part1("assets/day3.txt"));
    assert_eq!(part2("assets/day3_test.txt"), 70);
    dbg!(part2("assets/day3.txt"));
}

fn part2(file: &str) -> u32 {
    let mut groups = Vec::new();
    let mut group = [None; 3];
    let file = read_file(file);
    for (i, rucksack) in file.lines().enumerate() {
        group[i % 3] = Some(rucksack);
        if i % 3 == 2 {
            groups.push(std::mem::replace(&mut group, [None; 3]));
        }
    }
    let group_ids = groups.iter().map(|group| {
        group[0].unwrap().chars().filter(|c1| group[1].unwrap().chars().any(|c2| *c1 == c2)).find(|c1| group[2].unwrap().chars().any(|c3| *c1 == c3)).unwrap()
    }).collect::<Vec<_>>();
    group_ids.iter().map(|x| priority(*x)).sum()
}

fn part1(file: &str) -> u32 {
    let mut common_items = Vec::new();
    for rucksack in read_file(file).lines() {
        let mid = rucksack.chars().count() / 2;
        common_items.push(rucksack.chars().take(mid).find(|c1| rucksack.chars().skip(mid).any(|c2| *c1 == c2)).expect("Yeet"));
    }
    common_items.iter().map(|x| priority(*x)).sum()
}

fn priority(x: char) -> u32 {
    let x = x as u32;
    if x > 94 {
        x - 96
    } else {
        x - 38
    }
}
