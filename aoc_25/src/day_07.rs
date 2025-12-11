use std::{cmp::max, collections::HashSet};
use utils::FileReader;

static DAY_07_FILE: &str = "./resources/aoc_25/day_07.txt";

pub struct Day07Processor(String);

impl Day07Processor {
    fn new() -> Self {
        Self(String::from(DAY_07_FILE))
    }

    pub fn process(&self) {
        self.process_part1();
        self.process_part2();
    }

    fn process_part1(&self) {
        let reader = FileReader::new(&self.0);

        match PartOneValue::try_from(reader) {
            Ok(p1_value) => println!("AoC 25 Day 07 Part 1: {}", p1_value.get()),
            Err(msg) => println!("AoC 25 Day 07 Part 1: Failed with message: {}", msg),
        }
    }

    fn process_part2(&self) {
        let reader = FileReader::new(&self.0);

        match PartTwoValue::try_from(reader) {
            Ok(p2_value) => println!("AoC 25 Day 07 Part 2: {}", p2_value.get()),
            Err(msg) => println!("AoC 25 Day 07 Part 2: Failed with message: {}", msg),
        }
    }
}

impl Default for Day07Processor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
struct TachyonManifold {
    diagram: Vec<Vec<char>>,
    tracker: BeamTracker,
}

impl TachyonManifold {
    fn create(input: Day07Input) -> Self {
        let manifold_size = &input.lines.len();
        Self {
            diagram: input.lines,
            tracker: BeamTracker::new(*manifold_size),
        }
    }

    // Determines the total number of splits from a bream traversal of the diagram.
    fn traversal_splits(&mut self) -> i64 {
        self.tracker.init(self.diagram[0].clone());

        for idx in 1..self.diagram.len() {
            self.traverse(idx);
        }

        self.tracker.splits
    }

    fn traverse(&mut self, row_idx: usize) {
        for (pos, value) in self.diagram[row_idx].clone().iter().enumerate() {
            if *value == '^' {
                self.tracker.split(pos);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct BeamTracker {
    splits: i64,
    right_index: usize,
    traversal_indexes: HashSet<usize>,
}

impl BeamTracker {
    fn new(right_index: usize) -> Self {
        Self {
            splits: 0,
            traversal_indexes: HashSet::new(),
            right_index,
        }
    }

    // Looks for the entry point of the beam and tracks that.
    fn init(&mut self, row: Vec<char>) {
        for (pos, value) in row.iter().enumerate() {
            if *value == 'S' {
                self.traversal_indexes.insert(pos);
                break;
            }
        }
    }

    // Creates a split at this index if the beam was traversing through this spot.
    fn split(&mut self, idx: usize) {
        // If we weren't traversing through this spot, we can ignore.
        if self.traversal_indexes.contains(&idx) {
            self.splits += 1;
            // Handle the left split if the idx is greater than 0.
            if idx > 0 {
                self.traversal_indexes.insert(idx - 1);
            }

            // Handle the right split if the idx is less than the right edge index.
            if idx < self.right_index {
                self.traversal_indexes.insert(idx + 1);
            }

            // Remove this index from the traversal tree.
            self.traversal_indexes.remove(&idx);
        }
    }
}

#[derive(Debug, PartialEq)]
struct QuantumTachyonManifold {
    diagram: Vec<Vec<char>>,
}

impl QuantumTachyonManifold {
    fn create(input: Day07Input) -> Self {
        Self {
            diagram: input.lines,
        }
    }

    fn build_grid(rows: usize, columns: usize) -> Vec<Vec<i64>> {
        let cols = vec![0; columns];
        vec![cols; rows]
    }

    // Returns the total number of paths across all timelines.
    fn traverse_all(&mut self) -> i64 {
        let mut grid =
            QuantumTachyonManifold::build_grid(self.diagram.len(), self.diagram[0].len().clone());

        for (row, line) in self.diagram.iter().enumerate() {
            for (col, value) in line.iter().enumerate() {
                if *value == 'S' {
                    grid[row][col] = 1;
                    continue;
                }

                if *value == '.' && row > 0 {
                    grid[row][col] += grid[row - 1][col];
                    continue;
                }

                if *value == '^' {
                    for idx in [col - 1, col + 1] {
                        grid[row][idx] = max(grid[row - 1][col] + grid[row][idx], grid[row][idx]);
                    }
                }
            }
        }

        grid[grid.len() - 1].iter().sum()
    }
}

#[derive(Debug, PartialEq)]
struct Day07Input {
    lines: Vec<Vec<char>>,
}

impl TryFrom<FileReader> for Day07Input {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let lines: Vec<Vec<char>> = reader
            .read_lines()?
            .map_while(Result::ok)
            .map(|x| x.chars().collect())
            .collect();

        Ok(Self { lines })
    }
}

#[derive(Debug, PartialEq)]
struct PartOneValue(i64);

impl PartOneValue {
    fn get(&self) -> i64 {
        self.0
    }
}

impl TryFrom<FileReader> for PartOneValue {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let input = Day07Input::try_from(reader)?;
        let mut manifold = TachyonManifold::create(input);

        Ok(Self(manifold.traversal_splits()))
    }
}

#[derive(Debug, PartialEq)]
struct PartTwoValue(i64);

impl PartTwoValue {
    #[allow(dead_code)]
    fn get(&self) -> i64 {
        self.0
    }
}

impl TryFrom<FileReader> for PartTwoValue {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let input = Day07Input::try_from(reader)?;
        let mut manifold = QuantumTachyonManifold::create(input);

        Ok(Self(manifold.traverse_all()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_FILE: &str = "../test-resources/aoc_25/day_07_sample.txt";

    #[test]
    fn test_input_from_file_reader() {
        let reader = FileReader::new(SAMPLE_FILE);
        let input = Day07Input::try_from(reader).unwrap();

        assert_eq!(
            ".......S.......".chars().collect::<Vec<char>>(),
            input.lines[0]
        )
    }

    #[test]
    fn test_part_one_value_from_reader() {
        let reader = FileReader::new(SAMPLE_FILE);
        let p1_value = PartOneValue::try_from(reader).unwrap();

        assert_eq!(21, p1_value.get())
    }

    #[test]
    fn test_part_two_value_from_reader() {
        let reader = FileReader::new(SAMPLE_FILE);
        let p2_value = PartTwoValue::try_from(reader).unwrap();

        assert_eq!(40, p2_value.get())
    }
}
