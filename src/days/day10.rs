use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day10_test.txt"), 13140);
    dbg!(part1("assets/day10.txt"));
    assert_eq!(part2("assets/day10_test.txt"), "##..##..##..##..##..##..##..##..##..##..\n\
        ###...###...###...###...###...###...###.\n\
        ####....####....####....####....####....\n\
        #####.....#####.....#####.....#####.....\n\
        ######......######......######......####\n\
        #######.......#######.......#######.....\n");
    dbg!(part2("assets/day10.txt"));
}

fn part2(file: &str) -> String {
    let instructions = parse(file);
    let mut cycle: usize = 0;
    let mut reg: i32 = 1;
    let mut screen = [[false; 40]; 6];
    for instruction in instructions {
        let row = (cycle % 240) / 40;
        let col = cycle % 40;
        if (col as i32 - reg).abs() <= 1 {
            screen[row][col] = true;
        }
        match instruction {
            Instruction::NoOp => {},
            Instruction::Add(x) => reg += x,
        }
        cycle += 1;
    }
    let mut disp = String::new();
    for row in screen {
        for pixel in row {
            if pixel {
                disp.push_str("#");
            } else {
                disp.push_str(".");
            }
        }
        disp.push_str("\n");
    }
    disp
}

fn part1(file: &str) -> i32 {
    let instructions = parse(file);
    let mut cycle = 0;
    let mut reg = 1;
    let mut signal_sum = 0;
    for instruction in instructions {
        cycle += 1;
        if (cycle - 20) % 40 == 0 {
            signal_sum += reg * cycle;
        }
        match instruction {
            Instruction::NoOp => {},
            Instruction::Add(x) => reg += x,
        }
    }
    signal_sum
}

fn parse(file: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in read_file(file).lines() {
        match line {
            "noop" => {
                instructions.push(Instruction::NoOp);
            },
            add => {
                let (_, num) = add.split_once(' ').expect("Yeet");
                let num = num.parse::<i32>().expect("Yeet");
                instructions.push(Instruction::NoOp);
                instructions.push(Instruction::Add(num));
            }
        }
    }
    instructions
}

enum Instruction {
    NoOp,
    Add(i32),
}
