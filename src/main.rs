mod lib;
use lib::{
    sleep_handle,
    parse_args
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{env, io, process::exit, thread, thread::sleep, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

// TODO: proper error handling == remove unwrap's
// TODO? move to ncurses-like library for work time count, interactive pause

fn main() -> Result<(), io::Error> {
    // AlternateScreen - clears up the screen and returns what was before
    // enable_raw_mode()?; // allows to disable all kb presses
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout, 
        EnterAlternateScreen, 
        DisableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // here should be the loop
    sleep_handle(Duration::new(5, 0), "Work".to_string());
    terminal.draw(|f| { // should probably move everything from here to the function
        let size = f.size();
        let block = Block::default().borders(Borders::ALL);
        f.render_widget(block,size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
                )
            .split(size);
        let create_block = |title| {
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
        };
        let text = vec![
            Spans::from("Hi first line"),
            Spans::from("hi second line")
        ];
        let paragraph = Paragraph::new(text.clone())
            .style(Style::default())
            .block(create_block("Right, wrap"))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[0]);
    })?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    disable_raw_mode()?;
    terminal.show_cursor()?;

    let args: Vec<String> = env::args().collect();
    let organized_args = parse_args(args);
    let mut cycle: u32 = 0;
    let work_time: Duration;
    let short_break_time: Duration;
    let long_break_time: Duration;
    let long_break_interval: u32;

    // seconds -> minutes -> duration struct
    work_time = Duration::new(60 * organized_args.work_time, 0);
    short_break_time = Duration::new(60 * organized_args.short_break, 0);
    long_break_time = Duration::new(60 * organized_args.long_break, 0);
    long_break_interval = organized_args.long_break_interval;

    // main loop
    loop {
        sleep_handle(work_time, "Work".to_string());

        if cycle != long_break_interval {
            sleep_handle(short_break_time, "Break".to_string());
        } else {
            sleep_handle(long_break_time, "Break".to_string());
            cycle = 0;
        }
        cycle += 1;
    }
}
