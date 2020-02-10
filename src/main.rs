mod editor;
mod termios_config;
use std::io;
use termios_config::TermiosConfig;

fn main() -> io::Result<()> {
    let mut term_config = TermiosConfig::new();
    term_config.enable_raw_mode();

    let w_and_h = editor::get_rows_and_cols()?;
    editor::clear_screen()?;
    editor::proccess_character()?;

    println!("Col: {}", w_and_h.0);
    println!("Row: {}", w_and_h.1);

    term_config.disable_raw_mode();
    Ok(())
}
