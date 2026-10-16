use itertools::Itertools;

use crate::create_day;
use crate::day_7::interpreter::ParsedLine;
use crate::day_7::tree_walk::{get_filesystem_objects, FileType};

create_day!(7, part_1, part_2);

fn part_1(file_content: String) {
    let lines = file_content.lines();

    let parsed_lines = lines
        .map(ParsedLine::try_from)
        .map(Result::unwrap)
        .collect_vec();

    let fs_objects = get_filesystem_objects(parsed_lines);

    // filter out directories with size <= 100000 and sum their sizes
    let total_size: u32 = fs_objects
        .iter()
        .filter(|(_, (size, file_type))| *file_type == FileType::Directory && *size <= 100_000)
        .map(|(_, (size, _))| *size)
        .sum();

    println!("{:#?}", total_size);
}

#[allow(unused)]
fn part_2(file_content: String) {
    let lines = file_content.lines();

    let parsed_lines = lines
        .map(ParsedLine::try_from)
        .map(Result::unwrap)
        .collect_vec();

    let fs_objects = get_filesystem_objects(parsed_lines);
    let total_disk_space = 70000000;
    let used_space = fs_objects.get("/").unwrap().0;

    let min_size = 30_000_000 - (total_disk_space - used_space);
    let smallest_dir_to_delete = fs_objects
        .iter()
        .filter(|(_, (size, file_type))| *file_type == FileType::Directory && *size >= min_size)
        .min_by_key(|(_, (size, _))| *size);

    if let Some((path, (size, _))) = smallest_dir_to_delete {
        println!("Smallest directory to delete: {} with size {}", path, size);
    } else {
        println!("No directory found that meets the criteria.");
    }
}
