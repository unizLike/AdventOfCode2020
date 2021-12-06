#![allow(dead_code)]

use advent_of_code_2021::aoc::{DayFive, Solution};

fn main() {
    let input = DayFive::input();

    let first = DayFive::solve_first(input);
    let second = DayFive::solve_second(input);

    dbg!(first);
    dbg!(second);
}
