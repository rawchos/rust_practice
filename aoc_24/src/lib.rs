pub mod day_01;

pub use day_01::Day01Processor;

pub struct AoC24Processor {}

impl AoC24Processor {
    pub fn process() {
        Day01Processor::default().process();
    }
}
