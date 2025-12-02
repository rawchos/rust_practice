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
        PartTwoProcessor::new(self.0.as_str()).process();
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

        match Password::try_from(file_reader) {
            Ok(password) => println!("AoC 25 Day 01 Part 1: {}", password.landed_on_zero()),
            Err(msg) => println!("AoC 25 Day 01 Part 1: Failed with this message: {}", msg),
        }
    }
}

struct PartTwoProcessor(String);

impl PartTwoProcessor {
    fn new(s: &str) -> Self {
        Self(s.to_string())
    }

    fn process(&self) {
        let file_reader = FileReader::new(&self.0);

        match Password::try_from(file_reader) {
            Ok(password) => println!(
                "AoC 25 Day 01 Part 2: {}",
                password.landed_on_zero() + password.zero_clicks()
            ),
            Err(msg) => println!("AoC 25 Day 01 Part 2: Failed with this message: {}", msg),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Rotation {
    direction: Direction,
    distance: i32,
    full_rotations: i32,
}

impl TryFrom<String> for Rotation {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = regex::Regex::new(r"(?<direction>[LR])(?<distance>\d+)")?;

        let Some(rotation_data) = re.captures(&value) else {
            return Err(Self::Error::InvalidInput);
        };

        let full_distance: i32 = rotation_data["distance"].parse::<i32>()?;
        let direction = if &rotation_data["direction"] == "L" {
            Direction::Left
        } else {
            Direction::Right
        };

        Ok(Rotation {
            direction,
            distance: full_distance % 100,
            full_rotations: full_distance / 100,
        })
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
    zero_clicks: i32,
}

impl SafeDial {
    fn new() -> Self {
        Self {
            current_position: 50,
            zero_counter: 0,
            zero_clicks: 0,
        }
    }

    fn apply_rotation(&mut self, rotation: &Rotation) {
        let previous_position = self.current_position;

        let new_position = match rotation.direction {
            Direction::Left => self.current_position - rotation.distance,
            Direction::Right => self.current_position + rotation.distance,
        };

        match new_position {
            n if n < 0 => {
                self.current_position = 100 + n;
                if self.current_position != 0 && previous_position != 0 {
                    self.zero_clicks += 1;
                }
            }
            n if n > 99 => {
                self.current_position = n - 100;
                if self.current_position != 0 && previous_position != 0 {
                    self.zero_clicks += 1;
                }
            }
            _ => self.current_position = new_position,
        }

        self.zero_clicks += rotation.full_rotations;

        if self.current_position == 0 {
            self.zero_counter += 1
        }
    }
}

struct Password(i32, i32);

impl Password {
    fn landed_on_zero(&self) -> i32 {
        self.0
    }

    fn zero_clicks(&self) -> i32 {
        self.1
    }
}

impl From<RotationList> for Password {
    fn from(value: RotationList) -> Self {
        let safe_dial: SafeDial = value.iter().fold(SafeDial::new(), |mut dial, rotation| {
            dial.apply_rotation(rotation);
            dial
        });

        Password(safe_dial.zero_counter, safe_dial.zero_clicks)
    }
}

impl TryFrom<FileReader> for Password {
    type Error = crate::Error;

    fn try_from(value: FileReader) -> Result<Self, Self::Error> {
        let rotations = RotationList::try_from(value)?;

        Ok(Password::from(rotations))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_FILE: &str = "../test-resources/aoc_25/day_01_sample.txt";

    #[test]
    fn left_rotation_from_string() {
        let expected = Rotation {
            direction: Direction::Left,
            distance: 32,
            full_rotations: 0,
        };

        assert_eq!(expected, Rotation::try_from(String::from("L32")).unwrap())
    }

    #[test]
    fn right_rotation_from_string() {
        let expected = Rotation {
            direction: Direction::Right,
            distance: 77,
            full_rotations: 0,
        };

        assert_eq!(expected, Rotation::try_from(String::from("R77")).unwrap())
    }

    #[test]
    fn rotation_list_from_reader() {
        let rotations = vec![
            Rotation {
                direction: Direction::Left,
                distance: 68,
                full_rotations: 0,
            },
            Rotation {
                direction: Direction::Left,
                distance: 30,
                full_rotations: 0,
            },
            Rotation {
                direction: Direction::Right,
                distance: 48,
                full_rotations: 0,
            },
            Rotation {
                direction: Direction::Left,
                distance: 5,
                full_rotations: 0,
            },
            Rotation {
                direction: Direction::Right,
                distance: 60,
                full_rotations: 0,
            },
            Rotation {
                direction: Direction::Left,
                distance: 55,
                full_rotations: 0,
            },
            Rotation {
                direction: Direction::Left,
                distance: 1,
                full_rotations: 0,
            },
            Rotation {
                direction: Direction::Left,
                distance: 99,
                full_rotations: 0,
            },
            Rotation {
                direction: Direction::Right,
                distance: 14,
                full_rotations: 0,
            },
            Rotation {
                direction: Direction::Left,
                distance: 82,
                full_rotations: 0,
            },
        ];
        let expected = RotationList(rotations);
        let reader = FileReader::new(SAMPLE_FILE);

        assert_eq!(expected, RotationList::try_from(reader).unwrap())
    }

    #[test]
    fn password_from_rotation_list() {
        let rotations = RotationList::try_from(FileReader::new(SAMPLE_FILE)).unwrap();
        let password = Password::from(rotations);

        assert_eq!(3, password.landed_on_zero());
        assert_eq!(6, password.landed_on_zero() + password.zero_clicks())
    }

    #[test]
    fn password_from_filereader() {
        let password = Password::try_from(FileReader::new(SAMPLE_FILE)).unwrap();

        assert_eq!(3, password.landed_on_zero());
        assert_eq!(6, password.landed_on_zero() + password.zero_clicks())
    }
}
