use crate::create_day;
use itertools::Itertools;

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

enum PlayerIntention {
    Win,
    Draw,
    Loss,
}

impl TryFrom<&str> for PlayerIntention {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(PlayerIntention::Loss),
            "Y" => Ok(PlayerIntention::Draw),
            "Z" => Ok(PlayerIntention::Win),
            _ => Err(format!("Illegal player intention {value}")),
        }
    }
}

create_day!(
    2,
    |file_content: String| {
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
    },
    |file_content: String| {
        let turns_as_strings: Vec<(&str, &str)> = file_content
            .lines()
            .map(|line| line.split(' ').collect_tuple())
            .map(Option::unwrap)
            .collect_vec();

        let total_score = turns_as_strings
            .into_iter()
            .map(|(opponent, intention)| {
                (
                    Move::try_from(opponent).unwrap(),
                    get_fitting_player_move(
                        Move::try_from(opponent).unwrap(),
                        PlayerIntention::try_from(intention).unwrap(),
                    ),
                )
            })
            .map(|(opponent, player)| round_score(opponent, player))
            .sum::<i32>();

        println!("{total_score}");
    }
);

fn get_fitting_player_move(opponent: Move, player: PlayerIntention) -> Move {
    match (opponent, player) {
        (any_move, PlayerIntention::Draw) => any_move,
        (Move::Rock, PlayerIntention::Loss) => Move::Scissors,
        (Move::Rock, PlayerIntention::Win) => Move::Paper,
        (Move::Paper, PlayerIntention::Win) => Move::Scissors,
        (Move::Paper, PlayerIntention::Loss) => Move::Rock,
        (Move::Scissors, PlayerIntention::Win) => Move::Rock,
        (Move::Scissors, PlayerIntention::Loss) => Move::Paper,
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
        .collect()
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
