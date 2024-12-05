use itertools::Itertools;
use regex;
use std::collections::HashMap;
use utils::FileReader;

pub struct Day01Processor(String);

impl Day01Processor {
    fn new() -> Self {
        Self(String::from("./resources/aoc_24/day_01.txt"))
    }

    pub fn process(&self) {
        PartOneProcessor::new(self.0.as_str()).process();
        PartTwoProcessor::new(self.0.as_str()).process()
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
            Err(msg) => println!("AoC 24 Day 01 Part 1: Failed with this message: {}", msg),
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

struct PartTwoProcessor(String);

impl PartTwoProcessor {
    fn new(s: &str) -> Self {
        Self(s.to_string())
    }

    fn process(&self) {
        let file_reader = FileReader::new(&self.0);

        match PartTwoValue::try_from(file_reader) {
            Ok(p2_value) => println!("AoC 24 Day 01 Part 2: {}", p2_value.value()),
            Err(msg) => println!("AoC 24 Day 01 Part 2: Failed with this message: {}", msg),
        }
    }
}

impl TryFrom<FileReader> for PartTwoValue {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let input_nodes = reader
            .read_lines()?
            .map_while(Result::ok)
            .map(InputNode::try_from)
            .collect::<Result<Vec<InputNode>, crate::Error>>()?;

        SimilarityCalculator::new(input_nodes).calculate()
    }
}

#[derive(Debug, PartialEq)]
struct PartTwoValue(i32);

impl PartTwoValue {
    fn value(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, PartialEq)]
struct SimilarityCalculator {
    left_values: Vec<i32>,
    right_groups: HashMap<i32, Vec<i32>>,
}

impl SimilarityCalculator {
    fn new(input_nodes: Vec<InputNode>) -> Self {
        let mut left_nodes = vec![];
        let mut right_nodes = vec![];

        for input_node in input_nodes.iter() {
            left_nodes.push(input_node.left);
            right_nodes.push(input_node.right);
        }

        Self {
            left_values: left_nodes,
            right_groups: right_nodes.into_iter().into_group_map_by(|x| *x),
        }
    }

    fn calculate(&self) -> Result<PartTwoValue, crate::Error> {
        let mut total = 0;

        for value in self.left_values.iter() {
            let score = match self.right_groups.get(value) {
                Some(matches) => matches.len().try_into()?,
                None => 0i32,
            };

            total += value * score;
        }

        Ok(PartTwoValue(total))
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

    #[test]
    fn similarity_calculation() {
        let input_nodes = vec![
            build_input_node(3, 4),
            build_input_node(4, 3),
            build_input_node(2, 5),
            build_input_node(1, 3),
            build_input_node(3, 9),
            build_input_node(3, 3),
        ];
        let similarity_calculator = SimilarityCalculator::new(input_nodes);

        assert_eq!(PartTwoValue(31), similarity_calculator.calculate().unwrap())
    }
}
