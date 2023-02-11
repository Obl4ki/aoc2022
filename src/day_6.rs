use std::collections::HashSet;

use itertools::Itertools;

use crate::create_day;

create_day!(
    6,
    |file_content: String| {
        let index: usize = file_content
            .chars()
            .tuple_windows()
            .position(is_start_of_packet_marker)
            .expect("There should be a solution.")
            + 4; // because this calculates the beginning of marker and we want the ending
        println!("{index}")
    },
    |file_content: String| {}
);

fn is_start_of_packet_marker(characters: (char, char, char, char)) -> bool {
    let hs = HashSet::from([characters.0, characters.1, characters.2, characters.3]);
    hs.len() == 4
}
