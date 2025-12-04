use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FloorBlock {
    Blank,
    Paper,
}

struct FloorPlan {
    blocks: Vec<FloorBlock>,
    width: usize,
}

impl FloorPlan {
    fn new(blocks: Vec<FloorBlock>, width: usize) -> Self {
        Self { blocks, width }
    }

    fn parse_from_str(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            panic!("Empty input");
        }

        let first_line_length = lines[0].len();
        let num_lines = lines.len();

        println!("First line length: {}", first_line_length);
        println!("Number of lines: {}", num_lines);

        assert_eq!(num_lines, first_line_length, "the grid should be square");
        let width = first_line_length;

        let mut blocks: Vec<FloorBlock> = Vec::new();

        for line in lines {
            for ch in line.chars() {
                let block = match ch {
                    '.' => FloorBlock::Blank,
                    '@' => FloorBlock::Paper,
                    _ => {
                        panic!("unknown character, exiting program");
                    }
                };
                blocks.push(block);
            }
        }

        Self { blocks, width }
    }

    fn get_neighbours_at(&self, row: usize, col: usize) -> Vec<Option<FloorBlock>> {
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1), // top-left, top, top-right
            (0, -1),
            (0, 1), // left, right
            (1, -1),
            (1, 0),
            (1, 1), // bottom-left, bottom, bottom-right
        ];

        directions
            .iter()
            .map(|(dr, dc)| {
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;

                if new_row >= 0
                    && new_row < self.width as isize
                    && new_col >= 0
                    && new_col < self.width as isize
                {
                    self.get_block_at(new_row as usize, new_col as usize)
                } else {
                    None
                }
            })
            .collect()
    }

    fn idx(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.width || col >= self.width {
            None
        } else {
            let idx = row * self.width + col;
            Some(idx)
        }
    }

    fn get_block_at(&self, row: usize, col: usize) -> Option<FloorBlock> {
        if let Some(idx) = self.idx(row, col) {
            Some(self.blocks[idx])
        } else {
            None
        }
    }

    fn remove_roll_at(&mut self, row: usize, col: usize) {
        let idx = self.idx(row, col).unwrap();
        self.blocks[idx] = FloorBlock::Blank;
    }

    fn get_pickable_rolls(&self) -> Vec<(usize, usize)> {
        let mut removable_rolls = vec![];

        for i in 0..self.width {
            for j in 0..self.width {
                let (neighbours, at_place) =
                    (self.get_neighbours_at(i, j), self.get_block_at(i, j));

                if at_place == Some(FloorBlock::Paper)
                    && neighbours
                        .iter()
                        .filter(|x| **x == Some(FloorBlock::Paper))
                        .count()
                        < 4
                {
                    removable_rolls.push((i, j));
                }
            }
        }

        removable_rolls
    }

    fn remove_rolls_until(&mut self) -> usize {
        let mut rolls_removed = 0;
        while self.get_pickable_rolls().len() != 0 {
            let pickable_rolls = self.get_pickable_rolls();
            rolls_removed += pickable_rolls.len();

            pickable_rolls.iter().for_each(|(row, col)| {
                self.remove_roll_at(*row, *col);
            });
        }
        rolls_removed
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut plan = FloorPlan::parse_from_str(&input);
    let number_of_rolls = plan.get_pickable_rolls().len();
    println!("Can be removed after the first one: {number_of_rolls}");

    let removed_rolls = plan.remove_rolls_until();
    println!("total removed rolls: {removed_rolls}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_grid() {
        let input = "..@.\n.@@.\n....\n....";
        let plan = FloorPlan::parse_from_str(input);

        assert_eq!(plan.width, 4);
        assert_eq!(plan.blocks.len(), 16);
        assert_eq!(plan.blocks[0], FloorBlock::Blank);
        assert_eq!(plan.blocks[2], FloorBlock::Paper);
    }

    #[test]
    fn test_get_block_at() {
        let input = "..@.\n.@@.\n....\n....";
        let plan = FloorPlan::parse_from_str(input);

        assert_eq!(plan.get_block_at(0, 0), Some(FloorBlock::Blank));
        assert_eq!(plan.get_block_at(0, 2), Some(FloorBlock::Paper));
        assert_eq!(plan.get_block_at(1, 1), Some(FloorBlock::Paper));
        assert_eq!(plan.get_block_at(2, 0), Some(FloorBlock::Blank));
        assert_eq!(plan.get_block_at(4, 0), None);
        assert_eq!(plan.get_block_at(0, 4), None);
    }

    #[test]
    #[should_panic(expected = "Empty input")]
    fn test_empty_input_panics() {
        let input = "";
        FloorPlan::parse_from_str(input);
    }

    #[test]
    fn test_get_neighbours_center() {
        let input = "...\n.@.\n...";
        let plan = FloorPlan::parse_from_str(input);

        let neighbours = plan.get_neighbours_at(1, 1);
        assert_eq!(neighbours.len(), 8);
        assert!(neighbours.iter().all(|n| n == &Some(FloorBlock::Blank)));
    }

    #[test]
    fn test_get_neighbours_corner() {
        let input = "@..\n...\n...";
        let plan = FloorPlan::parse_from_str(input);

        let neighbours = plan.get_neighbours_at(0, 0);
        assert_eq!(neighbours.len(), 8);
        let valid_count = neighbours.iter().filter(|n| n.is_some()).count();
        assert_eq!(valid_count, 3);

        assert_eq!(neighbours[4], Some(FloorBlock::Blank));
        assert_eq!(neighbours[7], Some(FloorBlock::Blank));
        assert_eq!(neighbours[6], Some(FloorBlock::Blank));
    }

    #[test]
    fn test_get_neighbours_edge() {
        let input = "...\n@..\n...";
        let plan = FloorPlan::parse_from_str(input);

        let neighbours = plan.get_neighbours_at(1, 0);
        assert_eq!(neighbours.len(), 8);
        let valid_count = neighbours.iter().filter(|n| n.is_some()).count();
        assert_eq!(valid_count, 5);
    }
}
