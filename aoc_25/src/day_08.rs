use disjoint::DisjointSetVec;
use itertools::Itertools;
use std::{
    cmp::{Ord, Ordering},
    collections::{BTreeMap, HashMap},
    hash::{Hash, Hasher},
};
use utils::FileReader;

static DAY_08_FILE: &str = "./resources/aoc_25/day_08.txt";

pub struct Day08Processor(String);

impl Day08Processor {
    fn new() -> Self {
        Self(String::from(DAY_08_FILE))
    }

    pub fn process(&self) {
        self.process_part1();
        self.process_part2();
    }

    fn process_part1(&self) {
        // println!("Running part 1 of day 8 using: {}", self.0);
        let reader = FileReader::new(&self.0);

        match PlaygroundDecoration::try_from(reader) {
            Ok(pd) => println!("AoC 25 Day 08 Part 1: {}", pd.calculate_circuits(1000)),
            Err(msg) => println!("AoC 25 Day 08 Part 1: Failed with message: {}", msg),
        }
    }

    fn process_part2(&self) {
        let reader = FileReader::new(&self.0);

        match PlaygroundDecoration::try_from(reader) {
            Ok(pd) => println!("AoC 25 Day 08 Part 2: {}", pd.join_all()),
            Err(msg) => println!("AoC 25 Day 08 Part 2: Failed with message: {}", msg),
        }
    }
}

impl Default for Day08Processor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Location(i64, i64, i64);

impl Location {
    fn x(&self) -> i64 {
        self.0
    }

    fn y(&self) -> i64 {
        self.1
    }

    fn z(&self) -> i64 {
        self.2
    }

    fn distance_from(&self, other: Location) -> Distance {
        let x = (self.x() - other.x()).pow(2);
        let y = (self.y() - other.y()).pow(2);
        let z = (self.z() - other.z()).pow(2);

        let total = x + y + z;
        Distance((total as f64).sqrt())
    }
}

impl TryFrom<String> for Location {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let vals = value
            .split(",")
            .map(|x| x.parse::<i64>())
            .collect::<Result<Vec<i64>, std::num::ParseIntError>>()?;

        Ok(Self(vals[0], vals[1], vals[2]))
    }
}

#[derive(Clone, Copy, Debug)]
struct Distance(f64);

impl Distance {
    fn value(&self) -> f64 {
        self.0
    }

    /// Normalize to a canonical bit representation so that:
    /// - All NaNs hash/compare equal to one canonical NaN.
    /// - -0.0 and 0.0 are treated the same.
    /// - Otherwise, use the raw IEEE-754 bits.
    fn norm_bits(self) -> u64 {
        let x = self.0;

        if x == 0.0 {
            // Collapse -0.0 to +0.0
            0.0f64.to_bits()
        } else if x.is_nan() {
            // Canonical quiet NaN (chosen representative)
            // 0x7ff8_0000_0000_0000 is a common canonical qNaN pattern.
            0x7ff8_0000_0000_0000u64
        } else {
            x.to_bits()
        }
    }
}

impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.norm_bits() == other.norm_bits()
    }
}

impl Eq for Distance {}

impl Hash for Distance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.norm_bits().hash(state);
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().total_cmp(&other.value())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct JunctionBox(Location);

impl JunctionBox {
    fn location(&self) -> Location {
        self.0
    }
}

impl TryFrom<String> for JunctionBox {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let location = Location::try_from(value)?;

        Ok(Self(location))
    }
}

#[derive(Debug, PartialEq)]
struct PlaygroundDecoration {
    junction_boxes: Vec<JunctionBox>,
}

impl PlaygroundDecoration {
    fn calculate_circuits(&self, num_pairs: usize) -> i64 {
        let mut distances: BTreeMap<Distance, (usize, usize)> = BTreeMap::new();
        let num_junction_boxes = self.junction_boxes.len();

        for i in 0..(num_junction_boxes - 1) {
            for j in (i + 1)..num_junction_boxes {
                let distance: Distance = self.junction_boxes[i]
                    .location()
                    .distance_from(self.junction_boxes[j].location());
                distances.insert(distance, (i, j));
            }
        }

        let mut boxes: DisjointSetVec<JunctionBox> =
            DisjointSetVec::from(self.junction_boxes.clone());

        let mut num_connected = 0;
        for (_, (i, j)) in distances.iter() {
            boxes.join(*i, *j);
            num_connected += 1;

            if num_connected >= num_pairs {
                break;
            }
        }

        let mut circuits: HashMap<usize, i64> = HashMap::new();
        for idx in 0..boxes.len() {
            let root = boxes.root_of(idx);
            *circuits.entry(root).or_insert(0) += 1;
        }

        circuits
            .iter()
            .sorted_by(|a, b| b.1.cmp(a.1))
            .take(3)
            .map(|(_, count)| count)
            .product()
    }

    fn join_all(&self) -> i64 {
        let mut distances: BTreeMap<Distance, (usize, usize)> = BTreeMap::new();
        let num_boxes = self.junction_boxes.len();

        for i in 0..(num_boxes - 1) {
            for j in (i + 1)..num_boxes {
                let distance = self.junction_boxes[i]
                    .location()
                    .distance_from(self.junction_boxes[j].location());
                distances.insert(distance, (i, j));
            }
        }

        let mut boxes: DisjointSetVec<JunctionBox> =
            DisjointSetVec::from(self.junction_boxes.clone());
        let mut last_join: [(usize, usize); 1] = [(0, 1)];

        for (_, (i, j)) in distances.iter() {
            if boxes.join(*i, *j) {
                last_join[0] = (*i, *j);
            }
        }

        self.junction_boxes[last_join[0].0].location().x()
            * self.junction_boxes[last_join[0].1].location().x()
    }
}

impl TryFrom<FileReader> for PlaygroundDecoration {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let input_lines = reader
            .read_lines()?
            .map_while(Result::ok)
            .collect::<Vec<String>>();

        let junction_boxes = input_lines
            .into_iter()
            .map(JunctionBox::try_from)
            .collect::<Result<Vec<JunctionBox>, crate::Error>>()?;

        Ok(Self { junction_boxes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_FILE: &str = "../test-resources/aoc_25/day_08_sample.txt";

    #[test]
    fn test_location_from_string() {
        assert_eq!(
            Location(162, 817, 812),
            Location::try_from(String::from("162,817,812")).unwrap()
        );
    }

    #[test]
    fn test_junction_box_from_string() {
        assert_eq!(
            JunctionBox(Location(162, 817, 812)),
            JunctionBox::try_from(String::from("162,817,812")).unwrap()
        )
    }

    #[test]
    fn test_playground_decoration_from_file_reader() {
        let reader = FileReader::new(SAMPLE_FILE);
        let playground_decoration = PlaygroundDecoration::try_from(reader).unwrap();

        assert_eq!(20, playground_decoration.junction_boxes.len());
        assert_eq!(
            JunctionBox(Location(162, 817, 812)),
            playground_decoration.junction_boxes[0]
        );
    }

    #[test]
    fn test_calculate_circuits() {
        let reader = FileReader::new(SAMPLE_FILE);
        let playground_decoration = PlaygroundDecoration::try_from(reader).unwrap();

        assert_eq!(40, playground_decoration.calculate_circuits(10));
    }

    #[test]
    fn test_join_all() {
        let reader = FileReader::new(SAMPLE_FILE);
        let playground_decoration = PlaygroundDecoration::try_from(reader).unwrap();

        assert_eq!(25272, playground_decoration.join_all());
    }
}
