mod editor;
mod termios_config;
use std::io;
use termios_config::TermiosConfig;

fn main() -> io::Result<()> {
    let mut term_config = TermiosConfig::new();
    term_config.enable_raw_mode();

    editor::proccess_character()?;

    term_config.disable_raw_mode();
    Ok(())
}
