use chrono::{Local, NaiveTime};
use std::process;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
struct Cli {
    #[structopt(short, long, help = "output khal events")]
    khal: bool,

    #[structopt(short, long, help = "output todos")]
    todo: bool,

    #[structopt(
        short = "w",
        default_value = "60",
        help = "threshold for warning status in minutes"
    )]
    threshold_warn: i64,

    #[structopt(
        short = "c",
        default_value = "15",
        help = "threshold for critical status in minutes"
    )]
    threshold_crit: i64,

    #[structopt(
        short,
        long,
        default_value = "calendar",
        help = "the icon used (only names valid in i3status-rust)"
    )]
    icon: String,
}

#[derive(Debug)]
enum Status {
    Idle,
    Warn,
    Crit,
}

fn get_status(events: Vec<String>, w: i64, c: i64) -> Result<Status, String> {
    let now = Local::now().time();
    let mut event_remaining: i64 = 24 * 60;
    let mut state = Status::Idle;

    for e in events.iter() {
        let e_start = match NaiveTime::parse_from_str(e, "%H:%M") {
            Ok(s) => s,
            Err(f) => {
                let message = format!("error parsing time value: {}", f);
                return Err(message);
            }
        };
        let diff = e_start - now;
        if (diff.num_minutes() < event_remaining) && (diff.num_minutes() >= 0) {
            event_remaining = diff.num_minutes()
        }

        if event_remaining >= 0 {
            if event_remaining <= w {
                state = Status::Warn;
            }
            if event_remaining <= c {
                state = Status::Crit;
            }
        }
    }
    Ok(state)
}

fn main() {
    let args = Cli::from_args();

    if (!args.khal && !args.todo) || (args.khal && args.todo) {
        eprintln!("Please provide either khal or todo flag.");
        Cli::clap().print_help();
        println!();
        process::exit(1);
    }

    let now = Local::now().time();
    let from = now.format("%H:%M").to_string();

    if args.khal {
        let cmd = Command::new("khal")
            .arg("list")
            .arg("-df")
            .arg("{name}")
            .arg("-f")
            .arg("{start-time}")
            .arg("--notstarted")
            .arg(&from)
            .arg("23:59")
            .output();

        let stdout = match cmd {
            Ok(o) => o.stdout,
            Err(e) => {
                eprintln!("Error running khal: {}", e);
                process::exit(1);
            }
        };

        let output = match String::from_utf8(stdout) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Converting output failed: {}", e);
                process::exit(1);
            }
        };

        let mut out_iter = output.lines();

        let dayline = match out_iter.nth(0) {
            Some(d) => d,
            None => {
                eprintln!("Output seems empty. Exiting");
                process::exit(1);
            }
        };

        let mut events: Vec<String> = Vec::new();
        if dayline.trim() == "Today" {
            for e in out_iter {
                events.push(e.to_string());
            }
        }
        let count = events.len();
        let state = match get_status(events, args.threshold_warn, args.threshold_crit) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };

        println!(
            "{{ \"icon\": \"{}\", \"state\": \"{:?}\", \"text\": \"{}\" }}",
            args.icon, state, count
        );
    } else if args.todo {
        println!("todo output");
    }
}
