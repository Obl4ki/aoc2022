#![deny(clippy::all)]

use core_traits::Solution;
mod core_traits;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod macros;

fn main() {
    let all_solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(day_1::Day),
        Box::new(day_2::Day),
        Box::new(day_3::Day),
        Box::new(day_4::Day),
        Box::new(day_5::Day),
    ];
    for solution in all_solutions {
        solution.execute_first();
        solution.execute_second();
    }
}
