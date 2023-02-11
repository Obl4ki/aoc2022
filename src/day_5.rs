use itertools::Itertools;

use crate::create_day;

create_day!(
    5,
    |file_content: String| {
        let info = get_stacks_info(&file_content);
        let stacks = crate_stacks_from_text(&file_content, &info);

        let commands = commands_from_text(&file_content, &info);

        let stacks = execute_commands_on_stacks(stacks, commands);
        let tops = get_tops(stacks);

        println!("{tops}");
    },
    |file_content: String| {
        let info = get_stacks_info(&file_content);
        let stacks = crate_stacks_from_text(&file_content, &info);

        let commands = commands_from_text(&file_content, &info);

        let stacks = execute_commands_on_stacks_part_2(stacks, commands);
        let tops = get_tops(stacks);

        println!("{tops}");
    }
);

struct CrateStacksInfo {
    crates_max_height: usize,
    num_of_stacks: usize,
}

fn get_stacks_info(text: &str) -> CrateStacksInfo {
    let crates_max_height = text
        .lines()
        .position(|line| line.chars().next().expect("Line must not be empty") == ' ')
        .expect(
            "Space as the first char in line is an indication 
    that the cargo crate input has ended.
    It seems that there isn't one present.",
        );

    let crate_rows: Vec<&str> = text
        .lines()
        .take(crates_max_height)
        .collect_vec()
        .into_iter()
        .rev()
        .collect_vec();

    let num_of_stacks = crate_rows[0].chars().filter(|c| c == &'[').count();
    CrateStacksInfo {
        crates_max_height,
        num_of_stacks,
    }
}

fn crate_stacks_from_text(text: &str, info: &CrateStacksInfo) -> Vec<Vec<String>> {
    let crate_rows: Vec<&str> = text
        .lines()
        .take(info.crates_max_height)
        .collect_vec()
        .into_iter()
        .rev()
        .collect_vec();

    (0..info.num_of_stacks)
        .map(|stack_idx| {
            let crate_start = stack_idx * 3 + stack_idx;
            let stack = crate_rows
                .iter()
                .map(move |row| row.get(crate_start..=crate_start + 2).unwrap().to_string())
                .filter(|c| !c.contains(' '))
                .collect_vec();
            stack
        })
        .collect_vec()
}

fn commands_from_text(file_content: &str, info: &CrateStacksInfo) -> Vec<CrateCommand> {
    file_content
        .lines()
        .skip(info.crates_max_height + 2) // skip all crates, numerical description and empty line
        .map(parse_command)
        .collect_vec()
}

#[derive(Debug)]
struct CrateCommand {
    quantity: usize,
    source: usize,
    destination: usize,
}

fn parse_command(command: &str) -> CrateCommand {
    let tokens = command.split(' ').collect_vec();

    CrateCommand {
        quantity: tokens[1].parse().unwrap(),
        source: tokens[3].parse().unwrap(),
        destination: tokens[5].parse().unwrap(),
    }
}

fn execute_commands_on_stacks(
    mut stacks: Vec<Vec<String>>,
    commands: Vec<CrateCommand>,
) -> Vec<Vec<String>> {
    for command in &commands {
        for _ in 0..command.quantity {
            let popped_crate = stacks[command.source - 1].pop().unwrap();
            stacks[command.destination - 1].push(popped_crate);
        }
    }

    stacks
}

fn execute_commands_on_stacks_part_2(
    mut stacks: Vec<Vec<String>>,
    commands: Vec<CrateCommand>,
) -> Vec<Vec<String>> {
    for command in &commands {
        let mut temp_stack = vec![];
        for _ in 0..command.quantity {
            let popped_crate = stacks[command.source - 1].pop().unwrap();
            temp_stack.push(popped_crate)
        }

        stacks[command.destination - 1].extend(temp_stack.into_iter().rev())
    }

    stacks
}

fn get_tops(stacks: Vec<Vec<String>>) -> String {
    stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .map(|top| top.chars().nth(1).unwrap())
        .collect()
}
