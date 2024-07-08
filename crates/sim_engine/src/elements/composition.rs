use std::iter::Sum;

use crate::compounds::Compound;

use super::{element::ElementProperties, elements::{Replica, Shannon}, operations::CompositionOperations};

#[derive(Copy, Clone, Default)]
pub struct CompoundComposition {
    shannon: u8,
    replica: u8,
    compounds: [Compound;8]
}

impl CompositionOperations for CompoundComposition {
    fn total_mass(&self) -> u32 {
        let compound_mass = self.compounds.iter().fold(0u32, |a,b| a + b.total_mass());
        let s: u32 = self.shannon as u32 * Shannon::mass();
        let r: u32= self.replica as u32 * Shannon::mass();
        compound_mass + s + r
    }
}
