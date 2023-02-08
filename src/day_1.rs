use crate::create_day;
use itertools::Itertools;

create_day!(
    1,
    |file_content: String| {
        let calories = text_into_elves_calories(&file_content);
        let max_calories = calories
            .into_iter()
            .max()
            .expect("There should be a maximum value");
        println!("{max_calories}");
    },
    |file_content: String| {
        let calories = text_into_elves_calories(&file_content);
        let top_three_calories = calories.into_iter().sorted().rev().take(3).sum::<i32>();
        println!("{top_three_calories}");
    }
);

fn text_into_elves_calories(file_content: &str) -> impl Iterator<Item = i32> + '_ {
    let elves = file_content
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .collect_vec();

    elves.into_iter().map(|elf| {
        elf.split('\n')
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .sum::<i32>()
    })
}
