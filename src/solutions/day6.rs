use nom::{bytes::complete::tag, combinator::map, multi::separated_list0};
use std::fs;

use super::utils::parse_usize;
static INPUT_FILE: &str = "./inputs/day6/input.txt";

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct LanternFish {
    lifetime: usize,
}

fn read_input(path: &str) -> Vec<LanternFish> {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    let fish_parser = map(parse_usize, |lifetime| LanternFish { lifetime });
    let (_, result) = separated_list0(tag(","), fish_parser)(&contents).unwrap();
    result
}

fn simulate(days: usize, fishes: &Vec<LanternFish>) {
    let length = 9;
    let mut fish_lifetime_count: Vec<usize> = vec![0; length];
    for fish in fishes {
        fish_lifetime_count[fish.lifetime] += 1;
    }
    println!("{:?}", fish_lifetime_count);

    for day in 0..days {
        let fish_at_zero = fish_lifetime_count[0];
        fish_lifetime_count.rotate_left(1);
        fish_lifetime_count[6] += fish_at_zero;
        fish_lifetime_count[8] = fish_at_zero;

        println!(
            "Fish population after day {}: {}",
            day,
            fish_lifetime_count.iter().sum::<usize>(),
        )
    }

    println!("{:?}", fish_lifetime_count);
}

pub fn solve() {
    let days = 256;
    let fishes = read_input(INPUT_FILE);
    simulate(days, &fishes);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_FILE: &str = "./inputs/day6/test.txt";

    #[test]
    fn can_read_input() {
        assert_eq!(
            read_input(TEST_FILE),
            vec![
                LanternFish { lifetime: 3 },
                LanternFish { lifetime: 4 },
                LanternFish { lifetime: 3 },
                LanternFish { lifetime: 1 },
                LanternFish { lifetime: 2 },
            ]
        )
    }
}
