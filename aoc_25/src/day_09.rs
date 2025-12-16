use utils::FileReader;

static DAY_09_FILE: &str = "./resources/aoc_25/day_09.txt";

pub struct Day09Processor(String);

impl Day09Processor {
    fn new() -> Self {
        Self(String::from(DAY_09_FILE))
    }

    pub fn process(&self) {
        self.process_part1();
    }

    fn process_part1(&self) {
        let reader = FileReader::new(&self.0);

        match Day09Input::try_from(reader) {
            Ok(input) => println!("AoC 25 Day 09 Part 1: {}", input.largest_rectangle()),
            Err(msg) => println!("AoC 25 Day 09 Part 1: Failed with message: {}", msg),
        }
    }
}

impl Default for Day09Processor {
    fn default() -> Self {
        Day09Processor::new()
    }
}

#[derive(Debug, PartialEq)]
struct Day09Input {
    points: Vec<Point>,
}

impl Day09Input {
    fn largest_rectangle(&self) -> i64 {
        let mut max_area: i64 = 0;

        for i in 0..(self.points.len() - 1) {
            for j in (i + 1)..self.points.len() {
                let area = self.points[i].area_between(self.points[j]);
                // sizes.entry(area).or_insert((i, j));
                max_area = max_area.max(area);
            }
        }

        max_area
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point(usize, usize);

impl Point {
    fn row(&self) -> usize {
        self.1
    }

    fn column(&self) -> usize {
        self.0
    }

    fn area_between(&self, other: Point) -> i64 {
        let width = self.row().abs_diff(other.row()) + 1;
        let height = self.column().abs_diff(other.column()) + 1;

        (width * height) as i64
    }
}

impl TryFrom<String> for Point {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let point: Vec<&str> = value.split(",").collect();
        Ok(Self(point[0].parse::<usize>()?, point[1].parse::<usize>()?))
    }
}

impl TryFrom<FileReader> for Day09Input {
    type Error = crate::Error;

    fn try_from(reader: FileReader) -> Result<Self, Self::Error> {
        let points: Vec<Point> = reader
            .read_lines()?
            .map_while(Result::ok)
            .collect::<Vec<String>>()
            .into_iter()
            .map(Point::try_from)
            .collect::<Result<Vec<Point>, crate::Error>>()?;

        Ok(Self { points })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_FILE: &str = "../test-resources/aoc_25/day_09_sample.txt";

    #[test]
    fn test_point_from_string() {
        assert_eq!(Point(7, 1), Point::try_from(String::from("7,1")).unwrap());
    }

    #[test]
    fn test_day09input_from_reader() {
        let points = vec![
            Point(7, 1),
            Point(11, 1),
            Point(11, 7),
            Point(9, 7),
            Point(9, 5),
            Point(2, 5),
            Point(2, 3),
            Point(7, 3),
        ];
        let expected = Day09Input { points };
        let reader = FileReader::new(SAMPLE_FILE);

        assert_eq!(expected, Day09Input::try_from(reader).unwrap());
    }

    #[test]
    fn test_largest_rectangle() {
        let reader = FileReader::new(SAMPLE_FILE);
        let sample_input = Day09Input::try_from(reader).unwrap();

        assert_eq!(50, sample_input.largest_rectangle());
    }
}
