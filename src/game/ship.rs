use crate::game::field::XY;

/// Ship type for the game
#[derive(Copy, Clone)]
pub struct ShipType {
    /// The size of the ship, value in range [1..4]
    pub ship_size: u8,
    /// Count of ships of this type in the field
    pub count: u8,
}

/// Shape of the ship
#[derive(Copy, Clone)]
pub struct ShipShape {
    /// Ship's direction i.e. horizontal/vertical
    pub dxy: XY,
    /// Ship's size
    pub size: u8,
}

/// A ship with it's coordinates in the game field
#[derive(Copy, Clone)]
pub struct Ship {
    /// Coordinates of the ship start
    pub xy: XY,
    /// Shape of the ship
    pub shape: ShipShape,
}

#[allow(clippy::copy_iterator)]
impl Iterator for Ship {
    type Item = XY;

    fn next(&mut self) -> Option<XY> {
        if self.shape.size > 0 {
            let result = self.xy;
            self.xy.0 += self.shape.dxy.0;
            self.xy.1 += self.shape.dxy.1;
            self.shape.size -= 1;
            Some(result)
        } else {
            None
        }
    }
}
