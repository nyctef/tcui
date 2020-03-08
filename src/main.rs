mod teamcity;

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
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols::{
        block::{FULL, HALF},
        DOT,
    },
    widgets::{Block, Borders, Gauge, Row, Table},
    Terminal,
};

enum Event<I> {
    Input(I),
    Tick,
}

fn draw<B: tui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<(), failure::Error> {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(2); 4].as_ref())
            .split(f.size());

        let mut gauge1 = Gauge::default()
            // todo: put title to left of progress bar?
            .block(Block::default().title("build 1"))
            .label("doing ok")
            .style(Style::default().fg(Color::Green).bg(Color::DarkGray))
            .percent(25);

        f.render(&mut gauge1, chunks[0]);
    })?;

    Ok(())
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
        draw(&mut terminal)?;
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
