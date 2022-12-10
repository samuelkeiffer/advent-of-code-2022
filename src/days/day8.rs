use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day8_test.txt"), 21);
    dbg!(part1("assets/day8.txt"));
    assert_eq!(part2("assets/day8_test.txt"), 8);
    dbg!(part2("assets/day8.txt"));
}

fn part2(file: &str) -> usize {
    let trees = parse(file);
    let mut high = 0;
    for (i, row) in trees.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            high = high.max(view_score(&trees, (i, j), *tree));
        }
    }
    high
}

fn part1(file: &str) -> usize {
    let trees = parse(file);
    let mut visible = 0;
    for (i, row) in trees.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            if is_visible(&trees, (i, j), *tree) {
                visible += 1;
            }
        }
    }
    visible
}

fn view_score(trees: &Vec<Vec<u32>>, pos: (usize, usize), tree: u32) -> usize {
    let s_n = (0..pos.0).rev().take_while(|i| trees[*i][pos.1] < tree).count();
    let s_s = ((pos.0 + 1)..trees.len()).take_while(|i| trees[*i][pos.1] < tree).count();
    let s_e = (0..pos.1).rev().take_while(|j| trees[pos.0][*j] < tree).count();
    let s_w = ((pos.1 + 1)..trees[0].len()).take_while(|j| trees[pos.0][*j] < tree).count();
    let s_n = s_n + (s_n + 1 < pos.0) as usize;
    let s_s = s_s + (s_s + 1 < trees.len() - pos.0) as usize;
    let s_e = s_e + (s_e + 1 < pos.1) as usize;
    let s_w = s_w + (s_w + 1 < trees[0].len() - pos.1) as usize;
    s_n * s_e * s_s * s_w
}

fn is_visible(trees: &Vec<Vec<u32>>, pos: (usize, usize), tree: u32) -> bool {
    let h_n = (0..pos.0).any(|i| trees[i][pos.1] >= tree);
    let h_s = ((pos.0 + 1)..trees.len()).any(|i| trees[i][pos.1] >= tree);
    let h_e = (0..pos.1).any(|j| trees[pos.0][j] >= tree);
    let h_w = ((pos.1 + 1)..trees.len()).any(|j| trees[pos.0][j] >= tree);
    !(h_n && h_s && h_e && h_w)
}

fn parse(file: &str) -> Vec<Vec<u32>> {
    let mut rows = Vec::new();
    for line in read_file(file).lines() {
        let row = line.chars().filter_map(|x| x.to_digit(10)).collect::<Vec<_>>();
        rows.push(row);
    }
    rows
}