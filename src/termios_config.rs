use termios::*;

pub struct TermiosConfig {
    pub term_settings: Termios,
    pub default_term_settings: Termios,
}

impl TermiosConfig {
    pub fn new() -> TermiosConfig {
        let termios = Termios::from_fd(0).unwrap();

        TermiosConfig {
            term_settings: termios,
            default_term_settings: termios.clone(),
        }
    }

    pub fn enable_raw_mode(&mut self) {
        let mut term_config = self.term_settings;

        term_config.c_iflag &= !(ICRNL | IXON);
        term_config.c_oflag &= !(OPOST);
        term_config.c_lflag &= !(ECHO | IEXTEN | ICANON | ISIG);
        // Handle result type proper
        tcsetattr(0, TCSAFLUSH, &term_config).unwrap();
    }

    pub fn disable_raw_mode(&self) {
        tcsetattr(0, TCSAFLUSH, &self.default_term_settings).unwrap();
    }
}
