#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_str(s: &str) -> Option<Direction> {
        match s {
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub struct Rope {
    pub start: Coordinate,
    pub other: Vec<Coordinate>,
}

impl Rope {
    pub fn new(start: Coordinate, other: Vec<Coordinate>) -> Rope {
        Rope { start, other }
    }

    pub fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.start.y += 1,
            Direction::Down => self.start.y -= 1,
            Direction::Left => self.start.x -= 1,
            Direction::Right => self.start.x += 1,
        }
        self.update_next(0);
    }

    fn update_next(&mut self, index: usize) {
        if index >= self.other.len() {
            return;
        }

        let head = if index == 0 {
            self.start
        } else {
            self.other[index - 1]
        };

        let tail = self.other[index];

        let dx = head.x - tail.x;
        let dy = head.y - tail.y;

        if dx.abs() > 1 || dy.abs() > 1 {
            self.other[index].x += dx.signum();
            self.other[index].y += dy.signum();
        }

        self.update_next(index + 1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rope_movement() {
        let mut rope = super::Rope::new(
            super::Coordinate { x: 0, y: 0 },
            vec![super::Coordinate { x: 0, y: 0 }],
        );

        rope.move_head(super::Direction::Up);
        assert_eq!(rope.start, super::Coordinate { x: 0, y: 1 });
        assert_eq!(rope.other[0], super::Coordinate { x: 0, y: 0 });

        rope.move_head(super::Direction::Up);
        assert_eq!(rope.start, super::Coordinate { x: 0, y: 2 });
        assert_eq!(rope.other[0], super::Coordinate { x: 0, y: 1 });

        rope.move_head(super::Direction::Right);
        assert_eq!(rope.start, super::Coordinate { x: 1, y: 2 });
        assert_eq!(rope.other[0], super::Coordinate { x: 0, y: 1 });

        rope.move_head(super::Direction::Right);
        assert_eq!(rope.start, super::Coordinate { x: 2, y: 2 });
        assert_eq!(rope.other[0], super::Coordinate { x: 1, y: 2 });
    }

    #[test]
    fn test_rope_movement_with_multiple_segments() {
        let mut rope = super::Rope::new(
            super::Coordinate { x: 0, y: 0 },
            vec![super::Coordinate { x: 0, y: 0 }; 9],
        );
        let movements = [
            (super::Direction::Right, 5),
            (super::Direction::Up, 8),
            (super::Direction::Left, 8),
            (super::Direction::Down, 3),
            (super::Direction::Right, 17),
            (super::Direction::Down, 10),
            (super::Direction::Left, 25),
            (super::Direction::Up, 20),
        ];
        let mut visited = std::collections::HashSet::from([(0, 0)]);

        for (direction, steps) in movements {
            for _ in 0..steps {
                rope.move_head(direction);
                let tail = rope.other[8];
                visited.insert((tail.x, tail.y));
            }
        }

        let expected_grid = [
            "..........................",
            "..........................",
            "..........................",
            "..........................",
            "..........................",
            "..........................",
            "..........................",
            "..........................",
            "..........................",
            "#.........................",
            "#.............###.........",
            "#............#...#........",
            ".#..........#.....#.......",
            "..#..........#.....#......",
            "...#........#.......#.....",
            "....#......s.........#....",
            ".....#..............#.....",
            "......#............#......",
            ".......#..........#.......",
            "........#........#........",
            ".........########.........",
        ];
        let (origin_x, origin_y) = expected_grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| row.find('s').map(|x| (x, y)))
            .expect("expected grid must contain the starting position");

        let expected_visited = expected_grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars().enumerate().filter_map(move |(x, position)| {
                    (position == '#' || position == 's')
                        .then_some((x as i32 - origin_x as i32, origin_y as i32 - y as i32))
                })
            })
            .collect::<std::collections::HashSet<_>>();

        assert_eq!(visited, expected_visited);
    }
}
