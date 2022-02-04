use std::fs;

static INPUT_FILE: &str = "./inputs/day9/input.txt";

fn read_input(path: &str) -> Vec<Vec<isize>> {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    contents
        .split('\n')
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect()
        })
        .filter(|v: &Vec<isize>| !v.is_empty())
        .collect()
}

fn is_local_minima(grid: &Vec<Vec<isize>>, i: isize, j: isize) -> bool {
    let h = grid.len() as isize;
    let w = grid[0].len() as isize;
    let mut is_local_minima = true;

    let neighbours = vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)];
    let neighbours: Vec<(isize, isize)> = neighbours
        .into_iter()
        .filter(|(x, y)| 0 <= *x && *x < h && 0 <= *y && *y < w)
        .collect();

    for n in neighbours {
        is_local_minima =
            is_local_minima && grid[i as usize][j as usize] < grid[n.0 as usize][n.1 as usize];
    }

    is_local_minima
}

fn part1(inputs: &Vec<Vec<isize>>) {
    let mut count = 0;
    for i in 0..inputs.len() {
        for j in 0..inputs[0].len() {
            if is_local_minima(inputs, i as isize, j as isize) {
                count += 1;
            }
        }
    }

    println!("Number of local minima: {}", count);
}

pub fn solve() {
    let input = read_input(INPUT_FILE);
    part1(&input);
}
