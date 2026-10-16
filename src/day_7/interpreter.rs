use anyhow::{anyhow, bail};
use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum ParsedLine {
    Ls,
    Cd { dir_path: String },
    DirRes { name: String },
    FileRes { size: usize, name: String },
}

impl TryFrom<&str> for ParsedLine {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split_line = value.split(' ').collect_vec();
        match split_line[..] {
            ["$", "ls"] => Ok(ParsedLine::Ls),
            ["$", "cd", dir_path] => Ok(Self::Cd {
                dir_path: dir_path.to_owned(),
            }),

            ["dir", directory_name] => Ok(Self::DirRes {
                name: directory_name.to_owned(),
            }),

            [file_size, file_name] => {
                let parsed_size = file_size.parse().map_err(|e| {
                    anyhow!(
                        "ERROR: Expected the file size as a first argument, but got {} instead",
                        e
                    )
                })?;

                Ok(Self::FileRes {
                    size: parsed_size,
                    name: file_name.to_owned(),
                })
            }

            _ => bail!(
                "ERROR: {:?} not recognized as an internal or external command...",
                value
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cd_command() {
        let input_str = "$ cd a
        $ cd dgf
        $ cd ..
        $ cd /
        $ cd a";

        let actual = input_str
            .lines()
            .map(str::trim)
            .map(ParsedLine::try_from)
            .map(Result::unwrap)
            .collect_vec();

        let expected: Vec<ParsedLine> = vec![
            ParsedLine::Cd {
                dir_path: "a".to_string(),
            },
            ParsedLine::Cd {
                dir_path: "dgf".to_string(),
            },
            ParsedLine::Cd {
                dir_path: "..".to_string(),
            },
            ParsedLine::Cd {
                dir_path: "/".to_string(),
            },
            ParsedLine::Cd {
                dir_path: "a".to_string(),
            },
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_ls_command() {
        let input_str = "$ ls
        dir cgw
        dir fbhz
        dir lvrzvt
        224312 vngq";

        let actual = input_str
            .lines()
            .map(str::trim)
            .map(ParsedLine::try_from)
            .map(Result::unwrap)
            .collect_vec();

        let expected: Vec<ParsedLine> = vec![
            ParsedLine::Ls,
            ParsedLine::DirRes {
                name: "cgw".to_string(),
            },
            ParsedLine::DirRes {
                name: "fbhz".to_string(),
            },
            ParsedLine::DirRes {
                name: "lvrzvt".to_string(),
            },
            ParsedLine::FileRes {
                size: 224312,
                name: "vngq".to_string(),
            },
        ];

        assert_eq!(expected, actual);
    }
}
