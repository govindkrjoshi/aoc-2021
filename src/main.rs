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
        "Day3" => solutions::day3::solve(),
        "Day4" => solutions::day4::solve(),
        "Day5" => solutions::day5::solve(),
        "Day6" => solutions::day6::solve(),
        "Day7" => solutions::day7::solve(),
        "Day8" => solutions::day8::solve(),
        "Day9" => solutions::day9::solve(),
        _ => println!("Solution for {} is not implemented yet!", args.day),
    }
}
