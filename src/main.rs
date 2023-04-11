use std::env;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

// TODO: play sound on ending the session?
// TODO: proper error handling == remove unwrap's

struct OrganizedArgs {
    work_time: u32,
    short_break: u32,
    long_break: u32,
    long_break_interval: u32,
}

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut organized_args = OrganizedArgs {
        work_time: 25,
        short_break: 5,
        long_break: 15,
        long_break_interval: 4,
    };
    let mut cycle: u32 = 0;
    let work_time: Duration;
    let short_break_time: Duration;
    let long_break_time: Duration;
    let long_break_interval: u32;

    // TODO: find another way to parse arguments, put them in another function
    // TODO: add --help
    // transition from seconds to minutes will happen not there
    match args.len() {
        // default case when the program is executed without any args e.g.
        // e.g. pomodoro 25 5 15 4
        5 => {
            organized_args.work_time = args[1].parse().unwrap();
            organized_args.short_break = args[2].parse().unwrap();
            organized_args.long_break = args[3].parse().unwrap();
            organized_args.long_break_interval = args[4].parse().unwrap();
        }
        // e.g. pomodoro --work 25 --short 5 --long 20 --cycles 4
        9 => {
            for i in 0..args.len() {
                if args[i] == "--work" {
                    organized_args.work_time = args[i + 1].parse().unwrap();
                } else if args[i] == "--short" {
                    organized_args.short_break = args[i + 1].parse().unwrap();
                } else if args[i] == "--long" {
                    organized_args.long_break = args[i + 1].parse().unwrap();
                } else if args[i] == "--cycles" {
                    organized_args.long_break_interval = args[i + 1].parse().unwrap();
                } else {
                    println!("Can't handle arguments, exiting the program");
                    exit(-1);
                }
            }
        }
        _ => {
            exit(-1);
        }
    }

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
