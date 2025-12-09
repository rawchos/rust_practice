use once_cell::sync::Lazy;
use regex::Regex;
use std::ops::Range;
use utils::FileReader;

static DAY_06_FILE: &str = "./resources/aoc_25/day_06.txt";
static OPERATIONS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\\*\\+] *").expect("Expected a valid regex for operations"));

pub struct Day06Processor(String);

impl Day06Processor {
    fn new() -> Self {
        Self(String::from(DAY_06_FILE))
    }

    pub fn process(&self) {
        self.process_part1();
        self.process_part2();
    }

    fn process_part1(&self) {
        let reader = FileReader::new(&self.0);

        match PartOneValue::try_from(reader) {
            Ok(p1_value) => println!("AoC 25 Day 06 Part 1: {}", p1_value.get()),
            Err(msg) => println!("AoC 25 Day 06 Part 1: Failed with message: {}", msg),
        }
    }

    fn process_part2(&self) {
        let reader = FileReader::new(&self.0);

        match PartTwoValue::try_from(reader) {
            Ok(p2_value) => println!("AoC 25 Day 06 Part 2: {}", p2_value.get()),
            Err(msg) => println!("AoC 25 Day 06 Part 2: Failed with message: {}", msg),
        }
    }
}

impl Default for Day06Processor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        if value.eq("+") {
            Self::Add
        } else {
            Self::Multiply
        }
    }
}

#[derive(Debug, PartialEq)]
struct Day06Input {
    worksheet_lines: Vec<String>,
    operations: String,
}

impl TryFrom<FileReader> for Day06Input {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let mut worksheet_lines: Vec<String> = reader
            .read_lines()?
            .map_while(Result::ok)
            .collect::<Vec<String>>();

        let Some(operations) = worksheet_lines.pop() else {
            return Err(Self::Error::InvalidInput);
        };

        Ok(Self {
            worksheet_lines,
            operations,
        })
    }
}

#[derive(Debug, PartialEq)]
struct PartOneAccumulator {
    column_totals: Vec<i64>,
}

impl PartOneAccumulator {
    // Initializes the totals for part one according to the operations provided
    fn init(operations: &Vec<Operation>) -> Self {
        let mut column_totals: Vec<i64> = vec![];

        for operation in operations {
            match operation {
                Operation::Add => column_totals.push(0),
                Operation::Multiply => column_totals.push(1),
            }
        }

        Self { column_totals }
    }

    fn add_value(&mut self, value: i64, column: usize) {
        self.column_totals[column] += value;
    }

    fn multiply_value(&mut self, value: i64, column: usize) {
        self.column_totals[column] *= value;
    }

    fn total(&self) -> i64 {
        self.column_totals.iter().sum()
    }
}

#[derive(Debug, PartialEq)]
struct PartOneValue(i64);

impl PartOneValue {
    fn get(&self) -> i64 {
        self.0
    }
}

impl TryFrom<Day06Input> for PartOneValue {
    type Error = crate::Error;

    fn try_from(value: Day06Input) -> Result<Self, Self::Error> {
        let operations: Vec<Operation> = value
            .operations
            .split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(Operation::from)
            .collect();

        let mut accumulator = PartOneAccumulator::init(&operations);

        for idx in 0..value.worksheet_lines.len() {
            let vals = value.worksheet_lines[idx]
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<i64>())
                .collect::<Result<Vec<i64>, std::num::ParseIntError>>()?;

            for (column, operation) in operations.iter().enumerate() {
                let value = vals[column];
                match operation {
                    Operation::Add => accumulator.add_value(value, column),
                    Operation::Multiply => accumulator.multiply_value(value, column),
                }
            }
        }

        Ok(Self(accumulator.total()))
    }
}

impl TryFrom<FileReader> for PartOneValue {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let input = Day06Input::try_from(reader)?;
        Self::try_from(input)
    }
}

