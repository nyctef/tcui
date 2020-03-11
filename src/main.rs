mod teamcity;
use teamcity::{download_build, Build};

mod git;
use git::get_current_branch;

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    env,
    io::{stdout, Write},
    iter,
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
    NewBuild(Build),
    Tick,
}

fn build_gauge(build: &Build) -> Gauge {
    let (build_type, percent) = match build {
        Build::Queued { build_type, .. } => (&build_type.name, 0),
        Build::Running {
            build_type,
            running_info,
            ..
        } => (&build_type.name, running_info.percentage_complete),
        Build::Finished { build_type, .. } => (&build_type.name, 100),
    };
    // TODO: figure out why this breaks if we try to merge it into the above tuple
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

fn n_lines(size: usize) -> Vec<Constraint> {
    iter::repeat(Constraint::Length(2)).take(size).collect()
}

static EMPTY_BUILD_LIST: Vec<Build> = Vec::new();

fn dependencies(build: &Build) -> &[Build] {
    let snapshot_dependencies = match build {
        Build::Queued {
            snapshot_dependencies,
            ..
        } => snapshot_dependencies,
        Build::Running {
            snapshot_dependencies,
            ..
        } => snapshot_dependencies,
        Build::Finished {
            snapshot_dependencies,
            ..
        } => snapshot_dependencies,
    };

    snapshot_dependencies
        .as_ref()
        .map(|x| &x.build)
        // TODO: is there a way to hand out an empty slice here without the global variable?
        .unwrap_or(&EMPTY_BUILD_LIST)
}

fn draw<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    build: &Build,
) -> Result<(), failure::Error> {
    terminal.draw(|mut f| {
        let deps = dependencies(build);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            // +2: +1 for top build and +1 for empty space at bottom
            .constraints(n_lines(deps.len() + 2).as_ref())
            .split(f.size());

        let mut gauge1 = build_gauge(build);
        f.render(&mut gauge1, chunks[0]);

        for (i, dep_build) in deps.iter().enumerate() {
            let mut dep_gauge = build_gauge(dep_build);
            f.render(&mut dep_gauge, chunks[i + 1]);
        }
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
    let tc_transmit = transmit_event.clone();

    // TODO: we're creating _2 copies of these variables to pass them into threads
    // is there a better way to do this?
    let branch = get_current_branch()?;
    let branch_2 = branch.clone();

    let tc_token = env::var("TCUI_TC_TOKEN").expect("TCUI_TC_TOKEN is required");
    let tc_token_2 = tc_token.clone();
    let tick_rate = 250;

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10));
        let build = download_build(
            &tc_token,
            "https://buildserver.red-gate.com",
            "RedgateChangeControl_OverallBuild",
            &branch,
        );
        build
            .and_then(|b| Ok(tc_transmit.send(Event::NewBuild(b))?))
            // TODO: is this how to do error handling here?
            .unwrap_or_else(|e| eprintln!("{:?}", e));
    });

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

    let mut latest_build = download_build(
        &tc_token_2,
        "https://buildserver.red-gate.com",
        "RedgateChangeControl_OverallBuild",
        &branch_2,
    )?;

    loop {
        draw(&mut terminal, &latest_build)?;
        match receive_event.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    break;
                }
                // KeyCode::Char(c) => app.on_key(c),
                // KeyCode::Left => app.on_left(),
                // KeyCode::Up => app.on_up(),
                // KeyCode::Right => app.on_right(),
                // KeyCode::Down => app.on_down(),
                _ => {}
            },
            Event::NewBuild(build) => {
                latest_build = build;
            }
            Event::Tick => {
                // app.on_tick();
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
