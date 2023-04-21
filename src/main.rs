use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{env, io, thread::sleep, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

enum Status {Menu, Timer, Break}

struct Timer {
    minutes: u16,
    seconds: u16,
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut current_time: &mut Timer) -> io::Result<()> {
    loop {
        if current_time.minutes == 0 && current_time.seconds == 0 {
            return Ok(());
        } else if current_time.seconds == 60 {
            current_time.minutes -= 1;
        } else if current_time.seconds == 0 {
            current_time.minutes -= 1;
            current_time.seconds += 60;
        }

        terminal.draw(|f| ui(f, &mut current_time))?;

        /*
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
        */
        sleep(Duration::new(1, 0));
        current_time.seconds -= 1;
    }
}

// need to initialize 3 vertical cencered chunks
// inside middle one should be promt or timer
// check in run_app for user key_presses
// if everything is fine timer += 1
// render time -= 1
fn ui<B: Backend>(f: &mut Frame<B>, current_time: &mut Timer) {
    // divide terminal into 3 vertical parts
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(32),
                Constraint::Percentage(39),
                //Constraint::Percentage(30),
            ]
            .as_ref(),
        )
        .split(f.size());

    // outer border
    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, f.size());

    // middle element
    let block = Block::default().borders(Borders::NONE);
    f.render_widget(block, chunks[1]);

    // text
    let output_text = Paragraph::new(current_time.minutes.to_string() +":" + &current_time.seconds.to_string())
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
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut timer = Timer {
        minutes: 25,
        seconds: 0,
    };

    // run the app
    let res = run_app(&mut terminal, &mut timer);

    // return terminal to default state
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
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
