use std::collections::HashMap;

use crate::day_7::interpreter::ParsedLine;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FileType {
    File,
    Directory,
}

type FileSystemObjects = HashMap<String, (u32, FileType)>;

pub fn get_filesystem_objects(parsed_lines: Vec<ParsedLine>) -> FileSystemObjects {
    let mut filesystem_objects: FileSystemObjects = HashMap::new();

    // Initialize with root directory
    filesystem_objects.insert("/".to_string(), (0, FileType::Directory));

    let mut current_path = "/".to_string();

    for line in parsed_lines {
        match line {
            ParsedLine::Cd { dir_path } => {
                if dir_path == ".." {
                    let path_parts: Vec<&str> = current_path.split('/').collect();
                    current_path = path_parts[..path_parts.len() - 1].join("/");
                } else if dir_path == "/" {
                    current_path = "".to_string();
                } else {
                    current_path = format!("{}/{}", current_path, dir_path);
                }
            }
            ParsedLine::Ls => {
                // Do nothing for `ls` command, as it doesn't affect the current path or filesystem
            }
            ParsedLine::DirRes { name } => {
                let dir_path = format!("{}/{}", current_path, name);
                filesystem_objects.insert(dir_path, (0, FileType::Directory)); // Initialize directory size to 0
            }
            ParsedLine::FileRes { size, name } => {
                let file_path = format!("{}/{}", current_path, name);
                filesystem_objects.insert(file_path, (size as u32, FileType::File));
            }
        }
    }

    // Now we can calculate the sizes of directories based on the files they contain
    let mut dir_sizes: FileSystemObjects = HashMap::new();

    // Go through the filesystem objects in reverse order of their paths (deepest first)
    for (path, (size, file_type)) in filesystem_objects.iter() {
        if let FileType::File = file_type {
            // It's a file, so we need to add its size to all parent directories
            let mut path_parts: Vec<&str> = path.split('/').collect();
            let mut first_file = true;
            while !path_parts.is_empty() {
                let mut dir_path = path_parts.join("/");
                let current_file_type = if first_file {
                    FileType::File
                } else {
                    FileType::Directory
                };
                if dir_path.is_empty() {
                    dir_path = "/".to_string();
                }
                *dir_sizes.entry(dir_path).or_insert((0, current_file_type)) = (
                    dir_sizes.get(&dir_path).map_or(0, |(s, _)| *s) + size,
                    current_file_type,
                );
                first_file = false;
                path_parts.pop(); // Move up one directory
            }
        }
    }

    // Update the filesystem objects with the calculated directory sizes
    for (path, (size, file_type)) in dir_sizes {
        *filesystem_objects
            .get_mut(&path)
            .unwrap_or_else(|| panic!("Expected `{}` to exist in filesystem objects", path)) =
            (size, file_type);
    }

    filesystem_objects
}

#[cfg(test)]
mod test {
    use crate::utils::testing::assert_equal_hashmaps;

    use super::*;
    // $ cd /
    // $ ls
    // dir a
    // 14848514 b.txt
    // 8504156 c.dat
    // dir d
    // $ cd a
    // $ ls
    // dir e
    // 29116 f
    // 2557 g
    // 62596 h.lst
    // $ cd e
    // $ ls
    // 584 i
    // $ cd ..
    // $ cd ..
    // $ cd d
    // $ ls
    // 4060174 j
    // 8033020 d.log
    // 5626152 d.ext
    // 7214296 k
    #[test]
    fn test_get_filesystem_objects() {
        let input: Vec<ParsedLine> = vec![
            ParsedLine::Cd {
                dir_path: "/".to_string(),
            },
            ParsedLine::Ls,
            ParsedLine::DirRes {
                name: "a".to_string(),
            },
            ParsedLine::FileRes {
                size: 14848514,
                name: "b.txt".to_string(),
            },
            ParsedLine::FileRes {
                size: 8504156,
                name: "c.dat".to_string(),
            },
            ParsedLine::DirRes {
                name: "d".to_string(),
            },
            ParsedLine::Cd {
                dir_path: "a".to_string(),
            },
            ParsedLine::Ls,
            ParsedLine::DirRes {
                name: "e".to_string(),
            },
            ParsedLine::FileRes {
                size: 29116,
                name: "f".to_string(),
            },
            ParsedLine::FileRes {
                size: 2557,
                name: "g".to_string(),
            },
            ParsedLine::FileRes {
                size: 62596,
                name: "h.lst".to_string(),
            },
            ParsedLine::Cd {
                dir_path: "e".to_string(),
            },
            ParsedLine::Ls,
            ParsedLine::FileRes {
                size: 584,
                name: "i".to_string(),
            },
            ParsedLine::Cd {
                dir_path: "..".to_string(),
            },
            ParsedLine::Cd {
                dir_path: "..".to_string(),
            },
            ParsedLine::Cd {
                dir_path: "d".to_string(),
            },
            ParsedLine::Ls,
            ParsedLine::FileRes {
                size: 4060174,
                name: "j".to_string(),
            },
            ParsedLine::FileRes {
                size: 8033020,
                name: "d.log".to_string(),
            },
            ParsedLine::FileRes {
                size: 5626152,
                name: "d.ext".to_string(),
            },
            ParsedLine::FileRes {
                size: 7214296,
                name: "k".to_string(),
            },
        ];

        let expected: FileSystemObjects = HashMap::from_iter(vec![
            ("/".to_string(), (48381165, FileType::Directory)),
            ("/a".to_string(), (94853, FileType::Directory)),
            ("/a/e".to_string(), (584, FileType::Directory)),
            ("/a/e/i".to_string(), (584, FileType::File)),
            ("/d".to_string(), (24933642, FileType::Directory)),
            ("/b.txt".to_string(), (14848514, FileType::File)),
            ("/c.dat".to_string(), (8504156, FileType::File)),
            ("/a/f".to_string(), (29116, FileType::File)),
            ("/a/g".to_string(), (2557, FileType::File)),
            ("/a/h.lst".to_string(), (62596, FileType::File)),
            ("/d/j".to_string(), (4060174, FileType::File)),
            ("/d/d.log".to_string(), (8033020, FileType::File)),
            ("/d/d.ext".to_string(), (5626152, FileType::File)),
            ("/d/k".to_string(), (7214296, FileType::File)),
        ]);

        let actual = get_filesystem_objects(input);

        assert_equal_hashmaps(&expected, &actual);
    }
}
