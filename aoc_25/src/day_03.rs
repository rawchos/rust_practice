use utils::FileReader;

static DAY_03_FILE: &str = "./resources/aoc_25/day_03.txt";

pub struct Day03Processor(String);

impl Day03Processor {
    fn new() -> Self {
        Self(String::from(DAY_03_FILE))
    }

    pub fn process(&self) {
        PartOneProcessor::new(self.0.as_str()).process();
    }
}

impl Default for Day03Processor {
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
            Ok(p1_value) => println!("AoC 25 Day 03 Part 1: {}", p1_value.get()),
            Err(msg) => println!("AoC 25 Day 03 Part 1: Failed with message: {}", msg),
        }
    }
}

#[derive(Debug, PartialEq)]
struct PartOneValue(i64);

impl PartOneValue {
    fn get(&self) -> i64 {
        self.0
    }
}

impl TryFrom<FileReader> for PartOneValue {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let battery_banks = reader
            .read_lines()?
            .map_while(Result::ok)
            .map(BatteryBank::try_from)
            .collect::<Result<Vec<BatteryBank>, crate::Error>>()?;

        let all_joltages: Vec<i8> = battery_banks
            .into_iter()
            .map(|bb| bb.max_joltage())
            .collect();

        let total_joltage = all_joltages.into_iter().map(|j| j as i64).sum();

        Ok(PartOneValue(total_joltage))
    }
}

#[derive(Debug, PartialEq)]
struct BatteryBank(Vec<i8>);

impl BatteryBank {
    fn max_joltage(&self) -> i8 {
        let mut battery_counter = 0;
        let mut first_battery = 0i8;
        let mut second_battery = 0i8;
        let num_batteries = self.0.len();

        for battery in self.0.iter() {
            battery_counter += 1;

            if battery > &first_battery && battery_counter < num_batteries {
                first_battery = *battery;
                second_battery = 0;
                continue;
            }

            if battery > &second_battery {
                second_battery = *battery
            }
        }

        let Ok(joltage) =
            (first_battery.to_string() + second_battery.to_string().as_str()).parse::<i8>()
        else {
            // This should be able to parse but return 0 if not.
            return 0;
        };

        joltage
    }
}

impl TryFrom<String> for BatteryBank {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let batteries: Vec<i8> = value
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|x| x.to_string().parse::<i8>())
            .collect::<Result<Vec<i8>, std::num::ParseIntError>>()?;

        Ok(Self(batteries))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_FILE: &str = "../test-resources/aoc_25/day_03_sample.txt";

    #[test]
    fn test_battery_bank_from_string() {
        let batteries = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        assert_eq!(
            BatteryBank(batteries),
            BatteryBank::try_from(String::from("123456789")).unwrap()
        )
    }

    #[test]
    fn test_max_joltage() {
        assert_eq!(
            98,
            BatteryBank::try_from(String::from("987654321111111"))
                .unwrap()
                .max_joltage()
        );

        assert_eq!(
            89,
            BatteryBank::try_from(String::from("811111111111119"))
                .unwrap()
                .max_joltage()
        );

        assert_eq!(
            78,
            BatteryBank::try_from(String::from("234234234234278"))
                .unwrap()
                .max_joltage()
        );

        assert_eq!(
            92,
            BatteryBank::try_from(String::from("818181911112111"))
                .unwrap()
                .max_joltage()
        );
    }

    #[test]
    fn test_part_one_value_from_reader() {
        let reader = FileReader::new(SAMPLE_FILE);

        assert_eq!(357, PartOneValue::try_from(reader).unwrap().get())
    }
}
