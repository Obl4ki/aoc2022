use itertools::Itertools;

use crate::create_day;

create_day!(
    '4',
    |file_content: String| {
        let output: u32 = file_content
            .lines()
            .map(|line| line.split(',').collect_tuple::<(_, _)>())
            .map(Option::unwrap)
            .map(|(first, second)| {
                let split_parse_elf = |elf: String| -> (u32, u32) {
                    elf.split('-')
                        .map(str::parse::<u32>)
                        .map(Result::unwrap)
                        .collect_tuple::<(_,_)>()
                        .unwrap()
                };

                let (first_start, first_end) = split_parse_elf(first.to_owned());
                let (second_start, second_end) = split_parse_elf(second.to_owned());

                let (first_start, second_start, first_end, second_end) =
                    if first_start < second_start {
                        (first_start, second_start, first_end, second_end)
                    } else {
                        (second_start, first_start, second_end, first_end)
                    };

                if first_end >= second_end || first_start == second_start
                // || first_start < second_start && first_end < second_end
                {
                    println!("T {first_start}-{first_end}, {second_start}-{second_end}");
                    1
                } else {
                    println!("X {first_start}-{first_end}, {second_start}-{second_end}");
                    0
                }
            })
            .sum();

        println!("{output}")
    },
    |file_content: String| { todo!() }
);
