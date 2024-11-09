use std::fs::File;
use std::io::Read;
use sha2::{Digest, Sha256};
use hex;

pub fn file_sha256_digest(path: &std::path::Path) -> Result<String, Box<dyn std::error::Error>>  {
  let mut file = File::open(path)?;

  // 创建一个sha256对象
  let mut hasher = Sha256::new();

  // 将文件读取到hash对象中
  let mut buffer = [0; 1024];
  loop {
    let bytes_read = file.read(&mut buffer)?;
    if bytes_read == 0 {
      break;
    }
    hasher.update(&buffer[0..bytes_read]);
  }

  Ok(hex::encode(hasher.finalize()))
}