#[derive(Debug, PartialEq)]
struct RangedOperation {
    operation: Operation,
    range: Range<usize>,
}

#[derive(Debug, PartialEq)]
struct PartTwoValue(i64);

impl PartTwoValue {
    fn get(&self) -> i64 {
        self.0
    }
}

impl TryFrom<Day06Input> for PartTwoValue {
    type Error = crate::Error;

    fn try_from(value: Day06Input) -> Result<Self, Self::Error> {
        let worksheet_size = value.worksheet_lines[0].len();

        // Create the ranged operations
        let matches = OPERATIONS_RE
            .find_iter(&value.operations)
            .collect::<Vec<_>>();
        let num_matches = matches.len();

        let mut operations: Vec<RangedOperation> = vec![];
        for (idx, op_match) in matches.iter().enumerate() {
            let end = if idx == num_matches - 1 {
                worksheet_size
            } else {
                op_match.end() - 1
            };

            let range = op_match.start()..end;
            let operation = Operation::from(op_match.as_str().trim());

            operations.push(RangedOperation { operation, range })
        }

        let mut columns: Vec<i64> = vec![];
        for operation in operations {
            let mut nums: Vec<i64> = vec![];

            for idx in operation.range {
                let mut digits: Vec<char> = vec![];
                for line in value.worksheet_lines.clone() {
                    if let Some(this_digit) = line.chars().nth(idx) {
                        if this_digit != ' ' {
                            digits.push(this_digit)
                        }
                    }
                }
                nums.push(digits.into_iter().collect::<String>().parse::<i64>()?);
            }

            if let Some(this_column) = match operation.operation {
                Operation::Add => Some(nums.into_iter().sum()),
                Operation::Multiply => nums.into_iter().reduce(|a, b| a * b),
            } {
                columns.push(this_column)
            }
        }

        Ok(PartTwoValue(columns.into_iter().sum()))
    }
}

impl TryFrom<FileReader> for PartTwoValue {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let input = Day06Input::try_from(reader)?;
        Self::try_from(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_FILE: &str = "../test-resources/aoc_25/day_06_sample.txt";

    #[test]
    fn test_input_from_reader() {
        let worksheet_lines = vec![
            String::from("123 328  51 64 "),
            String::from(" 45 64  387 23 "),
            String::from("  6 98  215 314"),
        ];
        let operations = String::from("*   +   *   + ");

        let expected = Day06Input {
            worksheet_lines,
            operations,
        };
        let reader = FileReader::new(SAMPLE_FILE);

        assert_eq!(expected, Day06Input::try_from(reader).unwrap());
    }

    #[test]
    fn test_operation_from_str() {
        assert_eq!(Operation::Add, Operation::from("+"));
        assert_eq!(Operation::Multiply, Operation::from("*"));
    }

    #[test]
    fn test_part_one_value_from_input() {
        let reader = FileReader::new(SAMPLE_FILE);
        let input = Day06Input::try_from(reader).unwrap();

        assert_eq!(
            PartOneValue(4277556),
            PartOneValue::try_from(input).unwrap()
        );
    }

    #[test]
    fn test_part_one_value_from_reader() {
        let reader = FileReader::new(SAMPLE_FILE);

        assert_eq!(
            PartOneValue(4277556),
            PartOneValue::try_from(reader).unwrap()
        );
    }

    #[test]
    fn test_part_two_value_from_input() {
        let reader = FileReader::new(SAMPLE_FILE);
        let input = Day06Input::try_from(reader).unwrap();

        assert_eq!(
            PartTwoValue(3263827),
            PartTwoValue::try_from(input).unwrap()
        );
    }

    #[test]
    fn test_part_two_value_from_reader() {
        let reader = FileReader::new(SAMPLE_FILE);

        assert_eq!(
            PartTwoValue(3263827),
            PartTwoValue::try_from(reader).unwrap()
        );
    }
}
