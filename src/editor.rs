use std::char;
use std::io::prelude::*;
use std::io::{self, stdin, stdout, Write};

// Terminal commands
const CLEAR_SCREEN: &'static [u8; 4] = b"\x1b[2J";
const CURSOR_POSITION_TOP_LEFT: &'static [u8; 3] = b"\x1b[H";
const CURSOR_POSTION_BOTTOM_RIGHT: &'static [u8; 12] = b"\x1b[999C\x1b[999B";
const CURSOR_POSTION: &'static [u8; 4] = b"\x1b[6n";

pub fn clear_screen() -> io::Result<()> {
  stdout().write_all(CLEAR_SCREEN)?;
  stdout().write_all(CURSOR_POSITION_TOP_LEFT)?;

  draw_rows()?;
  stdout().write_all(CURSOR_POSITION_TOP_LEFT)?;
  Ok(())
}

pub fn proccess_character() -> Result<(), std::io::Error> {
  loop {
    // Handle error kind EOF
    let key_character = read_character().unwrap_or('\0' as char);
    let as_ascii_code = key_character as i8;

    match key_character {
      'q' => {
        stdout().write_all(CLEAR_SCREEN)?;
        stdout().write_all(CURSOR_POSITION_TOP_LEFT)?;
        break;
      }
      key_character if key_character != '\0' => {
        println!(" {}, code: {}\r", key_character, as_ascii_code)
      }
      _ => continue,
    };
  }

  Ok(())
}

pub fn get_rows_and_cols() -> std::io::Result<CursorPos> {
  stdout().write_all(CURSOR_POSTION_BOTTOM_RIGHT)?;
  stdout().flush()?;

  // Get cursor position
  let cursor_position = get_cursor_postion()?;

  Ok(cursor_position)
}

enum PosDelimiter {
  FstPos,
  SndPos,
  End,
}

type PostionPart = Option<PosDelimiter>;

pub struct CursorPos(pub i64, pub i64);

fn get_cursor_postion() -> Result<CursorPos, std::io::Error> {
  // Query cursor postion
  stdout().write_all(CURSOR_POSTION)?;
  stdout().flush()?;

  let mut cursor_pos = Vec::new();

  // Read cursor position characters
  stdin().read_to_end(&mut cursor_pos)?;

  Ok(parse_cursor_position_chars(cursor_pos))
}

fn parse_cursor_position_chars(cursor_pos: Vec<u8>) -> CursorPos {
  let mut current_symbol: PostionPart = None;
  let mut col = String::new();
  let mut row = String::new();

  for character in cursor_pos {
    let character_char = character as char;

    match character_char {
      '[' => current_symbol = PostionPart::Some(PosDelimiter::FstPos),
      ';' => current_symbol = PostionPart::Some(PosDelimiter::SndPos),
      'R' => current_symbol = PostionPart::Some(PosDelimiter::End),
      character_char if character_char.is_ascii_digit() => match &current_symbol {
        Some(curr) => match curr {
          PosDelimiter::FstPos => col.push(character_char),
          PosDelimiter::SndPos => row.push(character_char),
          PosDelimiter::End => break,
        },
        None => continue,
      },
      _ => continue,
    }
  }

  let col: i64 = col.parse().unwrap();
  let row: i64 = row.parse().unwrap();

  CursorPos(col, row)
}

fn draw_rows() -> io::Result<()> {
  for _ in 0..24 {
    stdout().write_all(b"~\r\n")?;
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_cursor_position() {
    // [123;44R -> CursorPos(123, 44)
    // [0;0R -> CursorPos(0, 0)
    // [re;thR -> ?
    let pos_bytes: Vec<u8> = "[123;44R".bytes().collect();
    let parsed_position = parse_cursor_position_chars(pos_bytes);

    assert_eq!(parsed_position.1, 44);
  }
}
