use once_cell::sync::Lazy;
use regex::Regex;
use std::ops::RangeInclusive;
use utils::FileReader;

static DAY_05_FILE: &str = "./resources/aoc_25/day_05.txt";
static FRESH_ID_RANGE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?<start>\d+)-(?<end>\d+)$").expect("Expected a valid regex for fresh id ranges")
});
static INGREDIENT_ID_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<id>\d+)$").expect("Expected a valid regex for ingredient id"));

pub struct Day05Processor(String);

impl Day05Processor {
    fn new() -> Self {
        Self(String::from(DAY_05_FILE))
    }

    pub fn process(&self) {
        self.process_part1();
    }

    fn process_part1(&self) {
        let file_reader = FileReader::new(&self.0);

        match PartOneValue::try_from(file_reader) {
            Ok(p1_value) => println!("AoC Day 05 Part 1: {}", p1_value.get()),
            Err(msg) => println!("AoC Day 05 Part 2: Failed with message: {}", msg),
        }
    }
}

impl Default for Day05Processor {
    fn default() -> Self {
        Self::new()
    }
}

struct PartOneValue(i64);

impl PartOneValue {
    fn get(&self) -> i64 {
        self.0
    }
}

#[derive(Debug, PartialEq)]
struct FreshIdRanges(Vec<RangeInclusive<i64>>);

impl FreshIdRanges {
    fn new() -> Self {
        let empty_ranges: Vec<RangeInclusive<i64>> = vec![];
        Self(empty_ranges)
    }

    fn ranges(&self) -> &Vec<RangeInclusive<i64>> {
        &self.0
    }

    fn is_fresh_id_range(possible_range: String) -> bool {
        FRESH_ID_RANGE_RE.is_match(&possible_range)
    }

    fn add_range(&mut self, range: String) {
        let Some(range_data) = FRESH_ID_RANGE_RE.captures(&range) else {
            return;
        };

        let Ok(start) = range_data["start"].parse::<i64>() else {
            return;
        };

        let Ok(end) = range_data["end"].parse::<i64>() else {
            return;
        };

        self.0.push(start..=end)
    }

    // Checks through all the fresh id ranges to see if this ingredient id
    // is contained.
    fn is_fresh_ingredient(&self, ingredient_id: i64) -> bool {
        for range in self.ranges().iter() {
            if range.contains(&ingredient_id) {
                return true;
            }
        }

        false
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct IngredientIDs(Vec<i64>);

#[allow(dead_code)]
impl IngredientIDs {
    #[allow(dead_code)]
    fn new() -> Self {
        Self(vec![])
    }

    #[allow(dead_code)]
    fn add_ingredient(&mut self, ingredient_id: String) {
        if let Ok(id) = ingredient_id.parse::<i64>() {
            self.0.push(id);
        }
    }

    #[allow(dead_code)]
    fn is_ingredient_id(possible_ingredient_id: String) -> bool {
        INGREDIENT_ID_RE.is_match(&possible_ingredient_id)
    }
}

impl TryFrom<FileReader> for PartOneValue {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let input_lines = reader
            .read_lines()?
            .map_while(Result::ok)
            .collect::<Vec<String>>();

        let mut fresh_ranges = FreshIdRanges::new();
        let mut fresh_ingredients: i64 = 0;
        // let mut ingredients = IngredientIDs::new();

        // Maybe switch these to references
        for input in input_lines.into_iter() {
            if FreshIdRanges::is_fresh_id_range(input.clone()) {
                fresh_ranges.add_range(input.clone());
            } else if IngredientIDs::is_ingredient_id(input.clone()) {
                // ingredients.add_ingredient(input);
                if let Ok(ingredient) = input.parse::<i64>() {
                    if fresh_ranges.is_fresh_ingredient(ingredient) {
                        // println!("Fresh Ingredient: {}", input);
                        fresh_ingredients += 1;
                    }
                }
            }
        }

        Ok(PartOneValue(fresh_ingredients))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_FILE: &str = "../test-resources/aoc_25/day_05_sample.txt";

    #[test]
    fn test_is_fresh_id_range_match() {
        assert!(FreshIdRanges::is_fresh_id_range(String::from("15-30")));
        assert!(!FreshIdRanges::is_fresh_id_range(String::from("77")));
        assert!(!FreshIdRanges::is_fresh_id_range(String::from("")));
    }

    #[test]
    fn test_add_range() {
        let expected = FreshIdRanges(vec![5..=30]);
        let mut fresh_id_ranges = FreshIdRanges::new();
        fresh_id_ranges.add_range(String::from("5-30"));

        assert_eq!(expected, fresh_id_ranges);
    }

    #[test]
    fn test_is_ingredient_id() {
        assert!(IngredientIDs::is_ingredient_id(String::from("72")));
        assert!(!IngredientIDs::is_ingredient_id(String::from("")));
        assert!(!IngredientIDs::is_ingredient_id(String::from("5-30")));
    }

    #[test]
    fn test_is_fresh_ingredient() {
        let mut fresh_id_ranges = FreshIdRanges::new();
        fresh_id_ranges.add_range(String::from("100-125"));
        fresh_id_ranges.add_range(String::from("150-175"));

        // Fresh Ingredients
        assert!(fresh_id_ranges.is_fresh_ingredient(100));
        assert!(fresh_id_ranges.is_fresh_ingredient(115));
        assert!(fresh_id_ranges.is_fresh_ingredient(125));
        assert!(fresh_id_ranges.is_fresh_ingredient(150));
        assert!(fresh_id_ranges.is_fresh_ingredient(165));
        assert!(fresh_id_ranges.is_fresh_ingredient(175));

        // Spoiled Ingredients
        assert!(!fresh_id_ranges.is_fresh_ingredient(99));
        assert!(!fresh_id_ranges.is_fresh_ingredient(126));
        assert!(!fresh_id_ranges.is_fresh_ingredient(149));
        assert!(!fresh_id_ranges.is_fresh_ingredient(176));
        assert!(!fresh_id_ranges.is_fresh_ingredient(200));
    }

    #[test]
    fn test_part_one_value_from_reader() {
        let reader = FileReader::new(SAMPLE_FILE);

        assert_eq!(3, PartOneValue::try_from(reader).unwrap().get());
    }
}
