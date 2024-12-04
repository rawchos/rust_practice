use std::{
    fs::File,
    io::{self, BufRead},
};

pub struct FileReader(String);

impl FileReader {
    pub fn new(filepath: &str) -> Self {
        Self(String::from(filepath))
    }

    pub fn read_lines(&self) -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(self.0.as_str())?;
        Ok(io::BufReader::new(file).lines())
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
}
