
use std::{collections::HashSet, path::{Path, PathBuf}};
use walkdir::{DirEntry, WalkDir};

use crate::{base::{digest::file_sha256_digest, file::{copy_file_to_dir, create_dir, has_content, is_dir_exists}}, common::sys::CommandHandler};
#[derive(Default)]
pub struct CompareFilesStruct {
  output_dir: String,
  dirs_to_compare: Vec<String>,
  files_sha256: HashSet<String>
}

impl CompareFilesStruct {
  fn process_entry(&mut self, root_path:&Path, entry: &walkdir::DirEntry) {
    let path = entry.path();
    if path == root_path {
      return;
    }

    match entry.file_type() {
      ft if ft.is_dir() => {
        println!("Encountered directory: {}", path.display());
        for sub_entry in WalkDir::new(&path).into_iter().filter_map(Result::ok) {
          self.process_entry(path, &sub_entry);
        }
      },
      ft if ft.is_file() => {
        self.process_file(entry);
      },
      _ => {
        eprintln!("Encountered non-file item: {}", path.display());
      }
    }
  }

  fn process_file(&mut self, entry: &DirEntry) {
    match file_sha256_digest(entry.path()) {
      Ok(s) => {
        if self.files_sha256.insert(s) == true {
          copy_file_to_dir(&self.output_dir, entry.path());     
        }
      },
      Err(e) => {
        eprintln!("Failed to read string from file: {:?}", e);
      }
    }
  }

  pub fn compare_files_and_copy_to(&mut self, index: usize) {
    let dir = &self.dirs_to_compare[index];
    let root_path = PathBuf::from(&dir);

    for entry in WalkDir::new(&dir) {
      let entry = match entry {
        Ok(e) => e,
        Err(e) => {
          eprintln!("{}", e);
          continue;
        }
      };

      self.process_entry(&root_path, &entry);
    }
  }
}

impl CommandHandler for CompareFilesStruct {
  fn do_command(&mut self) -> i8 {
    // check dir exists
    {
      for dir in self.dirs_to_compare.iter() {
        if is_dir_exists(dir) != true {
          eprintln!("{} is not exists.", dir);
          return -1;
        }
      }
      if is_dir_exists(&self.output_dir) != true && create_dir(&self.output_dir) != 0 {
        eprintln!("create {} error.", &self.output_dir);
        return -1;
      } else if has_content(&self.output_dir) {
        eprintln!("Target directory {} already contains content. Stopping copy.", self.output_dir);
        return -1;
      }
    }

    let len = self.dirs_to_compare.len();

    // compare files
    for i in 0..len {
      self.compare_files_and_copy_to(i);
    }
    return 0;
  }

  fn store_args(&mut self, args: &[String]) -> i8 {
    if args.len() <= 1 {
      return -1;
    }
    self.output_dir = args[0].clone();
    self.dirs_to_compare.extend_from_slice(&args[1..]);
    0
  }
}