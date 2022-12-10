use crate::*;

pub fn run() {
    // assert_eq!(part1("assets/day9_test.txt"), 13);
    dbg!(part1("assets/day9.txt"));
    assert_eq!(part2("assets/day9_test.txt"), 36);
    dbg!(part2("assets/day9.txt"));
}

fn part2(file: &str) -> usize {
    let path = parse(file);
    let mut rope = [Vec2::zero(); 10];
    let mut positions = HashSet::new();
    positions.insert(rope[9]);
    for step in path {
        rope[0] += step;
        for i in 1..10 {
            move_tail(rope[i - 1], &mut rope[i]);
        }
        positions.insert(rope[9]);
    }
    positions.len()
}

fn part1(file: &str) -> usize {
    let path = parse(file);
    let mut head = Vec2::zero();
    let mut tail = Vec2::zero();
    let mut positions = HashSet::new();
    positions.insert(tail);
    for step in path {
        head += step;
        move_tail(head, &mut tail);
        positions.insert(tail);
    }
    positions.len()
}

fn move_tail(head: Vec2<i32>, tail: &mut Vec2<i32>) {
    let offset = head - *tail;
    if offset.magnitude_squared() > 3 {
        *tail += Vec2::new(offset.x.signum(), offset.y.signum());
    }
}

fn parse(file: &str) -> Vec<Vec2<i32>> {
    let mut path = Vec::new();
    for line in read_file(file).lines() {
        let (dir, dist) = line.split_once(' ').expect("Yeet");
        let vec = match dir {
            "U" => Vec2::unit_y(),
            "D" => -Vec2::unit_y(),
            "R" => Vec2::unit_x(),
            "L" => -Vec2::unit_x(),
            _ => Vec2::zero(),
        };
        let dist = dist.parse::<usize>().expect("Yeet");
        for _ in 0..dist {
            path.push(vec);
        }
    }
    path
}