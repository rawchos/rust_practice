use regex;
use utils::FileReader;

pub struct Day01Processor(String);

impl Day01Processor {
    fn new() -> Self {
        Self(String::from("./resources/aoc_24/day_01.txt"))
        // Self(String::from("./resources/aoc_24/sample_data.txt"))
    }

    pub fn process(&self) {
        PartOneProcessor::new(self.0.as_str()).process()
    }
}

impl Default for Day01Processor {
    fn default() -> Self {
        Self::new()
    }
}

struct PartOneProcessor(String);

impl PartOneProcessor {
    fn new(s: &str) -> Self {
        Self(s.to_string())
    }

    fn process(&self) {
        let file_reader = FileReader::new(&self.0);

        match PartOneValue::try_from(file_reader) {
            Ok(p1_value) => println!("AoC 24 Day 01 Part 1: {}", p1_value.value()),
            Err(msg) => println!("AoC 24 Day 01 Part 2: Failed with this message: {}", msg),
        }
    }
}

#[derive(Debug, PartialEq)]
struct PartOneValue(i32);

impl PartOneValue {
    fn value(&self) -> i32 {
        self.0
    }
}

impl TryFrom<FileReader> for PartOneValue {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let input_nodes = reader
            .read_lines()?
            .map_while(Result::ok)
            .map(InputNode::try_from)
            .collect::<Result<Vec<InputNode>, crate::Error>>()?;

        Ok(DistanceCalculator::new(input_nodes).calculate())
    }
}

#[derive(Debug, PartialEq)]
struct DistanceCalculator {
    nodes: Vec<InputNode>,
}

impl DistanceCalculator {
    fn new(nodes: Vec<InputNode>) -> Self {
        Self { nodes }
    }

    fn calculate(&self) -> PartOneValue {
        let mut left = vec![];
        let mut right = vec![];

        for input_node in self.nodes.iter() {
            left.push(input_node.left);
            right.push(input_node.right);
        }

        left.sort();
        right.sort();

        let mut distance = 0i32;
        for idx in 0..left.len() {
            distance += (left[idx] - right[idx]).abs()
        }

        PartOneValue(distance)
    }
}

#[derive(Debug, PartialEq)]
struct InputNode {
    pub left: i32,
    pub right: i32,
}

impl TryFrom<String> for InputNode {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = regex::Regex::new(r"(?<left>\d+) +(?<right>\d+)$")?;

        let Some(input_data) = re.captures(&value) else {
            return Err(Self::Error::InvalidInput);
        };

        let input_node = InputNode {
            left: input_data["left"].parse::<i32>()?,
            right: input_data["right"].parse::<i32>()?,
        };

        Ok(input_node)
    }
}

// Sample Data:
//  3   4
//  4   3
//  2   5
//  1   3
//  3   9
//  3   3

#[cfg(test)]
mod tests {
    use super::*;

    fn build_input_node(left: i32, right: i32) -> InputNode {
        InputNode { left, right }
    }

    #[test]
    fn input_node_from_string() {
        let expected = InputNode { left: 3, right: 4 };

        assert_eq!(
            expected,
            InputNode::try_from(String::from("3   4")).unwrap()
        )
    }

    #[test]
    fn distance_calculation() {
        let distance_calculator = DistanceCalculator {
            nodes: vec![
                build_input_node(3, 4),
                build_input_node(4, 3),
                build_input_node(2, 5),
                build_input_node(1, 3),
                build_input_node(3, 9),
                build_input_node(3, 3),
            ],
        };

        assert_eq!(PartOneValue(11), distance_calculator.calculate())
    }
}
