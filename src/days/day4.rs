use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day4_test.txt"), 2);
    dbg!(part1("assets/day4.txt"));
    assert_eq!(part2("assets/day4_test.txt"), 4);
    dbg!(part2("assets/day4.txt"));
}

fn part2(file: &str) -> u32 {
    read_file(file).lines().filter(|pair| {
        let boundaries = pair.split(|x| matches!(x, ',' | '-')).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let range1 = boundaries[0]..=boundaries[1];
        let range2 = boundaries[2]..=boundaries[3];
        range1.contains(&boundaries[2]) || range1.contains(&boundaries[3])
        || range2.contains(&boundaries[0]) || range2.contains(&boundaries[1])
    }).count() as u32
}

fn part1(file: &str) -> u32 {
    read_file(file).lines().filter(|pair| {
        let boundaries = pair.split(|x| matches!(x, ',' | '-')).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let range1 = boundaries[0]..=boundaries[1];
        let range2 = boundaries[2]..=boundaries[3];
        range1.contains(&boundaries[2]) && range1.contains(&boundaries[3])
        || range2.contains(&boundaries[0]) && range2.contains(&boundaries[1])
    }).count() as u32
}
