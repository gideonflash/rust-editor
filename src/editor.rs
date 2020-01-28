use std::char;
use std::io::prelude::*;
use std::io::stdin;

pub fn proccess_character() -> Result<(), std::io::Error> {
  loop {
    let key_character = read_character()?;
    let as_ascii_code = key_character as i8;

    match key_character {
      key_character if key_character.is_ascii_control() => {
        println!("control code: {}\r", as_ascii_code)
      }
      'q' => break,
      key_character => println!(
        "printable char: {}, code: {}\r",
        key_character, as_ascii_code
      ),
    };
  }

  Ok(())
}

fn read_character() -> Result<char, std::io::Error> {
  let mut char_buffer = [0; 1];

  match stdin().read_exact(&mut char_buffer) {
    Ok(_) => Ok(char_buffer[0] as char),
    Err(error) => Err(error),
  }
}
