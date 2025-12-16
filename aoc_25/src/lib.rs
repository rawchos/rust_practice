mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod error;

pub use error::Error;

use day_01::Day01Processor;
use day_02::Day02Processor;
use day_03::Day03Processor;
use day_04::Day04Processor;
use day_05::Day05Processor;
use day_06::Day06Processor;
use day_07::Day07Processor;
use day_08::Day08Processor;
use day_09::Day09Processor;

pub struct AoC25Processor {}

impl AoC25Processor {
    pub fn process() {
        Day01Processor::default().process();
        Day02Processor::default().process();
        Day03Processor::default().process();
        Day04Processor::default().process();
        Day05Processor::default().process();
        Day06Processor::default().process();
        Day07Processor::default().process();
        Day08Processor::default().process();
        Day09Processor::default().process();
    }
}
