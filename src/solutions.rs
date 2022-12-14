use solution_macro::match_solutions;
use std::io::{Error, ErrorKind};
use std::{fs, io};

mod solution_1_1;
mod solution_1_2;
mod solution_2_1;
mod solution_2_2;
mod solution_3_1;
mod solution_3_2;
mod solution_4_1;
mod solution_4_2;
mod solution_5_1;
mod solution_5_2;
mod solution_6_1;
mod solution_6_2;
mod solution_7_1;
mod solution_7_2;

pub fn get_solution(arg: &str) -> io::Result<String> {
    let input = fs::read_to_string("input")?;

    // use proc macro to generate a match for valid .rs files in /solutions
    match_solutions!();
}
