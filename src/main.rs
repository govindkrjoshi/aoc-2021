use structopt::StructOpt;

mod solutions;

#[derive(StructOpt)]
struct CLI {
    day: String,
}

fn main() {
    let args = CLI::from_args();

    match args.day.as_ref() {
        "Day1" => solutions::day1::solve(),
        "Day2" => solutions::day2::solve(),
        _ => println!("Solution for {} is not implemented yet!", args.day),
    }
}
