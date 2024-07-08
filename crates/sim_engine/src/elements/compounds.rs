use super::{elements::Element, operations::CompositionOperations};

#[derive(Copy, Clone)]
pub enum Compound {
    None,
    Pair(Element, Element),
    TriChain(Element, Element, Element),
    Square(Element, Element, Element, Element),
}

impl CompositionOperations for Compound {
    fn total_mass(&self) -> u32 {
        match self {
            Compound::None => 0,
            Compound::Pair(a, b) => a.mass() + b.mass(),
            Compound::TriChain(a, b, c) => a.mass() + b.mass() + c.mass(),
            Compound::Square(a, b, c, d) => a.mass() + b.mass() + c.mass() + d.mass(),
        }
    }
}

impl Default for Compound {
    fn default() -> Self {
        Self::None
    }
}