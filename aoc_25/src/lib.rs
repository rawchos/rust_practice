mod day_01;
mod day_02;
mod error;

pub use error::Error;

use day_01::Day01Processor;
use day_02::Day02Processor;

pub struct AoC25Processor {}

impl AoC25Processor {
    pub fn process() {
        Day01Processor::default().process();
        Day02Processor::default().process()
    }
}
