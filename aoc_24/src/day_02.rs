use utils::FileReader;

static SAFE_LEVEL_DIFF: [i16; 3] = [1, 2, 3];

pub struct Day02Processor(String);

impl Day02Processor {
    fn new() -> Self {
        Self(String::from("./resources/aoc_24/day_02.txt"))
    }

    pub fn process(&self) {
        PartOneProcessor::new(self.0.as_str()).process();
        PartTwoProcessor::new(self.0.as_str()).process();
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
            Ok(p1_value) => println!("AoC 24 Day 02 Part 1: {}", p1_value.value()),
            Err(msg) => println!("AoC 24 Day 02 Part 1: Failed with this message: {}", msg),
        }
    }
}

#[derive(Debug, PartialEq)]
struct PartOneValue(i16);

impl PartOneValue {
    fn value(&self) -> i16 {
        self.0
    }
}

impl TryFrom<FileReader> for PartOneValue {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let reactor_reports = reader
            .read_lines()?
            .map_while(Result::ok)
            .map(ReactorReport::try_from)
            .collect::<Result<Vec<ReactorReport>, crate::Error>>()?;

        let safe_reports: Vec<ReactorReport> = reactor_reports
            .into_iter()
            .filter(|rr| rr.is_safe())
            .collect();

        Ok(PartOneValue(i16::try_from(safe_reports.len())?))
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
            Ok(p2_value) => println!("AoC 24 Day 02 Part 2: {}", p2_value.value()),
            Err(msg) => println!("AoC 24 Day 02 Part 2: Failed with this message: {}", msg),
        }
    }
}

#[derive(Debug, PartialEq)]
struct PartTwoValue(i16);

impl PartTwoValue {
    fn value(&self) -> i16 {
        self.0
    }
}

impl TryFrom<FileReader> for PartTwoValue {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let reactor_reports = reader
            .read_lines()?
            .map_while(Result::ok)
            .map(ReactorReport::try_from)
            .collect::<Result<Vec<ReactorReport>, crate::Error>>()?;

        let safe_reports: Vec<ReactorReport> = reactor_reports
            .into_iter()
            .filter(|rr| rr.is_dampened_safe())
            .collect();

        Ok(PartTwoValue(i16::try_from(safe_reports.len())?))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Level(i16);

#[derive(Debug, PartialEq)]
struct ReactorReport {
    levels: Vec<Level>,
}

impl ReactorReport {
    fn is_safe(&self) -> bool {
        let mut prev = self.levels[0];
        let mut positives = vec![];
        let mut negatives = vec![];

        for idx in 1..self.levels.len() {
            let current = self.levels[idx];
            let diff = current.0 - prev.0;

            if !SAFE_LEVEL_DIFF.contains(&diff.abs()) {
                return false;
            } else {
                if diff.is_positive() {
                    positives.push(diff)
                } else {
                    negatives.push(diff)
                }

                if !positives.is_empty() && !negatives.is_empty() {
                    return false;
                }
                prev = current;
            }
        }

        true
    }

    fn is_dampened_safe(&self) -> bool {
        self.is_safe() || {
            for idx in 0..self.levels.len() {
                let mut levels = self.levels.clone();
                levels.remove(idx);

                if (ReactorReport { levels }).is_safe() {
                    return true;
                }
            }
            false
        }
    }
}

impl TryFrom<String> for ReactorReport {
    type Error = crate::Error;

    fn try_from(report: String) -> Result<Self, Self::Error> {
        let mut levels = vec![];
        for level in report.split(" ") {
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

    #[test]
    fn report_safely_decreasing() {
        assert!(ReactorReport::try_from("7 6 4 2 1".to_string())
            .unwrap()
            .is_safe())
    }

    #[test]
    fn report_safely_increasing() {
        assert!(ReactorReport::try_from("1 3 6 7 9".to_string())
            .unwrap()
            .is_safe())
    }

    #[test]
    fn report_unsafe_increase() {
        assert!(!ReactorReport::try_from("1 2 7 8 9".to_string())
            .unwrap()
            .is_safe())
    }

    #[test]
    fn report_dampened_unsafe_increase() {
        assert!(!ReactorReport::try_from("1 2 7 8 9".to_string())
            .unwrap()
            .is_dampened_safe())
    }

    #[test]
    fn report_unsafe_increase_and_decrease() {
        assert!(!ReactorReport::try_from("1 3 2 4 5".to_string())
            .unwrap()
            .is_safe())
    }

    #[test]
    fn report_dampened_safe_increase_and_decrease() {
        assert!(ReactorReport::try_from("1 3 2 4 5".to_string())
            .unwrap()
            .is_dampened_safe())
    }

    #[test]
    fn report_unsafe_no_change_in_level() {
        assert!(!ReactorReport::try_from("8 6 4 4 1".to_string())
            .unwrap()
            .is_safe())
    }

    #[test]
    fn report_dampened_safe_no_change_in_level() {
        assert!(ReactorReport::try_from("8 6 4 4 1".to_string())
            .unwrap()
            .is_dampened_safe())
    }
}
