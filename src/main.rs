use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt( author, about)]
struct Cli {
    #[structopt(short = "k", help="output khal events")]
    khal: bool,
    #[structopt(short = "t", help="output todos")]
    todo: bool,
}

fn main() {
    let args = Cli::from_args();

    if args.khal && args.todo {
        println!("khal");
    }

    println!("{:?}", args);

}
