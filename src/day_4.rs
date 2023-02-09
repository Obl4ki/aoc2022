use itertools::Itertools;

use crate::create_day;

create_day!(
    '4',
    |file_content: String| {
        let pairs_iter = file_content
            .lines()
            .map(|line| line.split(',').collect_tuple::<(_, _)>())
            .map(Option::unwrap);

        let split_parse_elf = |elf: String| -> (u32, u32) {
            elf.split('-')
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect_tuple::<(_, _)>()
                .unwrap()
        };

        let counts: usize = pairs_iter
            .map(|(first, second)| {
                let (first_start, first_end) = split_parse_elf(first.to_owned());
                let (second_start, second_end) = split_parse_elf(second.to_owned());

                vec![(first_start, first_end), (second_start, second_end)]
                    .into_iter()
                    .sorted_by(|(x1, _), (x2, _)| x1.cmp(x2))
                    .collect_tuple::<((u32, u32), (u32, u32))>()
                    .unwrap()
            })
            .filter(|((first_start, first_end), (second_start, second_end))| {
                first_end >= second_end || first_start == second_start
            })
            .count();

        println!("{}", &counts)
    },
    |file_content: String| {
        todo!();
    }
);
