use once_cell::sync::Lazy;
use regex::Regex;
use utils::FileReader;

static DAY_02_FILE: &str = "./resources/aoc_25/day_02.txt";
static PRODUCT_RANGE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<start>\d+)-(?<end>\d+)$").expect("Expected a valid regex"));

pub struct Day02Processor(String);

impl Day02Processor {
    fn new() -> Self {
        Self(String::from(DAY_02_FILE))
    }

    pub fn process(&self) {
        // println!("Processing [{}] for day 02", self.0)
        PartOneProcessor::new(self.0.as_str()).process()
    }
}

impl Default for Day02Processor {
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
            Ok(p1_value) => println!("AoC 25 Day 02 Part 1: {}", p1_value.get()),
            Err(msg) => println!("AoC 25 Day 02 Part 1: Failed with message: {}", msg),
        }
    }
}

#[derive(Debug, PartialEq)]
struct ProductRange {
    start_id: i64,
    end_id: i64,
}

impl ProductRange {
    #[allow(dead_code)]
    fn invalid_ids(&self) -> Vec<i64> {
        let mut invalids: Vec<i64> = vec![];

        for prod_id in self.start_id..self.end_id + 1 {
            if ProductRange::invalid_product_id(prod_id) {
                invalids.push(prod_id)
            }
        }

        invalids
    }

    #[allow(dead_code)]
    fn invalid_product_id(prod_id: i64) -> bool {
        let prod_id_str = prod_id.to_string();
        let length = prod_id_str.len();

        // If it's odd, it can't be the same number twice
        if length % 2 > 0 {
            return false;
        }

        let midpoint = length / 2;
        if prod_id_str[0..midpoint] == prod_id_str[midpoint..length] {
            return true;
        }

        false
    }
}

impl TryFrom<String> for ProductRange {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let Some(product_range) = PRODUCT_RANGE_RE.captures(&value) else {
            return Err(Self::Error::InvalidInput);
        };

        let start_id = product_range["start"].parse::<i64>()?;
        let end_id = product_range["end"].parse::<i64>()?;

        Ok(ProductRange { start_id, end_id })
    }
}

#[derive(Debug, PartialEq)]
struct PartOneValue(i64);

impl PartOneValue {
    #[allow(dead_code)]
    fn get(&self) -> i64 {
        self.0
    }
}

impl TryFrom<FileReader> for PartOneValue {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let mut input_lines = reader.read_lines()?;
        let Some(raw_input) = input_lines.next() else {
            return Ok(PartOneValue(0));
        };

        let first_line = raw_input?;
        let lines: Vec<&str> = first_line.split(',').collect();

        let product_ranges = lines
            .iter()
            .map(|s| ProductRange::try_from(s.to_string()))
            .collect::<Result<Vec<ProductRange>, crate::Error>>()?;
        let total = product_ranges
            .iter()
            .map(|pr| pr.invalid_ids())
            .flatten()
            .sum();

        Ok(PartOneValue(total))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_FILE: &str = "../test-resources/aoc_25/day_02_sample.txt";

    #[test]
    fn test_setup() {
        assert_eq!(1, 1)
    }

    #[test]
    fn product_range_from_string() {
        let expected = ProductRange {
            start_id: 11,
            end_id: 22,
        };

        assert_eq!(
            expected,
            ProductRange::try_from(String::from("11-22")).unwrap()
        )
    }

    #[test]
    fn invalid_product_id() {
        assert!(ProductRange::invalid_product_id(11));
        assert!(!ProductRange::invalid_product_id(12));
        assert!(!ProductRange::invalid_product_id(111));
        assert!(ProductRange::invalid_product_id(1111))
    }

    #[test]
    fn invalid_ids() {
        let product_range = ProductRange {
            start_id: 11,
            end_id: 22,
        };
        let expected = vec![11, 22];

        let product_range2 = ProductRange {
            start_id: 95,
            end_id: 112,
        };
        let expected2 = vec![99];

        assert_eq!(expected, product_range.invalid_ids());
        assert_eq!(expected2, product_range2.invalid_ids());
    }

    #[test]
    fn sum_invalid_product_ids() {
        let reader = FileReader::new(SAMPLE_FILE);

        assert_eq!(1227775554, PartOneValue::try_from(reader).unwrap().get())
    }
}
