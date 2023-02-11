use std::collections::HashSet;

use itertools::Itertools;

use crate::create_day;

create_day!(
    6,
    |file_content: String| {
        let n = 4;
        let characters = file_content.chars().collect_vec();
        let all_window_positions = 0..characters.len() - n;

        let position = all_window_positions
            .map(move |start| start..start + n)
            .map(|range| characters.get(range).unwrap())
            .position(is_start_of_packet_marker)
            .expect("There should be a solution.")
            + n;

        println!("{position}");
    },
    |file_content: String| {
        let n = 14;
        let characters = file_content.chars().collect_vec();
        let all_window_positions = 0..characters.len() - n;

        let position = all_window_positions
            .map(move |start| start..start + n)
            .map(|range| characters.get(range).unwrap())
            .position(is_start_of_packet_marker)
            .expect("There should be a solution.")
            + n;

        println!("{position}");
    }
);

fn is_start_of_packet_marker(characters: &[char]) -> bool {
    let hs: HashSet<char> = HashSet::from_iter(characters.iter().map(ToOwned::to_owned));
    hs.len() == characters.len()
}
