
mod element;
mod operations;
#[cfg(test)]
mod tests;

pub mod composition;
pub mod elements;
pub mod compounds;

#[derive(Copy, Clone, Debug)]
pub struct ElementalComposition {
    flux: u16, // A simple chemical that
    stat: u16
}

impl Default for ElementalComposition {
    fn default() -> Self {
        Self { flux: 3, stat: 3 }
    }
}