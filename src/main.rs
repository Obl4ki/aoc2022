#![deny(clippy::all)]

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

mod utils;

use runner::DayExerciseRunStrategy;
use utils::runner;

fn main() {
    // runner::RunEveryDayBothParts::run(day_solutions);
    runner::RunLastDayBothParts::run(runner::DaysToRun(vec![
        Box::new(day_1::Day),
        Box::new(day_2::Day),
        Box::new(day_3::Day),
        Box::new(day_4::Day),
        Box::new(day_5::Day),
        Box::new(day_6::Day),
        Box::new(day_7::Day),
    ]));
}
