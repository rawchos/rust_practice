use regex;

pub struct Day01Processor(String);

impl Day01Processor {
    fn new() -> Self {
        // Self(String::from("./resources/day_01.txt"))
        Self(String::from("./resources/aoc_24/sample_data.txt"))
    }

    pub fn process(&self) {
        println!("Solving problems for day 01 using: {}", self.0)
    }
}

impl Default for Day01Processor {
    fn default() -> Self {
        Self::new()
    }
}

// #[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct InputNode {
    pub left: u16,
    pub right: u16,
}

impl TryFrom<String> for InputNode {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = regex::Regex::new(r"(?<left>\d+) +(?<right>\d+)$")?;

        let Some(input_data) = re.captures(&value) else {
            return Err(Self::Error::InvalidInput);
        };

        let input_node = InputNode {
            left: input_data["left"].parse::<u16>()?,
            right: input_data["right"].parse::<u16>()?,
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

    #[test]
    fn input_node_from_string() {
        let expected = InputNode { left: 3, right: 4 };

        assert_eq!(
            expected,
            InputNode::try_from(String::from("3   4")).unwrap()
        )
    }
}
