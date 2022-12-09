use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day2_test.txt"), 15);
    dbg!(part1("assets/day2.txt"));
    assert_eq!(part2("assets/day2_test.txt"), 12);
    dbg!(part2("assets/day2.txt"));
}

fn part2(file: &str) -> u32 {
    let mut rounds = Vec::new();
    for round in read_file(file).lines() {
        let mut chars = round.chars();
        let left = Shape::from_char(chars.next().unwrap());
        chars.next();
        let right = Shape::from_result(&left, chars.next().unwrap());
        rounds.push((left, right));
    }
    rounds.iter().map(|(left, right)| right.outcome(left) + right.score()).sum()
}

fn part1(file: &str) -> u32 {
    let mut rounds = Vec::new();
    for round in read_file(file).lines() {
        let mut chars = round.chars();
        let left = Shape::from_char(chars.next().unwrap());
        chars.next();
        let right = Shape::from_char(chars.next().unwrap());
        rounds.push((left, right));
    }
    rounds.iter().map(|(left, right)| right.outcome(left) + right.score()).sum()
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_char(x: char) -> Shape {
        use Shape::*;
        match x {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!("Yeet"),
        }
    }

    fn from_result(&self, x: char) -> Shape {
        use Shape::*;
        match (self, x) {
            (Paper, 'X') | (Rock, 'Y') | (Scissors, 'Z') => Rock,
            (Scissors, 'X') | (Paper, 'Y') | (Rock, 'Z') => Paper,
            (Rock, 'X') | (Scissors, 'Y') | (Paper, 'Z') => Scissors,
            _ => panic!("Yeet"),
        }
    }

    fn score(&self) -> u32 {
        use Shape::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn outcome(&self, other: &Self) -> u32 {
        use Shape::*;
        match (self, other) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 0,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 6,
        }
    }
}
