use std::error::Error;
use std::io;

use crate::game::battleship::{Battleship, SHIPS_FOR_GAME};
use crate::game::field::{FIELD_SIZE, XY};
use crate::zkp::battleship_verification::BattleshipVerification;

mod game;
mod zkp;

fn main() {
    let mut field = Battleship::generate();
    println!("{field}");

    BattleshipVerification {
        field: field.field,
        ships: SHIPS_FOR_GAME.to_vec(),
    }
    .verify_cs();

    loop {
        println!("Enter XY to fire:");
        match read_stdin_xy() {
            Ok(xy) => {
                field.shoot(xy);
                println!("{field}");
            }

            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        }
    }
}

fn read_stdin_xy() -> Result<XY, Box<dyn Error>> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    let user_input = user_input.trim();

    let coordinates = user_input.trim().parse::<usize>()?;
    if coordinates > 99 {
        Err(From::from(format!(
            "Wrong input: '{user_input}'. Expected to be coordinates in range [00..99]"
        )))
    } else {
        Ok(XY(coordinates / FIELD_SIZE, coordinates % FIELD_SIZE))
    }
}
