use common::sys::create_command;

mod base;
mod common;
mod extensions;

fn usage(exector: &String) {
  eprintln!("Usage: {} COMMAND output_dir dir1 [dir2 ...]", exector);
  eprintln!("COMMAND:");
  eprintln!("\tCOMPAREFILES: \n\t\t{} COMPAREFILES output_dir dir1 [dir2 ...]", exector);
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if let Some(mut command) = create_command(&args) {
    command.do_command();
  } else {
    usage(&args[0]);
  }
}