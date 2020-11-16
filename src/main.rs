use chrono::{Local, NaiveTime};
use std::process::Command;

fn main() {
    const THRESHOLD_WARNING: i64 = 60;
    const THRESHOLD_CRITICAL: i64 = 15;
    let icon = String::from("calendar");

    let mut events: Vec<String> = Vec::new();
    let now = Local::now().time();
    let from = now.format("%H:%M").to_string();

    let khal_cmd = Command::new("khal")
        .arg("list")
        .arg("-df")
        .arg("{name}")
        .arg("-f")
        .arg("{start-time}")
        .arg("--notstarted")
        .arg(&from)
        .arg("23:59")
        .output();

    let khal_output = khal_cmd.expect("failed to run command");
    let khal_stdout = String::from_utf8(khal_output.stdout)
        .expect("can't read output");

    let mut lines = khal_stdout.lines();
    let dayline = lines.nth(0).expect("output seems empty");

    if dayline.trim() == "Today" {
        for e in lines {
            events.push(e.to_string());
        }
    }

    // get duration up to next event and set state
    let mut state = String::from("Idle");

    let mut event_remaining: i64 = 24 * 60;
    let event_count = events.len();
    for e in events.iter() {
        let e_start = match NaiveTime::parse_from_str(e, "%H:%M") {
            Ok(s) => s,
            Err(_f) => NaiveTime::from_hms(0, 0, 0),
        };
        let diff = e_start - now;
        if (diff.num_minutes() < event_remaining) && 
            (diff.num_minutes() >= 0) {
            event_remaining = diff.num_minutes()
        }

        if event_remaining >= 0 {
            if event_remaining <= THRESHOLD_WARNING {
                state = String::from("Warning");
            }
            if event_remaining <= THRESHOLD_CRITICAL {
                state = String::from("Critical");
            }
        }
    }

    println!(
        "{{ \"icon\": \"{}\", \"state\": \"{}\", \"text\": \"{}\" }}",
        icon, state, event_count
    );
}
