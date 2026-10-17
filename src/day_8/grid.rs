use std::collections::HashMap;

pub struct Grid {
    // Represented as a flat vector, with elements stored in row-major order
    elements: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

type VisibleElementPosition = (usize, usize); // (row_idx, col_idx)
type VisibleElements = HashMap<VisibleElementPosition, u32>; // (row_idx, col_idx) -> height

impl From<String> for Grid {
    fn from(file_content: String) -> Self {
        let width = file_content
            .lines()
            .next()
            .map(|line| line.len())
            .unwrap_or(0);
        let height = file_content.lines().count();

        let elements = file_content
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
            .collect();

        Grid {
            elements,
            width,
            height,
        }
    }
}

impl Grid {
    pub fn get(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.height && y < self.width {
            Some(self.elements[y * self.width + x])
        } else {
            None
        }
    }

    /// Returns a HashMap of all visible elements in the grid, with their positions as keys.
    pub fn get_visible(&self) -> VisibleElements {
        let mut visible_elements: VisibleElements = HashMap::new();

        // left -> right
        for row_idx in 0..self.height {
            let mut max_height = None;
            for col_idx in 0..self.width {
                let current_height = self.get(row_idx, col_idx).unwrap();
                if max_height.is_none() || current_height > max_height.unwrap() {
                    visible_elements.insert((row_idx, col_idx), current_height);
                    max_height = Some(current_height);
                }
            }
        }

        // right -> left
        for row_idx in 0..self.height {
            let mut max_height = None;
            for col_idx in (0..self.width).rev() {
                let current_height = self.get(row_idx, col_idx).unwrap();
                if max_height.is_none() || current_height > max_height.unwrap() {
                    visible_elements.insert((row_idx, col_idx), current_height);
                    max_height = Some(current_height);
                }
            }
        }

        // top -> bottom
        for col_idx in 0..self.width {
            let mut max_height = None;
            for row_idx in 0..self.height {
                let current_height = self.get(row_idx, col_idx).unwrap();
                if max_height.is_none() || current_height > max_height.unwrap() {
                    visible_elements.insert((row_idx, col_idx), current_height);
                    max_height = Some(current_height);
                }
            }
        }

        // bottom -> top
        for col_idx in 0..self.width {
            let mut max_height = None;
            for row_idx in (0..self.height).rev() {
                let current_height = self.get(row_idx, col_idx).unwrap();
                if max_height.is_none() || current_height > max_height.unwrap() {
                    visible_elements.insert((row_idx, col_idx), current_height);
                    max_height = Some(current_height);
                }
            }
        }

        visible_elements
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::testing::assert_equal_hashmaps;

    use super::*;

    #[test]
    fn test_grid_from_string() {
        let input = "123\n456\n789".to_string();
        let grid = Grid::from(input);

        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.get(0, 0), Some(1));
        assert_eq!(grid.get(1, 1), Some(5));
        assert_eq!(grid.get(2, 2), Some(9));
    }

    #[test]
    fn test_get_visible() {
        let input = "30373\n25512\n65332\n33549\n35390".to_string();
        let grid = Grid::from(input);
        let actual_visible = grid.get_visible();

        let expected_visible = HashMap::from([
            ((0, 0), 3),
            ((0, 1), 2),
            ((0, 2), 6),
            ((0, 3), 3),
            ((0, 4), 3),
            ((1, 0), 0),
            ((1, 1), 5),
            ((1, 2), 5),
            ((1, 4), 5),
            ((2, 0), 3),
            ((2, 1), 5),
            ((2, 3), 5),
            ((2, 4), 3),
            ((3, 0), 7),
            ((3, 2), 3),
            ((3, 4), 9),
            ((4, 0), 3),
            ((4, 1), 2),
            ((4, 2), 2),
            ((4, 3), 9),
            ((4, 4), 0),
        ]);

        assert_equal_hashmaps(&actual_visible, &expected_visible);
    }
}
