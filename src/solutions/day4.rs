use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{digit1, newline, space0, space1},
        streaming::char,
    },
    combinator::{map, map_res, recognize},
    multi::{count, separated_list0},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

static INPUT_FILE: &str = "./inputs/day4/input.txt";

type Draws = Vec<u32>;

#[derive(Debug, PartialEq, Eq)]
struct Elem {
    value: u32,
    is_marked: bool,
}

impl Elem {
    fn new(value: u32) -> Self {
        Self {
            value,
            is_marked: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Board {
    elems: Vec<Vec<Elem>>,
    has_won: bool,
}

impl Board {
    fn new(elems: Vec<Vec<Elem>>) -> Self {
        Self {
            elems,
            has_won: false,
        }
    }

    fn mark(&mut self, value: u32) {
        for i in 0..self.elems.len() {
            for j in 0..self.elems[0].len() {
                if self.elems[i][j].value == value {
                    self.elems[i][j].is_marked = true
                }
            }
        }
    }

    fn is_bingo(&mut self) -> bool {
        let mut row_cond = true;
        let mut col_cond = true;
        for i in 0..5 {
            for j in 0..5 {
                row_cond = row_cond && self.elems[i][j].is_marked;
                col_cond = col_cond && self.elems[j][i].is_marked;
            }

            if row_cond || col_cond {
                self.has_won = true;
                return row_cond || col_cond;
            }

            row_cond = true;
            col_cond = true;
        }

        false
    }

    fn score(&self) -> u32 {
        let mut sum = 0;
        for row in &self.elems {
            for elem in row {
                if !elem.is_marked {
                    sum += elem.value;
                }
            }
        }

        sum
    }
}

fn bingo_first(draws: &Draws, boards: &mut Vec<Board>) -> Option<(u32, u32)> {
    for draw in draws {
        for i in 0..boards.len() {
            boards[i].mark(*draw);
            if boards[i].is_bingo() {
                println!("The draw is: {}", draw);
                return Some((*draw, boards[i].score()));
            }
        }
    }

    None
}

fn bingo_last(draws: &Draws, boards: &mut Vec<Board>) -> Option<(u32, u32)> {
    let mut last_winner: Option<(u32, u32)> = None;
    for draw in draws {
        for i in 0..boards.len() {
            if boards[i].has_won {
                continue;
            }
            boards[i].mark(*draw);
            if boards[i].is_bingo() {
                last_winner = Some((*draw, boards[i].score()));
                println!("{:?}", last_winner.unwrap())
            }
        }
    }

    last_winner
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_draws(input: &str) -> IResult<&str, Draws> {
    terminated(separated_list0(char(','), parse_u32), newline)(input)
}

fn parse_board(input: &str) -> IResult<&str, Board> {
    let elem = map(
        terminated(preceded(space0, parse_u32), alt((space1, tag("\n")))),
        Elem::new,
    );
    let row = count(elem, 5);

    map(count(row, 5), Board::new)(input)
}

fn read_input(path: &str) -> (Draws, Vec<Board>) {
    let contents = fs::read_to_string(path).expect("Unable to read file");

    let (_, results) =
        separated_pair(parse_draws, newline, separated_list0(newline, parse_board))(&contents)
            .unwrap();

    results
}

pub fn solve() {
    let (draws, mut boards) = read_input(INPUT_FILE);
    let (first_draw, first_winner) = bingo_first(&draws, &mut boards).unwrap();
    let (last_draw, last_winner) = bingo_last(&draws, &mut boards).unwrap();

    println!("Result of part 1: {}", first_draw * first_winner);
    println!("Result of part 2: {}", last_draw * last_winner);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_FILE: &str = "./inputs/day4/test.txt";

    #[test]
    fn can_parse_draws() {
        let (_, result) = parse_draws("7,4,9,5,11,17\n").unwrap();
        assert_eq!(result, [7, 4, 9, 5, 11, 17]);
    }

    #[test]
    fn can_parse_a_board() {
        let input =
            "22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n";
        let (_, result) = parse_board(input).unwrap();
        let expected_result = Board::new(vec![
            vec![
                Elem::new(22),
                Elem::new(13),
                Elem::new(17),
                Elem::new(11),
                Elem::new(0),
            ],
            vec![
                Elem::new(8),
                Elem::new(2),
                Elem::new(23),
                Elem::new(4),
                Elem::new(24),
            ],
            vec![
                Elem::new(21),
                Elem::new(9),
                Elem::new(14),
                Elem::new(16),
                Elem::new(7),
            ],
            vec![
                Elem::new(6),
                Elem::new(10),
                Elem::new(3),
                Elem::new(18),
                Elem::new(5),
            ],
            vec![
                Elem::new(1),
                Elem::new(12),
                Elem::new(20),
                Elem::new(15),
                Elem::new(19),
            ],
        ]);
        assert_eq!(result, expected_result)
    }

    #[test]
    fn can_read_input() {
        let (draws, boards) = read_input(TEST_FILE);

        assert_eq!(draws.len(), 27);
        assert_eq!(boards.len(), 3);
        assert_eq!(draws[0..5], [7, 4, 9, 5, 11]);
        assert_eq!(
            boards[0],
            Board::new(vec![
                vec![
                    Elem::new(22),
                    Elem::new(13),
                    Elem::new(17),
                    Elem::new(11),
                    Elem::new(0)
                ],
                vec![
                    Elem::new(8),
                    Elem::new(2),
                    Elem::new(23),
                    Elem::new(4),
                    Elem::new(24)
                ],
                vec![
                    Elem::new(21),
                    Elem::new(9),
                    Elem::new(14),
                    Elem::new(16),
                    Elem::new(7)
                ],
                vec![
                    Elem::new(6),
                    Elem::new(10),
                    Elem::new(3),
                    Elem::new(18),
                    Elem::new(5)
                ],
                vec![
                    Elem::new(1),
                    Elem::new(12),
                    Elem::new(20),
                    Elem::new(15),
                    Elem::new(19)
                ]
            ])
        );
        let a = vec![
            vec![
                Elem {
                    value: 2,
                    is_marked: true,
                },
                Elem {
                    value: 16,
                    is_marked: false,
                },
                Elem {
                    value: 50,
                    is_marked: false,
                },
                Elem {
                    value: 26,
                    is_marked: true,
                },
                Elem {
                    value: 84,
                    is_marked: true,
                },
            ],
            vec![
                Elem {
                    value: 97,
                    is_marked: true,
                },
                Elem {
                    value: 24,
                    is_marked: false,
                },
                Elem {
                    value: 32,
                    is_marked: true,
                },
                Elem {
                    value: 51,
                    is_marked: true,
                },
                Elem {
                    value: 8,
                    is_marked: false,
                },
            ],
            vec![
                Elem {
                    value: 70,
                    is_marked: true,
                },
                Elem {
                    value: 0,
                    is_marked: false,
                },
                Elem {
                    value: 3,
                    is_marked: true,
                },
                Elem {
                    value: 52,
                    is_marked: true,
                },
                Elem {
                    value: 9,
                    is_marked: false,
                },
            ],
            vec![
                Elem {
                    value: 1,
                    is_marked: false,
                },
                Elem {
                    value: 59,
                    is_marked: false,
                },
                Elem {
                    value: 43,
                    is_marked: false,
                },
                Elem {
                    value: 64,
                    is_marked: true,
                },
                Elem {
                    value: 80,
                    is_marked: false,
                },
            ],
            vec![
                Elem {
                    value: 22,
                    is_marked: true,
                },
                Elem {
                    value: 23,
                    is_marked: false,
                },
                Elem {
                    value: 17,
                    is_marked: false,
                },
                Elem {
                    value: 92,
                    is_marked: true,
                },
                Elem {
                    value: 88,
                    is_marked: true,
                },
            ],
        ];
    }
}
