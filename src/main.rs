#[allow(unused_variables)]
mod days;
mod utils;

use chrono::{Datelike, Utc};
use std::time::Instant;

fn main() {
    let day: u8 = Utc::now().date_naive().day() as u8;
    let day_solver = get_day_solver(day);
    let time = Instant::now();
    let (p1, p2) = day_solver(utils::get_input(day));
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    let elapsed_s = elapsed_ms / 1000.0;

    if p1 != 0 || p2 != 0 {
        println!("\n=== Day {:02} ===", day);
        println!("  · Part 1: {}", p1);
        println!("  · Part 2: {}", p2);
    }
    if elapsed_ms < 1000.0 {
        println!("  · Elapsed: {:.4} ms", elapsed_ms);
    } else {
        println!("  · Elapsed: {:.4} s", elapsed_s);
    }
}

fn get_day_solver(day: u8) -> fn(String) -> (usize, usize) {
    match day {
        1 => days::day1::solve,
        2 => days::day2::solve,
        3 => days::day3::solve,
        4 => days::day4::solve,
        5 => days::day5::solve,
        6 => days::day6::solve,
        7 => days::day7::solve,
        8 => days::day8::solve,
        9 => days::day9::solve,
        10 => days::day10::solve,
        11 => days::day11::solve,
        12 => days::day12::solve,
        _ => unimplemented!("Day Not Found"),
    }
}
