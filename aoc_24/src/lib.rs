pub mod day_01;
mod error;

pub use day_01::Day01Processor;
pub use error::Error;

pub struct AoC24Processor {}

impl AoC24Processor {
    pub fn process() {
        Day01Processor::default().process();
    }
}
