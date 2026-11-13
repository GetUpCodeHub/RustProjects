use crossterm::{
    cursor,
    execute,
    terminal::{
        self,
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::io::stdout;

pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new() -> std::io::Result<Self> {
        terminal::enable_raw_mode()?;

        let mut out = stdout();

        if let Err(e) = execute!(
            out,
            EnterAlternateScreen,
            cursor::Hide,
            Clear(ClearType::All)
        ) {
            let _ = terminal::disable_raw_mode();
            return Err(e);
        }

        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let mut out = stdout();

        let _ = execute!(
            out,
            LeaveAlternateScreen,
            cursor::Show
        );

        let _ = terminal::disable_raw_mode();
    }
}