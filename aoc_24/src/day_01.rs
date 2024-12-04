pub struct Day01Processor(String);

impl Day01Processor {
    fn new() -> Self {
        Self(String::from("./resources/day_01.txt"))
    }

    pub fn process(&self) {
        println!("Solving problems for day 01 using: {}", self.0)
    }
}

impl Default for Day01Processor {
    fn default() -> Self {
        Self::new()
    }
}
