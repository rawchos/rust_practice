mod day_01;
mod day_02;
mod day_03;
mod error;

pub use error::Error;

use day_01::Day01Processor;
use day_02::Day02Processor;
use day_03::Day03Processor;

pub struct AoC25Processor {}

impl AoC25Processor {
    pub fn process() {
        Day01Processor::default().process();
        Day02Processor::default().process();
        Day03Processor::default().process();
    }
}
