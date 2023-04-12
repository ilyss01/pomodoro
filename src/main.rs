use std::env;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

// TODO: proper error handling == remove unwrap's
// TODO? move to ncurses-like library for work time count, interactive pause
// Known bug: program doesn't work like it should, parsing args is the problem

struct OrganizedArgs {
    work_time: u32,
    short_break: u32,
    long_break: u32,
    long_break_interval: u32,
}

// Sleeps and prints nice output for what it's doing
fn sleep_handle(time: Duration, str: String) {
    sleep(time);
    let what_to_do: String;
    if str == "Break" {
        what_to_do = "start working".to_string();
    } else {
        what_to_do = "relax".to_string();
    }
    println!("{} time has ended, press enter to {}", str, what_to_do);
}

// Parses arguments to fill OrganizedArgs struct. Otherwise prints help/version
fn parse_args(args: Vec<String>) -> OrganizedArgs {
    let mut temp_struct = OrganizedArgs {
        work_time: 25,
        short_break: 5,
        long_break: 20,
        long_break_interval: 4,
    };
    match args.len() {
        // e.g. pomodoro
        1 => temp_struct,
        // e.g. pomodoro --help
        2 => {
            if args[1] == "--help" || args[1] == "-h" {
                println!("Help page, hi");
                exit(0);
            } else if args[1] == "--version" || args[1] == "-v" {
                println!("{}", env!("CARGO_PKG_VERSION"));
                exit(0);
            } else {
                println!("Unknown args, exiting");
                exit(-1);
            }
        },
        // when the program is executed without explicit args e.g.
        // e.g. pomodoro 25 5 15 4
        5 => OrganizedArgs {
            work_time: args[1].parse().unwrap(),
            short_break: args[2].parse().unwrap(),
            long_break: args[3].parse().unwrap(),
            long_break_interval: args[4].parse().unwrap(),
        },
        // when explicit args are given
        // e.g. pomodoro --work 25 --short 5 --long 20 --cycles 4
        9 => {
            for i in 1..args.len() {
                if args[i] == "--work" {
                    temp_struct.work_time = args[i + 1].parse().unwrap();
                } else if args[i] == "--short" {
                    temp_struct.short_break = args[i + 1].parse().unwrap();
                } else if args[i] == "--long" {
                    temp_struct.long_break = args[i + 1].parse().unwrap();
                } else if args[i] == "--cycles" {
                    temp_struct.long_break_interval = args[i + 1].parse().unwrap();
                } else {
                    println!("Can't handle arguments, exiting the program");
                    exit(-1);
                }
            }
            temp_struct
        },
        _ => {
            println!("Can't handle arguments, exiting program");
            exit(-1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let organized_args = parse_args(args);
    let mut cycle: u32 = 0;
    let work_time: Duration;
    let short_break_time: Duration;
    let long_break_time: Duration;
    let long_break_interval: u32;

    // seconds -> minutes -> duration struct
    work_time = Duration::new(60 * organized_args.work_time as u64, 0);
    short_break_time = Duration::new(60 * organized_args.short_break as u64, 0);
    long_break_time = Duration::new(60 * organized_args.long_break as u64, 0);
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
