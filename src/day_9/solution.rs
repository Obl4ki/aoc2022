use crate::{
    create_day,
    day_9::rope::{self, Direction},
};

create_day!(9, part_1, part_2);

fn part_1(file_content: String) {
    let mut rope = rope::Rope::new(
        rope::Coordinate { x: 0, y: 0 },
        rope::Coordinate { x: 0, y: 0 },
    );

    let mut visited = std::collections::HashSet::new();

    for line in file_content.lines() {
        let mut parts = line.split_whitespace();

        let (direction_letter, steps) = match (parts.next(), parts.next()) {
            (Some(dir), Some(steps)) => (dir, steps),
            _ => panic!("Invalid input line: {}", line),
        };

        let steps = steps.parse::<usize>().expect("Invalid number of steps");

        let direction = Direction::from_str(direction_letter).expect("Invalid direction");

        for _ in 0..steps {
            rope.move_head(direction);
            visited.insert((rope.end.x, rope.end.y));
        }
    }

    println!("Part 1: {}", visited.len());
}

#[allow(unused)]
fn part_2(file_content: String) {}
