use structopt::StructOpt;
use chrono::{Local, NaiveTime};
use std::process;
use std::process::Command;

#[derive(Debug, StructOpt)]
#[structopt( author, about)]
struct Cli {
    #[structopt(short, long, help="output khal events")]
    khal: bool,
    #[structopt(short, long, help="output todos")]
    todo: bool,
}

fn main() {
    let args = Cli::from_args();

    if ( !args.khal && !args.todo ) || 
        ( args.khal && args.todo ) 
    {
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

        println!("khal output");
    } else if args.todo {
        println!("todo output");
    }


}
