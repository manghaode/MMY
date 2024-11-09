use crate::extensions::file_comparer::CompareFilesStruct;

#[derive(PartialEq)]
pub enum CommandEnum {
  COMPAREFILES,
  NONE
}

fn string_to_command(s: &String) -> Option<CommandEnum> {
  match s.to_uppercase().as_str() {
    "COMPAREFILES" => Some(CommandEnum::COMPAREFILES),
    _ => None
  }
}

pub trait CommandHandler {
  fn do_command(&mut self) -> i8;
  fn store_args(&mut self, args: &[String]) -> i8;
}

fn check_command(args: &Vec<String>) -> CommandEnum {
  if args.len() == 1 {
    return CommandEnum::NONE;
  }

  if let Some(command) = string_to_command(&args[1]) {
    return command;
  } else {
    eprintln!("InValid command: {}", &args[1]);
    return CommandEnum::NONE;
  }
}

pub fn create_command(args: &Vec<String>) -> Option<Box<dyn CommandHandler>> {
  match check_command(args) {
    CommandEnum::COMPAREFILES => {
      let mut command = Box::new(CompareFilesStruct::default());
      if command.store_args(&args[2..]) != 0 {
        eprintln!("Invaild args {:?}.", args);
        return None;
      }
      return Some(command);
    },
    CommandEnum::NONE => {None}
  }
}