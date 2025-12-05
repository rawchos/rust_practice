use utils::FileReader;

static DAY_04_FILE: &str = "./resources/aoc_25/day_04.txt";

pub struct Day04Processor(String);

impl Day04Processor {
    fn new() -> Self {
        Self(String::from(DAY_04_FILE))
    }

    pub fn process(&self) {
        self.process_part1()
    }

    fn process_part1(&self) {
        let file_reader = FileReader::new(&self.0);

        match PaperRollGrid::try_from(file_reader) {
            Ok(grid) => println!("AoC 25 Day 04 Part 1: {}", grid.removable_rolls()),
            Err(msg) => println!("AoC 25 Day 04 Part 1: Failed with message: {}", msg),
        }
    }
}

impl Default for Day04Processor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
struct PaperRollGrid {
    // The use of a buffered grid is for convenience. I build a buffer around the whole grid
    // so that I don't have to worry about any index out of bounds errors. If I'm looking at
    // a specific position and it's a BUFFERED_CHAR, I can just skip over it.
    buffered_grid: Vec<Vec<char>>,
}

impl PaperRollGrid {
    const BUFFER_CHAR: char = '=';
    const PAPER_ROLL: char = '@';

    fn new(size: usize) -> Self {
        // Create the first and last buffer rows
        let buffer_row: Vec<char> = vec![PaperRollGrid::BUFFER_CHAR; size + 2];
        let buffered_grid: Vec<Vec<char>> = vec![buffer_row.clone(), buffer_row];

        Self { buffered_grid }
    }

    fn add_row(&mut self, row: &str) {
        let current_size = self.buffered_grid.len();
        let buffered_row =
            PaperRollGrid::BUFFER_CHAR.to_string() + row + &PaperRollGrid::BUFFER_CHAR.to_string();

        self.buffered_grid.insert(
            current_size - 1,
            buffered_row.chars().collect::<Vec<char>>(),
        )
    }

    fn removable_rolls(&self) -> i64 {
        let mut removable_rolls: i64 = 0;

        // For simple sake, we can skip the first and last
        // rows of the grid since they're buffer rows.
        for row in 1..self.buffered_grid.len() {
            for column in 1..self.buffered_grid[row].len() {
                if self.buffered_grid[row][column] == PaperRollGrid::PAPER_ROLL
                    && self.is_removable(Position(row, column))
                {
                    removable_rolls += 1;
                }
            }
        }

        removable_rolls
    }

    fn is_removable(&self, position: Position) -> bool {
        let neighbors = self.neighbors(position);
        let paper_rolls = neighbors
            .into_iter()
            .filter(|pr| *pr == PaperRollGrid::PAPER_ROLL)
            .collect::<Vec<char>>()
            .len();

        paper_rolls < 4
    }

    fn neighbors(&self, position: Position) -> Vec<char> {
        let mut neighbors: Vec<char> = vec![];

        // If this position is a buffered character, no point in checking.
        if self.buffered_grid[position.row()][position.column()] == PaperRollGrid::BUFFER_CHAR {
            return neighbors;
        }

        for row in position.row() - 1..position.row() + 2 {
            for column in position.column() - 1..position.column() + 2 {
                if !(row == position.row() && column == position.column()) {
                    neighbors.push(self.buffered_grid[row][column])
                }
            }
        }

        neighbors
    }
}

impl TryFrom<FileReader> for PaperRollGrid {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let input_lines = reader
            .read_lines()?
            .map_while(Result::ok)
            .collect::<Vec<String>>();

        let mut grid = PaperRollGrid::new(input_lines[0].len());
        for grid_row in input_lines {
            grid.add_row(&grid_row);
        }

        Ok(grid)
    }
}

struct Position(usize, usize);

impl Position {
    fn row(&self) -> usize {
        self.0
    }

    fn column(&self) -> usize {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_FILE: &str = "../test-resources/aoc_25/day_04_sample.txt";

    fn create_buffered_row(size: usize) -> Vec<char> {
        vec![PaperRollGrid::BUFFER_CHAR; size]
    }

    #[test]
    fn test_new_paper_roll_grid() {
        let buffered_row = create_buffered_row(6);
        let initial_buffer: Vec<Vec<char>> = vec![buffered_row.clone(), buffered_row];
        let expected = PaperRollGrid {
            buffered_grid: initial_buffer,
        };

        assert_eq!(expected, PaperRollGrid::new(4));
    }

    #[test]
    fn test_add_row_to_paper_roll_grid() {
        let buffered_row = create_buffered_row(6);
        let added_row = vec![
            PaperRollGrid::BUFFER_CHAR,
            'b',
            'l',
            'a',
            'h',
            PaperRollGrid::BUFFER_CHAR,
        ];
        let expected = PaperRollGrid {
            buffered_grid: vec![buffered_row.clone(), added_row, buffered_row],
        };

        let mut paper_roll_grid = PaperRollGrid::new(4);
        paper_roll_grid.add_row("blah");

        assert_eq!(expected, paper_roll_grid);
    }

    #[test]
    fn test_neighbors() {
        // The grid:
        // = = = = = = =
        // = a b c d e =
        // = 1 2 3 4 5 =
        // = f g h i j =
        // = 6 7 8 9 0 =
        // = = = = = = =

        let mut grid = PaperRollGrid::new(5);
        grid.add_row("abcde");
        grid.add_row("12345");
        grid.add_row("fghij");
        grid.add_row("67890");

        let expected1 = vec!['b', 'c', 'd', '2', '4', 'g', 'h', 'i']; // position 2,3
        let expected2 = vec!['h', 'i', 'j', '8', '0', '=', '=', '=']; // position 4,4

        // Any buffer position should return no neighbors
        assert!(grid.neighbors(Position(0, 0)).is_empty());
        assert!(grid.neighbors(Position(5, 3)).is_empty());
        assert!(grid.neighbors(Position(2, 6)).is_empty());

        assert_eq!(expected1, grid.neighbors(Position(2, 3)));
        assert_eq!(expected2, grid.neighbors(Position(4, 4)));
    }

    #[test]
    fn test_removable_rolls() {
        let reader = FileReader::new(SAMPLE_FILE);
        let grid = PaperRollGrid::try_from(reader).unwrap();

        assert_eq!(13, grid.removable_rolls());
    }
}
