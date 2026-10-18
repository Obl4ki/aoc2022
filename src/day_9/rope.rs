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
    pub end: Coordinate,
}

impl Rope {
    pub fn new(start: Coordinate, end: Coordinate) -> Rope {
        Rope { start, end }
    }

    pub fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.start.y += 1,
            Direction::Down => self.start.y -= 1,
            Direction::Left => self.start.x -= 1,
            Direction::Right => self.start.x += 1,
        }
        self.update_tail();
    }

    fn update_tail(&mut self) {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;

        if dx.abs() > 1 || dy.abs() > 1 {
            if dx != 0 {
                self.end.x += dx.signum();
            }
            if dy != 0 {
                self.end.y += dy.signum();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rope_movement() {
        let mut rope = super::Rope::new(
            super::Coordinate { x: 0, y: 0 },
            super::Coordinate { x: 0, y: 0 },
        );

        rope.move_head(super::Direction::Up);
        assert_eq!(rope.start, super::Coordinate { x: 0, y: 1 });
        assert_eq!(rope.end, super::Coordinate { x: 0, y: 0 });

        rope.move_head(super::Direction::Up);
        assert_eq!(rope.start, super::Coordinate { x: 0, y: 2 });
        assert_eq!(rope.end, super::Coordinate { x: 0, y: 1 });

        rope.move_head(super::Direction::Right);
        assert_eq!(rope.start, super::Coordinate { x: 1, y: 2 });
        assert_eq!(rope.end, super::Coordinate { x: 0, y: 1 });

        rope.move_head(super::Direction::Right);
        assert_eq!(rope.start, super::Coordinate { x: 2, y: 2 });
        assert_eq!(rope.end, super::Coordinate { x: 1, y: 2 });
    }
}
