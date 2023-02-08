use crate::create_day;
use std::collections::HashSet;
create_day!(
    3,
    |file_content: String| {
        let output: u32 = file_content
            .lines()
            .map(|line| line.split_at(line.len() / 2))
            .map(|(first_comp, second_comp)| {
                let first = first_comp.chars().collect::<HashSet<char>>();
                let second = second_comp.chars().collect::<HashSet<char>>();

                let common_items_priorities_sum: u32 = first
                    .intersection(&second)
                    .map(ToOwned::to_owned)
                    .map(get_item_priority)
                    .enumerate()
                    .map(|(idx, item_result)| match item_result {
                        Ok(v) => v,
                        Err(e) => panic!("Error in line {idx}: {e}"),
                    })
                    .sum();
                common_items_priorities_sum
            })
            .sum();

        println!("{output}");
    },
    |file_content: String| { todo!() }
);

fn get_item_priority(item: char) -> Result<u32, String> {
    let converted = match u32::try_from(item) {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("\"{item}\" doesnt have any assigned priority.")),
    }?;

    if item.is_lowercase() {
        Ok(converted - 96)
    } else if item.is_uppercase() {
        Ok(converted - 38)
    } else {
        Err(format!("\"{item}\" is not alphanumeric"))
    }
}
