use std::fs;

static INPUT_FILE: &str = "./inputs/day3/input.txt";

fn result(bit_vec: &Vec<u32>) -> u32 {
    bit_vec.iter().fold(0, |acc, &b| acc << 1 | b)
}

fn most_frequent_bit(index: usize, readings: &Vec<Vec<u32>>) -> u32 {
    let count = readings.len();
    let sum: u32 = readings.iter().map(|v| v[index]).sum();
    if sum >= ((count + 1) as u32 / 2) {
        return 1;
    }

    0
}

fn least_frequent_bit(index: usize, readings: &Vec<Vec<u32>>) -> u32 {
    if most_frequent_bit(index, readings) == 1 {
        return 0;
    }

    1
}

fn gamma(readings: &Vec<Vec<u32>>) -> u32 {
    let bit_vec = (0..readings[0].len())
        .map(|i| most_frequent_bit(i, readings))
        .collect();
    result(&bit_vec)
}
fn epsilon(readings: &Vec<Vec<u32>>) -> u32 {
    let bit_vec = (0..readings[0].len())
        .map(|i| least_frequent_bit(i, readings))
        .collect();
    result(&bit_vec)
}

fn all_bit_equal_to(
    criteria: fn(usize, &Vec<Vec<u32>>) -> u32,
    index: usize,
    readings: &Vec<Vec<u32>>,
) -> Vec<u32> {
    if readings.len() == 1 {
        return readings[0].clone();
    }

    let bit_val = criteria(index, readings);
    let readings = readings
        .clone()
        .into_iter()
        .filter(|v| v[index] == bit_val)
        .collect();

    all_bit_equal_to(criteria, index + 1, &readings)
}

fn oxygen_generator(readings: &Vec<Vec<u32>>) -> u32 {
    result(&all_bit_equal_to(most_frequent_bit, 0, readings))
}

fn co2_scrubber(readings: &Vec<Vec<u32>>) -> u32 {
    result(&all_bit_equal_to(least_frequent_bit, 0, readings))
}

fn ratings(readings: &Vec<Vec<u32>>) -> (u32, u32, u32, u32) {
    (
        gamma(readings),
        epsilon(readings),
        oxygen_generator(readings),
        co2_scrubber(readings),
    )
}

fn read_input(path: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(path).expect("Unable to read file");

    contents
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(2).unwrap()).collect())
        .collect()
}

pub fn solve() {
    let inputs = read_input(INPUT_FILE);
    let (gamma, epsilon, oxygen_generator, co2_scrubber) = ratings(&inputs);
    println!(
        "Part 1: Gamma: {}, Epsilon: {}, Power Factor: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    println!(
        "Part 2: Oxygen Generator: {}, CO2 Scrubber: {}, Life Support Rating: {}",
        oxygen_generator,
        co2_scrubber,
        oxygen_generator * co2_scrubber
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_FILE: &str = "./inputs/day3/test.txt";

    #[test]
    fn can_read_input() {
        assert_eq!(
            read_input(TEST_FILE),
            vec![
                vec![0, 0, 1, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 1, 1, 0],
                vec![1, 0, 1, 1, 1],
                vec![1, 0, 1, 0, 1],
                vec![0, 1, 1, 1, 1],
                vec![0, 0, 1, 1, 1],
                vec![1, 1, 1, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 1],
                vec![0, 0, 0, 1, 0],
                vec![0, 1, 0, 1, 0],
            ]
        );
    }

    #[test]
    fn can_calculate_the_ratings() {
        assert_eq!(ratings(&read_input(TEST_FILE)), (22, 9, 23, 10));
    }
}
