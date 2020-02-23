use std::char;
use std::io::prelude::*;
use std::io::{self, stdin, stdout, Write};

// Terminal commands
const CLEAR_SCREEN: &'static [u8; 4] = b"\x1b[2J";
const CURSOR_POSITION_TOP_LEFT: &'static [u8; 3] = b"\x1b[H";
const CURSOR_POSTION_BOTTOM_RIGHT: &'static [u8; 12] = b"\x1b[999C\x1b[999B";
const CURSOR_POSTION: &'static [u8; 4] = b"\x1b[6n";

type TerminalChars = Vec<char>;

pub struct CursorPos(pub i64, pub i64);

pub struct Editor {
    rows_cols: CursorPos,
    buffer: Vec<u8>,
}

impl Editor {
    pub fn new() -> std::io::Result<Editor> {
        let pos = get_rows_and_cols()?;

        Ok(Editor {
            rows_cols: pos,
            buffer: Vec::new(),
        })
    }

    pub fn append_to_buffer(&mut self, c: &'static str) {
        for character in c.bytes() {
            self.buffer.push(character)
        }
    }

    pub fn write(&self) -> io::Result<()> {
        stdout().write(self.buffer.as_slice())?;

        Ok(())
    }

    pub fn clear_screen(&self) -> io::Result<()> {
        stdout().write_all(CLEAR_SCREEN)?;
        stdout().write_all(CURSOR_POSITION_TOP_LEFT)?;

        draw_rows(&self.rows_cols)?;
        stdout().write_all(CURSOR_POSITION_TOP_LEFT)?;

        Ok(())
    }
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

fn get_cursor_postion() -> Result<CursorPos, std::io::Error> {
    // Query cursor postion
    stdout().write_all(CURSOR_POSTION)?;
    stdout().flush()?;

    let mut cursor_pos = String::new();

    // Read cursor position characters
    stdin().read_to_string(&mut cursor_pos)?;

    Ok(parse_cursor_position_report(cursor_pos))
}

fn parse_cursor_position_report(cursor_pos: String) -> CursorPos {
    let first = index_at('[', &cursor_pos);
    let second = index_at(';', &cursor_pos);
    let end = index_at('R', &cursor_pos);

    let as_vector: TerminalChars = cursor_pos.chars().collect();

    let col = parse_pos(&as_vector[first + 1..second]);
    let row = parse_pos(&as_vector[second + 1..end]);

    CursorPos(col, row)
}

fn index_at(at: char, collection: &String) -> usize {
    collection.char_indices().find(|v| v.1 == at).unwrap().0
}

fn parse_pos(range: &[char]) -> i64 {
    range
        .iter()
        .fold(String::from(""), |mut acc, v| {
            acc.push(*v);
            acc
        })
        .parse()
        .unwrap()
}

fn draw_rows(pos: &CursorPos) -> io::Result<()> {
    for _ in 0..pos.1 {
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
        let pos_bytes = String::from("22[123;44R");
        let parsed_position = parse_cursor_position_report(pos_bytes);

        assert_eq!(parsed_position.1, 44);
        assert_eq!(parsed_position.0, 123);
    }
}
