#![allow(unused)]
use std::{borrow::BorrowMut, cell::RefCell, fs::File, rc::Rc};

use itertools::Itertools;

use crate::day_7::interpreter::ParsedLine;

pub trait TreeWalkExt {
    fn get_file_sizes(&self) -> usize;
}

impl<I: Iterator<Item = ParsedLine>> TreeWalkExt for I {
    fn get_file_sizes(&self) -> usize {
        todo!();
    }
}
// #[derive(Debug, Clone)]
// pub enum FileStructureObject {
//     Directory {
//         name: String,
//         contents: RefCell<Vec<FileStructureObject>>,
//     },
//     File {
//         name: String,
//         size: usize,
//     },
// }

// pub fn get_file_tree_root(commands: &[ParsedLine]) -> FileStructureObject {
//     let root_directory = Rc::new(FileStructureObject::Directory {
//         name: "\\".to_string(),
//         contents: RefCell::new(vec![]),
//     });

//     {
//         let mut head = root_directory.clone();
//         let mut current_path: Vec<Rc<FileStructureObject>> = vec![root_directory.clone()];

//         for line in commands {
//             println!("{:?}", &line);
//             println!("\n{:#?}\n", &root_directory);

//             match line {
//                 ParsedLine::Ls => continue,
//                 ParsedLine::Cd { dir_path } => {
//                     println!("{:?}", dir_path);
//                     match current_path.last().unwrap().as_ref() {
//                         FileStructureObject::Directory { name: _, contents } => {
//                             let c = contents
//                                 .borrow_mut()
//                                 .iter_mut()
//                                 .filter(|dir| match dir {
//                                     FileStructureObject::Directory { name, contents: _ } => {
//                                         name == dir_path.as_str()
//                                     }
//                                     _ => false,
//                                 })
//                                 .exactly_one()
//                                 .unwrap()
//                                 .clone();

//                             current_path.push(Rc::new(c.to_owned()));
//                             head = Rc::new(c.to_owned());
//                         }
//                         FileStructureObject::File { name: _, size: _ } => {
//                             panic!("File shouldn't get into the directory stack.")
//                         }
//                     }
//                 }
//                 ParsedLine::CdUpwards => {
//                     current_path.pop();
//                 }
//                 ParsedLine::CdIntoRoot => current_path = vec![root_directory.clone()],
//                 ParsedLine::CdDirRes { name } => {
//                     let current_dir = current_path.last().unwrap();
//                     match current_dir.as_ref() {
//                         FileStructureObject::Directory { name: _, contents } => {
//                             contents.borrow_mut().push(FileStructureObject::Directory {
//                                 name: name.to_string(),
//                                 contents: RefCell::new(vec![]),
//                             });
//                         }
//                         FileStructureObject::File { .. } => {
//                             panic!("File shouldn't get into the directory stack.")
//                         }
//                     }
//                 }
//                 ParsedLine::CdFileRes { size, name } => {
//                     let current_dir = current_path.last().unwrap();
//                     match current_dir.as_ref() {
//                         FileStructureObject::Directory { name: _, contents } => {
//                             contents.borrow_mut().push(FileStructureObject::File {
//                                 name: name.to_string(),
//                                 size,
//                             });
//                         }
//                         FileStructureObject::File { .. } => {
//                             panic!("File shouldn't get into the directory stack.")
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     Rc::try_unwrap(root_directory).unwrap()
// }
