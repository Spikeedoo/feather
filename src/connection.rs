use std::{
    net::TcpStream,
    io::{prelude::*, BufReader}
};

use crate::{
  command::{get_command_type, Command},
  store::Store
};

pub fn handle_connection(mut stream: TcpStream, mut data: &mut Store) {
  // Create a buffer reader
  let buffer_reader = BufReader::new(&mut stream);
  // Capture the "first line" of the buffer only
  let incoming_command = buffer_reader.lines().next().unwrap_or(Ok("".to_string())).unwrap_or("".to_string());
  
  // Split the incoming command into its parts
  let mut parts_of_command = incoming_command.split(' ');
  let mut command_type: &str = "";
  let selected_key: &str;
  let mut insert_data: Vec<&str> = vec![];

  // Check the command type
  match parts_of_command.next() {
      Some(ctype) => command_type = ctype,
      None => {}
  }
  if command_type.len() == 0 {
      // Return error to sender -- empty command
      let _ = stream.write(b"ERROR: Please specify a command!");
      return
  }
  if get_command_type(command_type).is_err() {
    // Return error to sender -- invalid
    let _ = stream.write(b"ERROR: Invalid command!");
    return
  }

  // Check the target key
  match parts_of_command.next() {
      Some(skey) => {
        selected_key = skey
      }
      None => {
        let _ = stream.write(b"ERROR: No key selected!");
        return
      },
  }

  // Check the target key
  match parts_of_command.next() {
      Some(ins) => insert_data = vec![vec![ins], parts_of_command.collect::<Vec<&str>>()].concat(),
      None => {},
  }
  
  let parsed_command = Command {
    command_type: get_command_type(command_type).unwrap(),
    selected_key: selected_key.to_string(),
    insert_data: insert_data.iter().map(|&s| s.to_string()).collect()
  };

  parsed_command.handle_command(&mut data, stream);

}