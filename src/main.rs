use clap::Parser;
use chrono::Datelike;

mod traits;
mod day_one;

use traits::Solves;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    day: Option<u8>,
}

fn main() {
    let args = Args::parse();
    let day = args.day.unwrap_or_else(|| chrono::Utc::now().day() as u8);
    let solver = match day {
        1 => day_one::Solver{},
        _ => panic!("Day {} not implemented", day),
    };

    let input = solver.input();

    println!("Day {} Solving...", day);
    let now = std::time::Instant::now();
    let p1 = solver.part_one(input);
    let elapsed_p1 = now.elapsed();
    let p2 = solver.part_two(input);
    let elapsed_p2 = now.elapsed() - elapsed_p1;

    println!("Solution Part1:\t{}", p1);
    println!("Runtime:\t{:.2?}", elapsed_p1);
    println!("Solution Part2:\t{}", p2);
    println!("Runtime:\t{:.2?}", elapsed_p2);
}
