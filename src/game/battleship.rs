use std::fmt;
use std::ops::{Index, IndexMut};

use crate::game::field::{Battlefield, CellType, FIELD_SIZE, XY};
use crate::game::ship::{Ship, ShipShape, ShipType};
use itertools::Itertools;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

/// Battleship Game
#[derive(Default)]
pub struct Battleship {
    /// field with ships for the game
    pub field: Battlefield,
    /// array with field shoots
    pub shoots: Vec<XY>,
}

/// A list of ships required by the game rules
pub const SHIPS_FOR_GAME: [ShipType; 4] = [
    ShipType {
        ship_size: 4,
        count: 1,
    },
    ShipType {
        ship_size: 3,
        count: 2,
    },
    ShipType {
        ship_size: 2,
        count: 3,
    },
    ShipType {
        ship_size: 1,
        count: 4,
    },
];

#[rustfmt::skip]
const DIRECTIONS: [(isize, isize); 9] = [(0, 0), (0, 1), (0, -1), (-1, 0), (1, 0), (-1, 1), (1, -1), (-1, -1), (1, 1)];

impl fmt::Display for Battleship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, element) in self.field.0.iter().enumerate() {
            let xy = XY(index % FIELD_SIZE, index / FIELD_SIZE);

            // Start of line
            if xy.0 == 0 {
                writeln!(f)?;
            }

            if self.shoots.contains(&xy) {
                write!(f, "{}", element)?;
            } else {
                write!(f, "⬜️")?;
            }
        }
        Ok(())
    }
}

impl Index<XY> for Battleship {
    type Output = CellType;

    fn index(&self, xy: XY) -> &CellType {
        &self.field.0[xy.0 + xy.1 * FIELD_SIZE]
    }
}

impl IndexMut<XY> for Battleship {
    fn index_mut(&mut self, xy: XY) -> &mut CellType {
        &mut self.field.0[xy.0 + xy.1 * FIELD_SIZE]
    }
}

impl Battleship {
    fn can_place_ship(&self, ship: Ship) -> bool {
        // I. Construct a bounding box for the placed ship.
        let bounds = 0..(FIELD_SIZE as isize);
        for xy in ship {
            // Move in every box direction.
            for direction in &DIRECTIONS {
                // Indices cannot be negative or >= FIELD_SIZE.
                let bound_x = xy.0 as isize + direction.0;
                let bound_y = xy.1 as isize + direction.1;

                if bounds.contains(&bound_x) && bounds.contains(&bound_y) {
                    let bounding_box_cell = self[XY(bound_x as usize, bound_y as usize)];

                    // If there's a ship within a bounding box, halt the loop -- we cannot place the ship here.
                    if bounding_box_cell == CellType::Occupied {
                        return false;
                    }
                }
            }
        }

        // II. Check whether the cells that are being used to place the ship onto are occupied.
        let bounds = 0..FIELD_SIZE;
        for xy in ship {
            if !bounds.contains(&xy.0) || !bounds.contains(&xy.1) {
                return false;
            }
            let current_cell = self[xy];
            if current_cell == CellType::Occupied {
                return false;
            }
        }
        true
    }

    fn get_available_cells(&self, shape: ShipShape) -> Vec<XY> {
        (0..FIELD_SIZE)
            .cartesian_product(0..FIELD_SIZE)
            .map(|(x, y)| XY(x, y))
            .filter(|&xy| self.can_place_ship(Ship { xy, shape }))
            .collect()
    }

    fn emplace_ships(&mut self, size: u8, rng: &mut impl Rng) {
        // Flip a coin to determine an alignment (horizontal / vertical).
        let dxy = if rng.gen() { XY(1, 0) } else { XY(0, 1) };
        let shape = ShipShape { dxy, size };
        // Get the vector of appropriate cells.
        let cell_coordinates = self.get_available_cells(shape);
        let xy = *cell_coordinates.choose(rng).unwrap();
        let ship = Ship { xy, shape };
        // Place a ship!
        for xy in ship {
            self[xy] = CellType::Occupied;
        }
    }

    pub fn shoot(&mut self, value: XY) {
        self.shoots.push(value)
    }

    pub fn generate() -> Self {
        /* Generating the field. */
        let mut result = Self::default();
        let mut rng = StdRng::from_entropy();

        for ship_type in SHIPS_FOR_GAME {
            for _ in 0..ship_type.count {
                result.emplace_ships(ship_type.ship_size, &mut rng);
            }
        }

        result
    }
}
