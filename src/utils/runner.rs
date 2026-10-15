use crate::utils::core_traits::Solution;

pub trait DayExerciseRunStrategy {
    fn run(days: DaysToRun);
}

pub struct   RunLastDayBothParts;

impl DayExerciseRunStrategy for RunLastDayBothParts {
    fn run(DaysToRun(days): DaysToRun) {
        let last_day = days.last().expect("There should be at least 1 day.");
        last_day.execute_first();
        last_day.execute_second();
    }
}

pub struct DaysToRun(pub Vec<Box<dyn Solution>>);
