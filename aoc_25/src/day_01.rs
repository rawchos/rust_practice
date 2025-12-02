use std::ops::Deref;
use utils::FileReader;

static DAY_01_FILE: &str = "./resources/aoc_25/day_01.txt";

pub struct Day01Processor(String);

impl Day01Processor {
    fn new() -> Self {
        Self(String::from(DAY_01_FILE))
    }

    pub fn process(&self) {
        PartOneProcessor::new(self.0.as_str()).process();
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

        match BasicPassword::try_from(file_reader) {
            Ok(password) => println!("AoC 25 Day 01 Part 1: {}", password.get()),
            Err(msg) => println!("AoC 25 Day 01 Part 1: Failed with this message: {}", msg),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl TryFrom<String> for Rotation {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = regex::Regex::new(r"(?<direction>[LR])(?<distance>\d+)")?;

        let Some(rotation_data) = re.captures(&value) else {
            return Err(Self::Error::InvalidInput);
        };

        let distance: i32 = rotation_data["distance"].parse::<i32>()?;
        let functional_distance = distance % 100;
        let rotation = if &rotation_data["direction"] == "L" {
            Self::Left(functional_distance)
        } else {
            Self::Right(functional_distance)
        };

        Ok(rotation)
    }
}

#[derive(Debug, PartialEq)]
struct RotationList(Vec<Rotation>);

impl Deref for RotationList {
    type Target = Vec<Rotation>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<FileReader> for RotationList {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let rotations = reader
            .read_lines()?
            .map_while(Result::ok)
            .map(Rotation::try_from)
            .collect::<Result<Vec<Rotation>, crate::Error>>()?;

        Ok(Self(rotations))
    }
}

struct SafeDial {
    current_position: i32,
    zero_counter: i32,
}

impl SafeDial {
    fn new() -> Self {
        Self {
            current_position: 50,
            zero_counter: 0,
        }
    }

    fn apply_rotation(&mut self, rotation: &Rotation) {
        let new_position = match rotation {
            Rotation::Left(distance) => self.current_position - distance,
            Rotation::Right(distance) => self.current_position + distance,
        };

        match new_position {
            n if n < 0 => self.current_position = 100 + n,
            n if n > 99 => self.current_position = n - 100,
            _ => self.current_position = new_position,
        }

        if self.current_position == 0 {
            self.zero_counter += 1
        }
    }
}

struct BasicPassword(i32);

impl BasicPassword {
    fn get(&self) -> i32 {
        self.0
    }
}

impl From<RotationList> for BasicPassword {
    fn from(value: RotationList) -> Self {
        let safe_dial: SafeDial = value.iter().fold(SafeDial::new(), |mut dial, rotation| {
            dial.apply_rotation(rotation);
            dial
        });

        BasicPassword(safe_dial.zero_counter)
    }
}

impl TryFrom<FileReader> for BasicPassword {
    type Error = crate::Error;

    fn try_from(value: FileReader) -> Result<Self, Self::Error> {
        let rotations = RotationList::try_from(value)?;

        Ok(BasicPassword::from(rotations))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_FILE: &str = "../test-resources/aoc_25/day_01_sample.txt";

    #[test]
    fn left_rotation_from_string() {
        let expected = Rotation::Left(32);

        assert_eq!(expected, Rotation::try_from(String::from("L32")).unwrap())
    }

    #[test]
    fn right_rotation_from_string() {
        let expected = Rotation::Right(77);

        assert_eq!(expected, Rotation::try_from(String::from("R77")).unwrap())
    }

    #[test]
    fn rotation_list_from_reader() {
        let rotations = vec![
            Rotation::Left(68),
            Rotation::Left(30),
            Rotation::Right(48),
            Rotation::Left(5),
            Rotation::Right(60),
            Rotation::Left(55),
            Rotation::Left(1),
            Rotation::Left(99),
            Rotation::Right(14),
            Rotation::Left(82),
        ];
        let expected = RotationList(rotations);
        let reader = FileReader::new(SAMPLE_FILE);

        assert_eq!(expected, RotationList::try_from(reader).unwrap())
    }

    #[test]
    fn basic_password_from_rotation_list() {
        let rotations = RotationList::try_from(FileReader::new(SAMPLE_FILE)).unwrap();
        let password = BasicPassword::from(rotations);

        assert_eq!(3, password.get())
    }

    #[test]
    fn basic_password_from_filereader() {
        let password = BasicPassword::try_from(FileReader::new(SAMPLE_FILE)).unwrap();

        assert_eq!(3, password.get())
    }
}
