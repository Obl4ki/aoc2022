use crate::day_7::interpreter::ParsedLine;

pub trait TreeWalkExt {
    fn get_file_sizes(&self) -> usize;
}

impl<I: Iterator<Item = ParsedLine>> TreeWalkExt for I {
    fn get_file_sizes(&self) -> usize {
        todo!();
    }
}
