use std::env;
use std::io;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

// TODO: play sound on ending the session?

fn print_error_message() {
    println!("Matched undocumented case, exiting program")
}

fn sleep_handle(time: Duration, str: String) {
    sleep(time);
    let what_to_do: String;
    if str == "Break" {
        what_to_do = "start working".to_string();
    } else {
        what_to_do = "chill".to_string();
    }
    println!("{} time has ended, press enter to {}", str, what_to_do);
    io::stdin();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // args are organized to [work_time, short_break, long_break, long_break_interval]
    let mut organized_args: [u64; 4] = [0; 4];
    let mut cycle: u64 = 0;
    let work_time: Duration;
    let short_break_time: Duration;
    let long_break_time: Duration;
    let long_break_interval: u64;

    match args.len() {
        // default case when the program is executed without any args e.g.
        // pomodoro
        1 => {
            organized_args = [25 * 60, 5 * 60, 15 * 60, 4];
        }
        // pomodoro 25 5 15 4
        5 => {
            for i in 0..args.len() {
                organized_args[i] = 60 * args[i].parse::<u64>().unwrap();
            }
            organized_args[3] = organized_args[3] / 60;
        }
        // pomodoro --work 25 --short 5 --long 20 --cycles 4
        9 => {
            for i in 0..args.len() {
                if args[i] == "--work" {
                    organized_args[0] = args[i + 1].parse::<u64>().unwrap();
                } else if args[i] == "--short" {
                    organized_args[1] = args[i + 1].parse::<u64>().unwrap();
                } else if args[i] == "--long" {
                    organized_args[2] = args[i + 1].parse::<u64>().unwrap();
                } else if args[i] == "--cycles" {
                    organized_args[3] = args[i + 1].parse::<u64>().unwrap();
                } else {
                    println!("Can't handle arguments, exiting the program");
                    exit(-1);
                }
            }
        }
        _ => {
            print_error_message();
            exit(-1);
        }
    }

    work_time = Duration::new(60 * organized_args[0], 0);
    short_break_time = Duration::new(60 * organized_args[1], 0);
    long_break_time = Duration::new(60 * organized_args[2], 0);
    long_break_interval = organized_args[3];

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
