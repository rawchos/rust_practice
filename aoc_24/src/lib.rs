mod day_01;
mod day_02;
mod error;

use day_01::Day01Processor;
use day_02::Day02Processor;
pub use error::Error;

pub struct AoC24Processor {}

impl AoC24Processor {
    pub fn process() {
        Day01Processor::default().process();
        Day02Processor::default().process();
    }
}
