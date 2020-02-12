mod editor;
mod termios_config;
use editor::{proccess_character, Editor};
use std::io;
use termios_config::TermiosConfig;

fn main() -> io::Result<()> {
    let mut term_config = TermiosConfig::new();
    term_config.enable_raw_mode();

    let editor = Editor::new()?;
    editor.clear_screen()?;

    proccess_character()?;

    term_config.disable_raw_mode();
    Ok(())
}
