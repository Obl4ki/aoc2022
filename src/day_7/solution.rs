use crate::create_day;
use crate::day_7::interpreter::ParsedLine;
use crate::day_7::tree_walk::TreeWalkExt;

create_day!(7, part_1, part_2);

fn part_1(file_content: String) {
    let file_sizes = file_content
        .lines()
        .map(ToOwned::to_owned)
        .map(ParsedLine::try_from)
        .map(Result::unwrap)
        .get_file_sizes();

    println!("{:#?}", file_sizes);
}

#[allow(unused)]
fn part_2(file_content: String) {
    todo!()
}
