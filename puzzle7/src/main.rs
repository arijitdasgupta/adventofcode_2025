use std::{
    collections::{HashSet, VecDeque},
    io::{Read, stdin},
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Block {
    Blank,
    Prism,
}

struct BlockPlan {
    items: Vec<Block>,
    width: usize,
    height: usize,
}

impl From<&str> for BlockPlan {
    fn from(value: &str) -> Self {
        let mut items = vec![];
        let mut height = 0;
        for i in value.chars() {
            match i {
                '.' => items.push(Block::Blank),
                '^' => items.push(Block::Prism),
                '\n' => height += 1,
                _ => panic!("unexpected character"),
            }
        }

        let width = items.len() / height;
        // This is only because the the file does not end with a new line

        assert!(
            items.len() % (height as usize) == 0,
            "should be a rounded rect"
        );

        Self {
            items,
            width,
            height,
        }
    }
}

impl BlockPlan {
    fn get_item_at(&self, (row, col): (i32, i32)) -> Option<Block> {
        if row < 0 || col < 0 {
            return None;
        } else if row as usize >= self.width || col as usize >= self.height {
            return None;
        }

        let row: usize = row.try_into().expect("problem monsueir!");
        let col: usize = col.try_into().expect("problem monseuir col!");

        let idx = (row * self.width) + col;

        Some(self.items[idx])
    }

    fn get_bottom_neighbours(
        &self,
        (row, col): (i32, i32),
    ) -> (
        Option<(Block, (i32, i32))>,
        Option<(Block, (i32, i32))>,
        Option<(Block, (i32, i32))>,
    ) {
        let coords = ((row + 1, col - 1), (row + 1, col), (row + 1, col + 1));
        (
            self.get_item_at(coords.0).map(|r| (r, coords.0)),
            self.get_item_at(coords.1).map(|r| (r, coords.1)),
            self.get_item_at(coords.2).map(|r| (r, coords.2)),
        )
    }

    fn get_bottom_neighbour(&self, (row, col): (i32, i32)) -> Option<(Block, (i32, i32))> {
        let coord = (row + 1, col);
        self.get_item_at(coord).map(|c| (c, coord))
    }
}

struct Traversal {
    traversal_q: VecDeque<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

impl Traversal {
    fn new(starting_position: (i32, i32)) -> Self {
        let mut q = VecDeque::new();
        q.push_back(starting_position);
        let mut v = HashSet::new();
        v.insert(starting_position);
        Self {
            traversal_q: q,
            visited: v,
        }
    }

    fn traverse_multiverse(mut self, block_plan: &BlockPlan) -> u32 {
        let mut line_count = 1;
        while self.traversal_q.len() != 0 {
            let coord = self
                .traversal_q
                .pop_front()
                .expect("Something must have gone horribly wrong Doctorr!");
            let bottom_neighbours = block_plan.get_bottom_neighbours(coord);

            match bottom_neighbours {
                (Some((Block::Blank, l_coord)), Some((Block::Prism, _)), None) => {
                    self.traversal_q.push_back(l_coord);
                }
                (None, Some((Block::Prism, _)), Some((Block::Blank, r_coord))) => {
                    self.traversal_q.push_back(r_coord);
                }
                (
                    Some((Block::Blank, l_coord)),
                    Some((Block::Prism, _)),
                    Some((Block::Blank, r_coord)),
                ) => {
                    line_count += 1;
                    self.traversal_q.push_back(l_coord);
                    self.traversal_q.push_back(r_coord);
                }
                (
                    Some((Block::Blank, l_coord)),
                    Some((Block::Prism, _)),
                    Some((Block::Prism, _)),
                ) => {
                    self.traversal_q.push_back(l_coord);
                }
                (
                    Some((Block::Prism, _)),
                    Some((Block::Prism, _)),
                    Some((Block::Blank, r_coord)),
                ) => {
                    self.traversal_q.push_back(r_coord);
                }
                (_, Some((Block::Blank, m_coord)), _) => {
                    let mut coord = m_coord;
                    while let Some((Block::Blank, new_coord)) =
                        block_plan.get_bottom_neighbour(coord)
                    {
                        coord = new_coord;
                    }
                    self.traversal_q.push_back(coord);
                }
                (_, _, _) => continue,
            }
        }

        line_count
    }

    fn traverse(mut self, block_plan: &BlockPlan) -> u32 {
        let mut split_count = 0;
        while self.traversal_q.len() != 0 {
            let coord = self
                .traversal_q
                .pop_front()
                .expect("Something must have gone horribly wrong");
            let bottom_neighbours = block_plan.get_bottom_neighbours(coord);

            match bottom_neighbours {
                (Some((Block::Blank, l_coord)), Some((Block::Prism, _)), None) => {
                    split_count += 1;
                    self.next_step(l_coord);
                }
                (None, Some((Block::Prism, _)), Some((Block::Blank, r_coord))) => {
                    split_count += 1;
                    self.next_step(r_coord);
                }
                (
                    Some((Block::Blank, l_coord)),
                    Some((Block::Prism, _)),
                    Some((Block::Blank, r_coord)),
                ) => {
                    split_count += 1;
                    self.next_step(l_coord);
                    self.next_step(r_coord);
                }
                (_, Some((Block::Blank, m_coord)), _) => {
                    self.next_step(m_coord);
                }
                (_, _, _) => continue,
            }
        }

        split_count
    }

    fn next_step(&mut self, coord: (i32, i32)) {
        if !self.visited.contains(&coord) {
            self.visited.insert(coord);
            self.traversal_q.push_back(coord);
        }
    }
}

fn split_and_line_count(s: &str) -> (u32, u32) {
    let first_line_idx = s
        .find("\n")
        .expect("the input should have at least more than one line");
    let first_line = &s[0..=first_line_idx];
    let rest_lines = &s[first_line_idx + 1..];

    let start_column = first_line
        .find("S")
        .expect("should be at least an S monseuir");

    let start_position = (0, start_column as i32); // Should be fine
    let block_plan: BlockPlan = rest_lines.into();

    let traversal = Traversal::new(start_position);
    let split_count = traversal.traverse(&block_plan);

    println!("split count {}", split_count);
    let traversal = Traversal::new(start_position);
    let line_count = traversal.traverse_multiverse(&block_plan);

    (split_count, line_count)
}

fn main() {
    let mut in_buf: String = Default::default();
    let _ = stdin().lock().read_to_string(&mut in_buf);

    let (split_count, line_count) = split_and_line_count(&in_buf);
    println!("Split count: {split_count}");
    println!("Line count: {line_count}");
}

#[cfg(test)]
mod test {
    use crate::{Block, BlockPlan, split_and_line_count};

    #[test]
    fn parse_text_1() {
        let test_input = include_str!("../test_input1.txt");
        let block_plan: BlockPlan = test_input.into();
        assert_eq!(block_plan.get_item_at((1, 7)), Some(Block::Prism));
    }

    #[test]
    fn parse_text_2() {
        let test_input = include_str!("../test_input1.txt");
        let block_plan: BlockPlan = test_input.into();
        assert_eq!(block_plan.get_item_at((-1, 7)), None);
    }

    #[test]
    fn parse_text_3() {
        let test_input = include_str!("../test_input1.txt");
        let block_plan: BlockPlan = test_input.into();
        assert_eq!(block_plan.get_item_at((0, 20)), None);
    }

    #[test]
    fn parse_text_4() {
        let test_input = include_str!("../test_input1.txt");
        let block_plan: BlockPlan = test_input.into();
        assert_eq!(block_plan.get_item_at((0, 0)), Some(Block::Blank));
    }

    #[test]
    fn parse_text_5() {
        let test_input = include_str!("../test_input1.txt");
        let block_plan: BlockPlan = test_input.into();
        assert_eq!(block_plan.get_item_at((20, 0)), None);
    }

    #[test]
    fn get_bottom_neighbours() {
        let test_input = include_str!("../test_input1.txt");
        let block_plan: BlockPlan = test_input.into();
        let nbs = block_plan.get_bottom_neighbours((0, 0));

        assert_eq!(
            nbs,
            (
                None,
                Some((Block::Blank, (1, 0))),
                Some((Block::Blank, (1, 1)))
            )
        );
    }

    #[test]
    fn split_count_1() {
        let test_input = include_str!("../test_input_complete_1.txt");
        let split_count = split_and_line_count(test_input).0;

        assert_eq!(split_count, 21);
    }

    #[test]
    fn lines_count_1() {
        let test_input = include_str!("../test_input_complete_1.txt");
        let line_count = split_and_line_count(test_input).1;

        assert_eq!(line_count, 40);
    }
}
