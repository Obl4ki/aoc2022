use crate::{create_day, day_8::grid::Grid};

create_day!(8, part_1, part_2);

fn part_1(file_content: String) {
    let grid = Grid::from(file_content);
    let visible = grid.get_visible();

    println!("Visible elements: {:?}", visible.len());
}

fn part_2(file_content: String) {
    let grid = Grid::from(file_content);
    let max_scenic_score = grid.get_biggest_scenic_score();

    println!("Max scenic score: {}", max_scenic_score);
}
