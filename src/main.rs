use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{stdout, Write},
    sync::mpsc,
    thread,
    time::Duration,
};
use tui::{backend::CrosstermBackend, Terminal};

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), failure::Error> {
    println!("started");
    enable_raw_mode()?;
    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;

    let (transmit_event, receive_event) = mpsc::channel();

    let tick_rate = 250;

    thread::spawn(move || {
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            if event::poll(Duration::from_millis(tick_rate)).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    transmit_event.send(Event::Input(key)).unwrap();
                }
            }

            transmit_event.send(Event::Tick).unwrap();
        }
    });

    loop {
        // ui::draw(&mut terminal, &app)?;
        match receive_event.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    terminal.show_cursor()?;
                    break;
                }
                // KeyCode::Char(c) => app.on_key(c),
                // KeyCode::Left => app.on_left(),
                // KeyCode::Up => app.on_up(),
                // KeyCode::Right => app.on_right(),
                // KeyCode::Down => app.on_down(),
                _ => {}
            },
            Event::Tick => {
                // app.on_tick();
            }
        }
    }

    Ok(())
}
