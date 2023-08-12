#[macro_export]
macro_rules! create_day {
    ($day_num:literal, $exec_first_code:expr, $exec_second_code:expr) => {
        use std::fs;

        use $crate::utils::core_traits::Solution;

        pub struct Day;

        impl Solution for Day {
            fn execute_first(&self) {
                println!("Day {} part 1:", $day_num);
                println!("--------------------------");
                let file_content =
                    fs::read_to_string(format!("data/day_{}.txt", $day_num)).unwrap();
                let file_content: String = file_content.replace('\r', "");

                $exec_first_code(file_content);
                println!("--------------------------");
            }

            fn execute_second(&self) {
                println!("Day {} part 2:", $day_num);
                println!("--------------------------");
                let file_content =
                    fs::read_to_string(format!("data/day_{}.txt", $day_num)).unwrap();
                let file_content = file_content.replace('\r', "");

                $exec_second_code(file_content);
                println!("--------------------------\n\n");
            }
        }
    };
}
