use std::io;
use std::io::Write;
use std::str::FromStr;
use crate::Position::{JOWLER, RAZORBACK, SIDE, SNOUTER, TROTTER};

/// Print the prompt to standard out and retrieve the specified value from standard input
pub fn prompt<F: FromStr>(prompt: &str) -> Result<F, F::Err> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).ok();

    input.trim().parse()
}

pub const NAMES: [&str; 10] = [
    "Wilbur",
    "Charlotte",
    "John",
    "Fern",
    "Templeton",
    "Avery",
    "Homer",
    "Henry",
    "Dr. Dorian",
    "Aranea"
];

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Position {
    SIDE,
    RAZORBACK,
    TROTTER,
    SNOUTER,
    JOWLER,
}

pub const PIG: [Position; 7] = [
    SIDE,
    SIDE,
    RAZORBACK,
    TROTTER,
    SNOUTER,
    JOWLER,
    JOWLER
];
