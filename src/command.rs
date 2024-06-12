use std::{net::TcpStream, io::Write};

use crate::store::{Store, get_value_safe, set_value_safe, delete_value_safe};

#[derive(PartialEq)]
pub enum ValidCommandType {
  GET,
  SET,
  DELETE,
}

pub struct Command {
  pub command_type: ValidCommandType,
  pub selected_key: String,
  pub insert_data: Vec<String>,
}

pub fn get_command_type(s: &str) -> Result<ValidCommandType, String> {
  match s {
    "GET" => Ok(ValidCommandType::GET),
    "SET" => Ok(ValidCommandType::SET),
    "DELETE" => Ok(ValidCommandType::DELETE),
    _ => Err("Invalid command type".to_string()),
  }
}

impl Command {
  pub fn handle_command(self, mut data: &mut Store, mut stream: TcpStream) {
    match self.command_type {
      ValidCommandType::GET => {
        let result = get_value_safe(data, self.selected_key);
        let _ = stream.write(result.as_bytes());
      }

      ValidCommandType::SET => {
        set_value_safe(&mut data, self.selected_key, self.insert_data);
        let _ = stream.write(b"Value saved!");
      }

      ValidCommandType::DELETE => {
        let delete_result: bool = delete_value_safe(&mut data, self.selected_key);
        if delete_result {
          let _ = stream.write(b"Value deleted!");
        } else {
          let _ = stream.write(b"ERROR: Invalid key!");
        }
      }
    }
  }
}