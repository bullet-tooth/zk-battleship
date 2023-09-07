use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::prelude::{AllocVar, EqGadget};
use ark_r1cs_std::uint8::UInt8;
use ark_r1cs_std::R1CSVar;
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystem, ConstraintSystemRef, SynthesisError,
};

use crate::game::field::{Battlefield, FIELD_SIZE};
use crate::game::ship::ShipType;
use crate::zkp::types::{BattlefieldVar, ConstraintF, ShipTypeVar};

/// ZK-SNARKS circuit implementation for the Battleship game.
/// It takes a list of ships as a public input and verifies the correctness of their presence
/// in the battlefield provided by the prover as a private input. I.e. it verifies that the field
/// was set according to the game rules without exposing the field information
pub struct BattleshipVerification {
    /// The public inputs to the circuit.
    pub ships: Vec<ShipType>,

    /// The private inputs to the circuit i.e. witness
    pub field: Battlefield,
}

impl BattleshipVerification {
    pub fn verify_cs(self) {
        println!("Checking a correctness of the battleship field...");
        let cs = ConstraintSystem::new_ref();
        self.generate_constraints(cs.clone()).unwrap();

        // Let's check whether the constraint system is satisfied
        let is_satisfied = cs.is_satisfied().unwrap();

        if !is_satisfied {
            // If it isn't, find out the offending constraint.
            println!("{:?}", cs.which_is_unsatisfied());
        }

        assert!(is_satisfied);
        println!("Battleship field is correct!");
    }
}

impl ConstraintSynthesizer<ConstraintF> for BattleshipVerification {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let field =
            BattlefieldVar::new_witness(ark_relations::ns!(cs, "field"), || Ok(self.field))?;

        for s in self.ships.into_iter() {
            let ship = ShipTypeVar::new_input(ark_relations::ns!(cs, "ship"), || Ok(s))?;
            let ships_count = count_ship_in_field(&field, &ship)?;
            let actual_ships =
                UInt8::new_input(ark_relations::ns!(cs, "actual ships"), || Ok(ships_count))?;

            actual_ships
                .is_eq(&ship.count)?
                .enforce_equal(&Boolean::TRUE)?;
        }

        Ok(())
    }
}

fn count_ship_in_field(field: &BattlefieldVar, ship: &ShipTypeVar) -> Result<u8, SynthesisError> {
    let ship_size = ship.ship_size.value()?;
    let ships_found = if ship_size == 1 {
        // lookup for 1-sized ships
        let mut ships_found = 0;
        for x in 0..FIELD_SIZE {
            for y in 0..FIELD_SIZE {
                match field[x][y].value()? {
                    0 => {}
                    1 => {
                        if !has_neighbors(field, x, y)? {
                            ships_found += 1;
                        }
                    }
                    _ => return Err(SynthesisError::AssignmentMissing),
                }
            }
        }
        ships_found
    } else {
        // lookup for 2..4-sized ships
        let mut ships_found = 0;
        // check horizontal lines
        for x in 0..FIELD_SIZE {
            let mut current_ship_size = 0;
            for y in 0..FIELD_SIZE {
                match field[x][y].value()? {
                    0 => {
                        if current_ship_size == ship_size {
                            ships_found += 1;
                        }
                        current_ship_size = 0;
                    }

                    1 => current_ship_size += 1,
                    _ => return Err(SynthesisError::AssignmentMissing),
                }
            }
            // check if the ship is in the end of line
            if current_ship_size == ship_size {
                ships_found += 1;
            }
        }

        // check vertical lines
        for y in 0..FIELD_SIZE {
            let mut current_ship_size = 0;
            for x in 0..FIELD_SIZE {
                match field[x][y].value()? {
                    0 => {
                        if current_ship_size == ship_size {
                            ships_found += 1;
                        }
                        current_ship_size = 0;
                    }
                    1 => current_ship_size += 1,
                    _ => return Err(SynthesisError::AssignmentMissing),
                }
            }
            // check if the ship is in the end of line
            if current_ship_size == ship_size {
                ships_found += 1;
            }
        }
        ships_found
    };

    Ok(ships_found)
}

fn has_neighbors(field: &BattlefieldVar, x: usize, y: usize) -> Result<bool, SynthesisError> {
    let bounds = 0..(FIELD_SIZE as isize);

    for direction in &[(0, 1), (0, -1), (-1, 0), (1, 0)] {
        // Indices cannot be negative or >= FIELD_SIZE.
        let neighbors_x = x as isize + direction.0;
        let neighbors_y = y as isize + direction.1;

        if bounds.contains(&neighbors_x) && bounds.contains(&neighbors_y) {
            let value = field[neighbors_x as usize][neighbors_y as usize].value()?;

            if value == 1 {
                return Ok(true); // neighbors ship element found
            }
        }
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use crate::game::field::{Battlefield, CellType, FIELD_SIZE};
    use crate::game::ship::ShipType;
    use crate::zkp::battleship_verification::BattleshipVerification;

    #[test]
    fn battleship_constraints_correctness() {
        let mut field = [CellType::Empty; FIELD_SIZE * FIELD_SIZE];
        // 4 ship-1
        field[00] = CellType::Occupied;
        field[55] = CellType::Occupied;
        field[17] = CellType::Occupied;
        field[99] = CellType::Occupied;

        // 1 ship-2
        field[39] = CellType::Occupied;
        field[49] = CellType::Occupied;

        let circuit = BattleshipVerification {
            field: Battlefield(field),
            ships: vec![
                ShipType {
                    ship_size: 1,
                    count: 4,
                },
                ShipType {
                    ship_size: 2,
                    count: 1,
                },
            ],
        };

        circuit.verify_cs();
    }
}
