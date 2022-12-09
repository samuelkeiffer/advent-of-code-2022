use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day1_test.txt"), 24000);
    dbg!(part1("assets/day1.txt"));
    assert_eq!(part2("assets/day1_test.txt"), 45000);
    dbg!(part2("assets/day1.txt"));
}

fn part2(file: &str) -> u32 {
    let elves = parse_file(file);
    elves.iter().map(|elf| elf.iter().sum::<u32>()).sorted().into_iter().rev().take(3).sum()
}

fn part1(file: &str) -> u32 {
    let elves = parse_file(file);
    elves.iter().map(|elf| elf.iter().sum()).max().unwrap_or(0)
}

fn parse_file(file: &str) -> Vec<Vec<u32>> {
    let mut elves = Vec::new();
    let mut elf = Vec::new();
    for food in read_file(file).lines() {
        if food == "" {
            elves.push(std::mem::replace(&mut elf, Vec::new()));
        } else {
            elf.push(food.parse::<u32>().unwrap_or(0));
        }
    }
    elves.push(elf);
    elves
}
