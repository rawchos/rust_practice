use std::{
    fs::File,
    io::{self, BufRead},
};

pub struct FileReader(String);

// Sometimes, the puzzle input is one large string on one line, rather than a string
// per line. Let's get a convenience method for that.
// ex: pub fn read_string(&self) -> io::Result<String>           (or something)

impl FileReader {
    pub fn new(filepath: &str) -> Self {
        Self(String::from(filepath))
    }

    pub fn read_lines(&self) -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(self.0.as_str())?;
        Ok(io::BufReader::new(file).lines())
    }
}

pub struct StringUtils;
/*
let splittable_string = "123456789";
    let sub_string: Vec<String> = splittable_string
    .chars()
    .collect::<Vec<char>>()
    .chunks(4)
    .map(|chunk| chunk.iter().collect())
    .collect();
    println!("Splittable String: {:?}", sub_string);
 */

impl StringUtils {
    pub fn partition_by(s: String, sub_length: usize) -> Vec<String> {
        if sub_length == 0 {
            return vec![s];
        }

        let sub_string: Vec<String> = s
            .chars()
            .collect::<Vec<char>>()
            .chunks(sub_length)
            .map(|chunk| chunk.iter().collect())
            .collect();

        sub_string
    }

    pub fn all_equal(parts: Vec<String>) -> bool {
        if parts.len() <= 1 {
            return true;
        }

        let first_string = &parts[0];
        for idx in 1..parts.len() {
            if parts[idx] != *first_string {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        let file_reader = FileReader::new("../test-resources/sample_file.txt");

        assert_eq!(4, file_reader.read_lines().unwrap().count());
        assert_eq!(
            String::from("pqr3stu8vwx"),
            file_reader.read_lines().unwrap().nth(1).unwrap().unwrap()
        )
    }

    #[test]
    fn test_partition_by() {
        assert_eq!(
            vec![String::from("123456789")],
            StringUtils::partition_by(String::from("123456789"), 0)
        );

        assert_eq!(
            vec![
                String::from("123"),
                String::from("456"),
                String::from("789")
            ],
            StringUtils::partition_by(String::from("123456789"), 3)
        );

        assert_eq!(
            vec![
                String::from("1234"),
                String::from("5678"),
                String::from("9")
            ],
            StringUtils::partition_by(String::from("123456789"), 4)
        )
    }

    #[test]
    fn test_all_equal() {
        let test1 = vec![
            String::from("123"),
            String::from("123"),
            String::from("123"),
        ];
        let test2 = vec![
            String::from("123"),
            String::from("456"),
            String::from("789"),
        ];
        let test3 = vec![
            String::from("123"),
            String::from("456"),
            String::from("123"),
        ];
        let test4 = vec![String::from("1234"), String::from("1234")];

        assert!(StringUtils::all_equal(test1));
        assert!(!StringUtils::all_equal(test2));
        assert!(!StringUtils::all_equal(test3));
        assert!(StringUtils::all_equal(test4));
    }
}
