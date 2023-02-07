#![deny(clippy::all)]

use core_traits::Solution;
mod core_traits;
mod day_1;
mod day_2;
mod macros;

fn main() {
    execute_solution(day_1::Day);
    execute_solution(day_2::Day);
}

fn execute_solution<S: Solution>(solution: S) {
    solution.execute_first();
    solution.execute_second();
}
