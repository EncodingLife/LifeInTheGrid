use super::element::ElementProperties;

// #10ADAD
pub struct Shannon(u8);

// #BD3939
pub struct Replica();

#[derive(Copy,Clone)]
pub enum Element {
    Shannon(u8),
    Replica()
}

impl Element {
    pub fn mass(&self) -> u32 {
        match self {
            Element::Shannon(_) => Shannon::mass(),
            Element::Replica() => Replica::mass(),
        }
    }
}

impl ElementProperties for Shannon {
    #[inline]
    fn mass() -> u32 {
        2
    }

    #[inline]
    fn size() -> u32 {
        1
    }

    #[inline]
    fn capacity() -> u32 {
        1
    }
}

impl ElementProperties for Replica {
    #[inline]
    fn mass() -> u32 {
        2
    }

    #[inline]
    fn size() -> u32 {
        1
    }

    #[inline]
    fn capacity() -> u32 {
        1
    }
}