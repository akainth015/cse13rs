use std::process::exit;
use asgn1::{PIG, prompt};
use asgn1::NAMES;
use rand::prelude::*;
use rand::rngs::StdRng;
use asgn1::Position::*;

struct Options {
    player_count: usize,
    random_seed: u64,
}

/// Read all options from standard input and return the corresponding struct
fn get_options() -> Options {
    let player_count: usize = match prompt("How many players? ") {
        Ok(player_count) => {
            if 2 <= player_count && player_count <= 10 {
                player_count
            } else {
                eprintln!("Invalid number of players. Using 2 instead.");
                2
            }
        }
        Err(_) => {
            eprintln!("Invalid number of players. Using 2 instead.");
            2
        }
    };
    let random_seed: u64 = match prompt("Random seed: ") {
        Ok(random_seed) => random_seed,
        Err(_) => {
            eprintln!("Invalid random seed. Using 2021 instead.");
            2021
        }
    };

    Options { player_count, random_seed }
}

fn main() {
    let options = get_options();

    let mut rng = StdRng::seed_from_u64(options.random_seed);
    // Some elements of this array may never get used; that is fine since it is small
    let mut points: [u32; 10] = [0; 10];
    let mut player = 0;

    loop {
        print!("{} rolls the pig...", NAMES[player]);
        let random_number = rng.gen_range(0..7);
        let mut roll = PIG[random_number];

        while roll != SIDE {
            let position = match roll {
                SIDE => "on side",
                RAZORBACK => "on back",
                TROTTER => "upright",
                SNOUTER => "on snout",
                JOWLER => "on tail",
            };
            print!(" pig lands {}", position);
            points[player] += match roll {
                SIDE => 0,
                RAZORBACK => 10,
                TROTTER => 10,
                SNOUTER => 15,
                JOWLER => 5,
            };

            if points[player] >= 100 {
                println!();
                println!("{} wins with {} points!", NAMES[player], points[player]);
                exit(0);
            }
            let random_number = rng.gen_range(0..7);
            roll = PIG[random_number];
        }
        println!(" pig lands on side");
        player = (player + 1) % options.player_count;
    }
}
