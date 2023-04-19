mod lib;
use lib::{
    sleep_handle,
    parse_args
};
use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{env, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

// need to initialize 3 vertical cencered chunks 
// inside middle one should be promt or timer
// check in run_app for user key_presses
// if everything is fine timer += 1
// render time -= 1
fn ui<B: Backend>(f: &mut Frame<B>) {
    // divide terminal into 3 vertical parts
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(39),
                Constraint::Percentage(30)
            ].as_ref()
        )
        .split(f.size());

    // outer border
    let block = Block::default()
        .borders(Borders::ALL);
    f.render_widget(block, f.size());

    // middle element
    let block = Block::default()
        .borders(Borders::NONE);
    f.render_widget(block, chunks[1]);
   
    // text
    let output_text = Paragraph::new("hello")
        .style(Style::default())
        .alignment(Alignment::Center);
    f.render_widget(output_text, chunks[1]);
}

// TODO: proper error handling == remove unwrap's
// TODO? move to ncurses-like library for work time count, interactive pause

fn main() -> Result<(), io::Error> {
    // AlternateScreen - clears up the screen and returns what was before

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout, 
        EnterAlternateScreen, 
        DisableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run the app
    let res = run_app(&mut terminal);

    // return terminal to default state
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())

    /*
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
    */
}
