use std::{fs, path::{Path, PathBuf}};

pub fn is_dir_exists(dir: &String) -> bool {
  if let Ok(metadata) = fs::metadata(dir) {
    return metadata.is_dir();
  } else {
    return false;
  }
}

pub fn create_dir(dir: &String) -> i8 {
  match fs::create_dir_all(dir) {
    Ok(_) => return 0,
    Err(e) => {
      eprintln!("{}", e);
      return -1;
    }
  }
}

pub fn copy_file_to_dir(dir: &str, path: &Path) -> i8 {
  let out_path = PathBuf::from(dir).join(path.file_name().unwrap());
  println!("Copy file \"{}\" to \"{}\".", path.display(), out_path.display());
  match fs::copy(path, &out_path) {
    Ok(_) => { 0 },
    Err(e) => {
    eprintln!("Failed to copy file \"{}\" to \"{}\": {}.", path.display(), out_path.display(), e);
    -1
  }
  }
}

pub fn has_content<P: AsRef<std::path::Path>>(target_dir: P) -> bool {
  let mut dir = fs::read_dir(target_dir).unwrap();

  for _ in &mut dir {
    return true;
  }
  false
}