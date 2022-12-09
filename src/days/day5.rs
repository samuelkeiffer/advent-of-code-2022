use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day5_test.txt"), "CMZ");
    dbg!(part1("assets/day5.txt"));
    assert_eq!(part2("assets/day5_test.txt"), "MCD");
    dbg!(part2("assets/day5.txt"));
}

fn part2(file: &str) -> String {
    let (mut crates, procedures) = parse(file);
    for Procedure { amount, to, from } in procedures {
        let mut buffer = VecDeque::new();
        for _ in 0..amount {
            if let Some(crate_) = crates[from].pop() {
                buffer.push_front(crate_);
            }
        }
        for crate_ in buffer {
            crates[to].push(crate_);
        }
    }
    crates.into_iter().filter_map(|mut stack| stack.pop()).collect()
}

fn part1(file: &str) -> String {
    let (mut crates, procedures) = parse(file);
    for Procedure { amount, to, from } in procedures {
        for _ in 0..amount {
            if let Some(crate_) = crates[from].pop() {
                crates[to].push(crate_);
            }
        }
    }
    crates.into_iter().filter_map(|mut stack| stack.pop()).collect()
}

fn parse(file: &str) -> (Vec<Vec<char>>, VecDeque<Procedure>) {
    let mut crate_lines = Vec::new();
    let mut bottom_line = String::new();
    let mut procedure_lines = Vec::new();
    let file = read_file(file);
    for line in file.lines() {
        if line.contains('[') {
            crate_lines.push(line);
        } else if line.starts_with("move") {
            procedure_lines.push(line);
        } else if line.chars().count() > 1 {
            bottom_line = String::from(line);
        }
    }

    let mut crates = Vec::new();
    for line in crate_lines.iter().rev() {
        for (i, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                let pillar = bottom_line.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
                while crates.get(pillar).is_none() {
                    crates.push(Vec::new());
                }
                crates[pillar].push(c);
            }
        }
    }

    let mut procedures = VecDeque::new();
    for line in procedure_lines {
        let splits = line.split(' ').collect::<Vec<_>>();
        let numbers = splits.iter().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<_>>();
        procedures.push_back(Procedure { amount: numbers[0], from: numbers[1], to: numbers[2] });
    }

    (crates, procedures)
}

struct Procedure {
    amount: usize,
    from: usize,
    to: usize,
}