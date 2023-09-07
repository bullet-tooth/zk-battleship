use std::borrow::Borrow;

use ark_r1cs_std::alloc::{AllocVar, AllocationMode};
use ark_r1cs_std::uint8::UInt8;
use ark_relations::r1cs::{Namespace, SynthesisError};

use crate::game::field::{Battlefield, CellType, FIELD_SIZE};
use crate::game::ship::ShipType;

/// The elliptic curve used by the proof system
pub type ConstraintF = ark_ed_on_bls12_381::Fq;

/// Battlefield type for the R1CS
pub type BattlefieldVar = [[UInt8<ConstraintF>; FIELD_SIZE]; FIELD_SIZE];

/// ShipType type for the R1CS
pub struct ShipTypeVar {
    pub ship_size: UInt8<ConstraintF>,
    pub count: UInt8<ConstraintF>,
}

impl AllocVar<Battlefield, ConstraintF> for BattlefieldVar {
    fn new_variable<T: Borrow<Battlefield>>(
        cs: impl Into<Namespace<ConstraintF>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError> {
        let cs = cs.into();
        f().and_then(|v| {
            let field = v.borrow().0;
            let row = [(); FIELD_SIZE].map(|_| UInt8::constant(0));
            let mut result = [(); FIELD_SIZE].map(|_| row.clone());

            for (i, cell) in field.into_iter().enumerate() {
                let value = u8::from(cell == CellType::Occupied);
                let x = i % FIELD_SIZE;
                let y = i / FIELD_SIZE;
                result[x][y] = UInt8::new_variable(cs.clone(), || Ok(value), mode)?;
            }
            Ok(result)
        })
    }
}

impl AllocVar<ShipType, ConstraintF> for ShipTypeVar {
    fn new_variable<T: Borrow<ShipType>>(
        cs: impl Into<Namespace<ConstraintF>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError> {
        let cs = cs.into();

        f().and_then(|v| {
            let ship_type = v.borrow();
            let ship_size = UInt8::new_variable(cs.clone(), || Ok(ship_type.ship_size), mode)?;
            let count = UInt8::new_variable(cs.clone(), || Ok(ship_type.count), mode)?;
            Ok(Self { ship_size, count })
        })
    }
}
