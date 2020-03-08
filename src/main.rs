mod teamcity;

use teamcity::{download_build, Build};

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    env,
    io::{stdout, Write},
    sync::mpsc,
    thread,
    time::Duration,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Gauge},
    Terminal,
};

enum Event<I> {
    Input(I),
    Tick,
}

fn build_gauge(build: &Build) -> Gauge {
    let build_type = match build {
        Build::Queued { build_type, .. } => &build_type.name,
        Build::Running { build_type, .. } => &build_type.name,
        Build::Finished { build_type, .. } => &build_type.name,
    };
    let percent = match build {
        Build::Queued { .. } => 0,
        Build::Running { running_info, .. } => running_info.percentage_complete,
        Build::Finished { .. } => 100,
    };
    let label = match build {
        Build::Queued { .. } => "",
        Build::Running { status_text, .. } => &status_text,
        Build::Finished { status_text, .. } => &status_text,
    };
    Gauge::default()
        // todo: put title to left of progress bar?
        .block(Block::default().title(&build_type))
        .label(label)
        .style(Style::default().fg(Color::Green).bg(Color::DarkGray))
        .percent(percent)
}

fn draw<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    build: &Build,
) -> Result<(), failure::Error> {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(2); 4].as_ref())
            .split(f.size());

        let mut gauge1 = build_gauge(build);

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

    let tc_token = env::var("TCUI_TC_TOKEN").expect("TCUI_TC_TOKEN is required");
    let latest_build = download_build(
        &tc_token,
        "https://buildserver.red-gate.com",
        "RedgateChangeControl_OverallBuild",
        "add-beta-tag",
    )?;

    loop {
        draw(&mut terminal, &latest_build)?;
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
