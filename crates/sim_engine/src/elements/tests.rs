use composition::CompoundComposition;
use operations::CompositionOperations;

use super::*;


#[test]
pub fn default_compound_composition_should_have_no_mass() {
    let cc = CompoundComposition::default();

    assert_eq!(0, cc.total_mass());
}