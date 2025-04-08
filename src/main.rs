use clap::Parser;
use rayon::prelude::*;
use std::time::Instant;

mod neighbor;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

/// Advent of code runner.
///
/// By default runs all days, unless a specific day is chosen.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Day to run
    day: Option<usize>,
}

const DAYS: [fn() -> String; 25] = [
    day01::run_outer,
    day02::run_outer,
    day03::run_outer,
    day04::run_outer,
    day05::run_outer,
    day06::run_outer,
    day07::run_outer,
    day08::run_outer,
    day09::run_outer,
    day10::run_outer,
    day11::run_outer,
    day12::run_outer,
    day13::run_outer,
    day14::run_outer,
    day15::run_outer,
    day16::run_outer,
    day17::run_outer,
    day18::run_outer,
    day19::run_outer,
    day20::run_outer,
    day21::run_outer,
    day22::run_outer,
    day23::run_outer,
    day24::run_outer,
    day25::run_outer,
];

// const RUN_ORDER

fn main() {
    let args = Args::parse();

    if let Some(day) = args.day {
        // Just run the specified day
        if 0 == day || day >= DAYS.len() {
            eprintln!("Day {day} does not exist!");
            return;
        }

        let runner = DAYS[day - 1];
        let output = runner();
        println!("{}", output);
    } else {
        // Run all of the days
        let start = Instant::now();
        let results: Vec<String> = (0..25)
            .into_par_iter()
            .map(|i| {
                let runner = DAYS[i];
                runner()
            })
            .collect();
        let elapsed = Instant::now() - start;

        for (i, output) in results.iter().enumerate() {
            println!("Day {:02}: {}", i + 1, output);
        }

        println!("Elapsed time for all days combined: {:?}", elapsed)
    }
}
