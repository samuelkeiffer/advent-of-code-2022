use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day6_test.txt"), 7);
    dbg!(part1("assets/day6.txt"));
    assert_eq!(part2("assets/day6_test.txt"), 19);
    dbg!(part2("assets/day6.txt"));
}

fn part2(file: &str) -> usize {
    let buffer = read_file(file).chars().collect::<Vec<_>>();
    for i in 14..buffer.len() {
        let mut check = HashSet::new();
        for j in (i - 14)..i {
            check.insert(buffer[j]);
        }
        if check.len() == 14 {
            return i;
        }
    }
    0
}

fn part1(file: &str) -> usize {
    let buffer = read_file(file).chars().collect::<Vec<_>>();
    for i in 4..buffer.len() {
        let mut check = HashSet::new();
        for j in (i - 4)..i {
            check.insert(buffer[j]);
        }
        if check.len() == 4 {
            return i;
        }
    }
    0
}
