use std::fs;

use crate::core_traits::Solution;
use itertools::Itertools;
pub struct Day2;

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for Move {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(format!("Illegal move {value}")),
        }
    }
}

impl Solution for Day2 {
    fn execute_first(&self) {
        let file_content = fs::read_to_string("data/day_2.txt").unwrap();
        let file_content = file_content.replace('\r', "");

        let turns_as_strings: Vec<(&str, &str)> = file_content
            .lines()
            .map(|line| line.split(' ').collect_tuple())
            .map(Option::unwrap)
            .collect_vec();

        let turns = parse_turns(turns_as_strings);
        let total_score = turns
            .into_iter()
            .map(|(opponent, player)| round_score(opponent.unwrap(), player.unwrap()))
            .sum::<i32>();
            
        println!("{total_score}")
    }

    fn execute_second(&self) {
        todo!()
    }
}

fn parse_turns(
    turn_as_str: Vec<(&str, &str)>,
) -> Vec<(Result<Move, String>, Result<Move, String>)> {
    turn_as_str
        .into_iter()
        .enumerate()
        .map(|(line_idx, (opponent, player))| {
            (
                match Move::try_from(opponent) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(format!(
                        "{e}: opponent played an illegal move {opponent} at {line_idx}"
                    )),
                },
                match Move::try_from(player) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(format!(
                        "{e}: player played an illegal move {player} at {line_idx}"
                    )),
                },
            )
        })
        .collect_vec()
}

fn round_score(opponent: Move, player: Move) -> i32 {
    let comparison_points = match (&opponent, &player) {
        (Move::Rock, Move::Paper)
        | (Move::Paper, Move::Scissors)
        | (Move::Scissors, Move::Rock) => 6,
        (Move::Scissors, Move::Scissors)
        | (Move::Rock, Move::Rock)
        | (Move::Paper, Move::Paper) => 3,
        (Move::Rock, Move::Scissors)
        | (Move::Paper, Move::Rock)
        | (Move::Scissors, Move::Paper) => 0,
    };

    let player_move_bonus = match &player {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    comparison_points + player_move_bonus
}
