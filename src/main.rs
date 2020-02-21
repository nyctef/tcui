use crossterm::terminal::enable_raw_mode;
use std::io::stdout;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> Result<(), failure::Error> {
    enable_raw_mode()?;
    let mut stdout = stdout();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    Ok(())
}
