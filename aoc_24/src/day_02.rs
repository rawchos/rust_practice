static SAFE_LEVEL_DIFF: [i16; 3] = [1, 2, 3];

pub struct Day02Processor(String);

impl Day02Processor {
    fn new() -> Self {
        Self(String::from("./resources/aoc_24/sample_data.txt"))
        // Self(String::from("./resources/aoc_24/day_02.txt"))
    }

    pub fn process(&self) {
        println!("Running Day02 Stuff for: {}", self.0)
    }
}

impl Default for Day02Processor {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
struct Level(i16);

#[derive(Debug, PartialEq)]
struct ReactorReport {
    levels: Vec<Level>,
}

impl ReactorReport {
    // TODO: Need a way to check increasing, decreasing, unsafe change
    #[allow(dead_code)]
    fn safe(&self) -> bool {
        // let mut increasing = false;
        // let mut decreasing = false;
        let mut prev = self.levels[0];

        for idx in 1..self.levels.len() {
            let current = self.levels[idx];
            let diff = current.0 - prev.0;

            if !SAFE_LEVEL_DIFF.contains(&diff.abs()) {
                return false;
            } else {
                // diffs.push(diff);
                prev = current;
            }
        }

        true
    }
}

impl TryFrom<String> for ReactorReport {
    type Error = crate::Error;

    fn try_from(report: String) -> Result<Self, Self::Error> {
        let mut levels = vec![];
        for level in report.split(" ").into_iter() {
            levels.push(Level::try_from(level)?)
        }

        Ok(Self { levels })
    }
}

impl TryFrom<&str> for Level {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(value.parse::<i16>()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_try_from_str() {
        assert_eq!(Level(5), Level::try_from("5").unwrap())
    }

    #[test]
    fn reactor_report_try_from_string() {
        let reactor_report = ReactorReport {
            levels: vec![Level(7), Level(6), Level(4), Level(2), Level(1)],
        };

        assert_eq!(
            reactor_report,
            ReactorReport::try_from("7 6 4 2 1".to_string()).unwrap()
        )
    }
}
