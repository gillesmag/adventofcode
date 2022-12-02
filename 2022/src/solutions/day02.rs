#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn parse_input(lines: &str) -> Vec<Vec<Move>> {
    lines
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|v| match v {
                    "A" | "X" => Move::Rock,
                    "B" | "Y" => Move::Paper,
                    "C" | "Z" => Move::Scissors,
                    _ => panic!("Unexpected character"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
}

fn is_win(opponent: &Move, player: &Move) -> bool {
    match (opponent, player) {
        (Move::Scissors, Move::Rock) => true,
        (Move::Paper, Move::Scissors) => true,
        (Move::Rock, Move::Paper) => true,
        _ => false,
    }
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

fn is_draw(opponent: &Move, player: &Move) -> bool {
    player == opponent
}

fn shape_score(m: &Move) -> usize {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn part_a(lines: &str) -> usize {
    let input = parse_input(lines);
    let mut player_score = 0;
    for round in input {
        let mut current_score = 0;

        let (opponent, player) = (&round[0], &round[1]);
        let is_player_win = !is_draw(opponent, player) && is_win(opponent, player);
        current_score += shape_score(player);

        if is_draw(opponent, player) {
            current_score += 3;
        } else {
            if is_player_win {
                current_score += 6;
            }
        }

        println!("{}", current_score);
        player_score += current_score;
    }
    player_score
}

fn part_b(lines: &str) -> usize {
    let input = parse_input(lines);
    let mut player_score = 0;
    for round in input {
        let (opponent, player) = (&round[0], &round[1]);

        let current_score = match player {
            Move::Rock => shape_score(&losing_move(opponent)) + 0,
            Move::Paper => shape_score(opponent) + 3,
            Move::Scissors => shape_score(&winning_move(opponent)) + 6,
        };

        player_score += current_score;
    }
    player_score
}

pub fn day02(input: &str) -> (String, String) {
    (part_a(input).to_string(), part_b(input).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_file;

    #[test]
    fn test_example_part_a() {
        let input = read_file("examples", 2);
        assert_eq!(part_a(&input), 15);
    }

    #[test]
    fn test_example_part_b() {
        let input = read_file("examples", 2);
        assert_eq!(part_b(&input), 12);
    }

    #[test]
    fn test_input_part_a() {
        let input = read_file("inputs", 2);
        assert_eq!(part_a(&input), 15572);
    }

    #[test]
    fn test_input_part_b() {
        let input = read_file("inputs", 2);
        assert_eq!(part_b(&input), 16098);
    }
}
