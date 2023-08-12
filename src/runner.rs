use crate::core_traits::Solution;

pub trait DayExerciseRunStrategy {
    fn run(days: Vec<Box<dyn Solution>>);
}

pub struct RunLastDayBothParts;

impl DayExerciseRunStrategy for RunLastDayBothParts {
    fn run(days: Vec<Box<dyn Solution>>) {
        let last_day = days.last().expect("There should be at least 1 day.");
        last_day.execute_first();
        last_day.execute_second();
    }
}

pub struct RunEveryDayBothParts;

impl DayExerciseRunStrategy for RunEveryDayBothParts {
    fn run(days: Vec<Box<dyn Solution>>) {
        for solution in days {
            solution.execute_first();
            solution.execute_second();
        };
    }
}
