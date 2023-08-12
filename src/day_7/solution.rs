use crate::create_day;
use crate::day_7::file_structure::get_file_tree_root;
use crate::day_7::interpreter::ParsedLine;
use itertools::Itertools;

create_day!(7, part_1, part_2);

fn part_1(file_content: String) {
    let parsed_lines = file_content
        .lines()
        .map(ToOwned::to_owned)
        .map(ParsedLine::try_from)
        .map(Result::unwrap)
        .collect_vec();

    let root = get_file_tree_root(parsed_lines);
    println!("{:#?}", root);
}

#[allow(unused)]
fn part_2(file_content: String) {
    todo!()
}
