use utils::FileReader;

static DAY_06_FILE: &str = "./resources/aoc_25/day_06.txt";

pub struct Day06Processor(String);

impl Day06Processor {
    fn new() -> Self {
        Self(String::from(DAY_06_FILE))
    }

    pub fn process(&self) {
        self.process_part1();
    }

    fn process_part1(&self) {
        let reader = FileReader::new(&self.0);

        match PartOneValue::try_from(reader) {
            Ok(p1_value) => println!("AoC 25 Day 06 Part 1: {}", p1_value.get()),
            Err(msg) => println!("AoC 25 Day 06 Part 1: Failed with message: {}", msg),
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
}
