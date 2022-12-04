use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn parse(input: &str) -> Vec<(Move, Move)> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|v| match v {
                    "A" | "X" => Move::Rock,
                    "B" | "Y" => Move::Paper,
                    "C" | "Z" => Move::Scissors,
                    _ => panic!("Unexpected character"),
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn losing_move(opponent: &Move) -> Move {
    match opponent {
        Move::Rock => Move::Scissors,
        Move::Scissors => Move::Paper,
        Move::Paper => Move::Rock,
    }
}

fn winning_move(opponent: &Move) -> Move {
    match opponent {
        Move::Scissors => Move::Rock,
        Move::Paper => Move::Scissors,
        Move::Rock => Move::Paper,
    }
}

fn shape_score(m: &Move) -> usize {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn part_a(rounds: &Vec<(Move, Move)>) -> usize {
    rounds
        .into_iter()
        .map(|(opponent, player)| {
            let is_win = match (opponent, player) {
                (Move::Scissors, Move::Rock) => true,
                (Move::Paper, Move::Scissors) => true,
                (Move::Rock, Move::Paper) => true,
                _ => false,
            };

            shape_score(player)
                + if is_win {
                    6
                } else if opponent == player {
                    3
                } else {
                    0
                }
        })
        .sum()
}

fn part_b(rounds: &Vec<(Move, Move)>) -> usize {
    rounds
        .into_iter()
        .map(|(opponent, player)| match player {
            Move::Rock => shape_score(&losing_move(opponent)) + 0,
            Move::Paper => shape_score(opponent) + 3,
            Move::Scissors => shape_score(&winning_move(opponent)) + 6,
        })
        .sum()
}

pub fn day02(input: &str) -> (String, String) {
    let input = parse(input);
    (part_a(&input).to_string(), part_b(&input).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 2);
        assert_eq!(part_a(&parse(&input.unwrap())), 15);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 2);
        assert_eq!(part_b(&parse(&input.unwrap())), 12);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 2);
        assert_eq!(part_a(&parse(&input.unwrap())), 15572);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 2);
        assert_eq!(part_b(&parse(&input.unwrap())), 16098);
    }
}
