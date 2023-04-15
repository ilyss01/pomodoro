use std::time::Duration;
use std::thread::sleep;
use std::process::exit;

pub struct OrganizedArgs {
    pub work_time: u64,
    pub short_break: u64,
    pub long_break: u64,
    pub long_break_interval: u32,
}

// Sleeps and prints nice output for what it's doing
pub fn sleep_handle(time: Duration, str: String) {
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
pub fn parse_args(args: Vec<String>) -> OrganizedArgs {
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
                println!("pomodoro {}", env!("CARGO_PKG_VERSION"));
                exit(0);
            } else {
                println!("Unknown args, exiting");
                exit(-1);
            }
        }
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
                }
            }
            temp_struct
        }
        _ => {
            println!("Can't handle arguments, exiting program");
            exit(-1);
        }
    }
}
