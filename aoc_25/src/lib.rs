mod day_01;
mod error;

pub use error::Error;

use day_01::Day01Processor;

pub struct AoC25Processor {}

impl AoC25Processor {
    pub fn process() {
        // println!("Running AoC 25 Puzzles");
        Day01Processor::default().process()
    }
}
