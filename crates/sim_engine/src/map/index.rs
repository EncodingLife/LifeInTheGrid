use std::num::TryFromIntError;

use apis::{HexCoord, HexCoordinate, HexWorld, HexWorldShape};

pub(super) struct HexagonMapIndexer {
    radius: i32,
    upper_trap_area: i32,
}

impl HexagonMapIndexer {
    pub fn new(radius: usize) -> Self {
        let mut r = radius.try_into().unwrap();
        r = r - 1;
        Self {
            radius: r,
            upper_trap_area: area_of_trapezoid(r + 1, 1 + (r * 2), r + 1),
        }
    }

    pub fn get(&self, hex: HexCoord) -> Result<usize, TryFromIntError> {
        let (q, r, s) = hex.qrs();
        // See: https://www.redblobgames.com/x/2231-hex-1d-coordinates/
        // The calculation attempts to number cells starting with 0 in the top left cell.
        // Then traveling diagonally up and right (q+1,r-1) the numbers increase until we reach the end of the line - determined by the line length.
        // Take for example Line 1 (-RADIUS,0) -> (0,-RADIUS) aka (-3,0) -> (0,-3) for a Radius of 3. Each step up and right will increase the index value by one.
        // This works fine until we reach line 4 aka (-2,3) -> (5,-4) where the formula actually considers (-3,4) to be the start of line when we shouldn't.
        // This shifting offset is S when s < 0. To avoid this we add min(0,3) to the solution so that negative s values will offset our calculation but positive ones won't

        // position_in_line = fc + o
        let fc: i32 = self.radius + q; // Our position in the line presuming all lines start at q = -RADIUS
        let o: i32 = i32::min(0, s); // The number of cells to ignore because they will not be drawn on our map, due to being outside the radius
        let position_in_line = fc + o;

        // start_of_line = the starting index for the line we're on. For line 1 this is 0, line 2 is 4, line 3 is 9, ... etc
        let start_of_line = self.get_start_index(q, r, s);

        // Finally to get our index we take our startOfLine and add our positionInLine
        let p = start_of_line + position_in_line;
        p.try_into()
    }

    fn get_start_index(&self, q: i32, r: i32, s: i32) -> i32 {
        // if s > 0 we're in our top half
        if s >= 0 {
            let height = self.radius - s;
            area_of_trapezoid(self.radius + 1, (self.radius * 2) - s, height)
        } else {
            let longest_side = (self.radius * 2) + 1;
            let height = s.abs();
            self.upper_trap_area
                + area_of_trapezoid(longest_side, longest_side - height, height - 1)
        }
    }
}

fn area_of_trapezoid(f: i32, l: i32, rows: i32) -> i32 {
    let ff: f32 = (f as i16).try_into().unwrap();
    let lf: f32 = (l as i16).try_into().unwrap();
    let rf: f32 = (rows as i16).try_into().unwrap();

    ((ff + lf) / 2.0 * rf).trunc() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(-3,0, 0)]
    #[test_case(-2,-1, 1 )]
    #[test_case(-1,-2, 2 )]
    #[test_case(0,-3, 3 )]
    #[test_case(-3,1, 4 )]
    #[test_case(0, 0, 18)]
    #[test_case(-2,3, 22 )]
    #[test_case(1, 0, 25)]
    #[test_case(3,-2, 27 )]
    #[test_case(-1,3, 28)]
    #[test_case(0, 2, 29)]
    #[test_case(0, 3, 33)]
    #[test_case(3, 0, 36)]
    fn test_to_1d_cases(q: i32, r: i32, i: usize) {
        let indexer = HexagonMapIndexer::new(3);
        let hex = HexCoord::new(q, r, -q-r);
        let r = indexer.get(hex).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(r, i, "({q},{r}) index is {r} expected {i}");
    }

    #[test_case(0, 0)]
    #[test_case(1, 3)]
    #[test_case(2, 9)]
    #[test_case(3, 18)]
    fn center_in_variable_radius(radius: usize, expected: usize) {
        let indexer = HexagonMapIndexer::new(radius);
        let hex = HexCoord::new(0, 0, 0);
        let r = indexer.get(hex).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(r, expected, "({radius},{r}) index is {r} expected {expected}");
    }

    #[test_case(-3, 0, 0; "The start index for (neg3,0) should be 0")]
    #[test_case(-2, -1, 0; "The start index for (neg2,-1) should be 0")]
    #[test_case(-3, 1, 4; "The start index for (neg3,1) should be 4")]
    #[test_case(-3, 2, 9; "The start index for (neg3,2) should be 9")]
    #[test_case(-3, 3, 15; "The start index for (neg3,3) should be 15")]
    #[test_case(-2, 3, 22; "The start index for (neg2,3) should be 22")]
    #[test_case(-1, 3, 28; "The start index for (neg1,3) should be 28")]
    #[test_case(0, 3, 33; "The start index for (0,3) should be 33")]
    // #[test_case(0, 18; "The start index for r=0 is 18")]
    fn start_index_cases(q: i32, r: i32, e: i32) {
        let indexer = HexagonMapIndexer::new(3);
        let (q, r, s) = HexCoord::new(q, r, -q-r).qrs();
        assert_eq!(indexer.get_start_index(q, r, s), e);
    }

    #[test_case(4, 4, 0, 0; "Area of a=4,b=4,h=0 triangle is 0")]
    #[test_case(4, 4, 1, 4; "Area of a=4,b=4,h=1 triangle is 4")]
    #[test_case(4, 5, 2, 9; "Area of a=4,b=5,h=2 triangle is 9")]
    #[test_case(4, 6, 0, 0; "Area of a=6,b=6,h=0 triangle is 0")]
    #[test_case(6, 6, 1, 6; "Area of a=6,b=6,h=1 triangle is 6")]
    #[test_case(6, 5, 2, 11; "Area of a=6,b=5,h=2 triangle is 11")]
    #[test_case(6, 4, 3, 15; "Area of a=6,b=4,h=3 triangle is 15")]
    fn aOfT_cases(a: i32, b: i32, h: i32, e: i32) {
        assert_eq!(area_of_trapezoid(a, b, h), e);
    }

    #[test_case(2,0,0,3)]
    fn get(radius: usize, q: i32, r: i32, expected: usize) {
        let indexer = HexagonMapIndexer::new(radius);
        assert_eq!(indexer.get(HexCoord::new(q,r,-q-r)), Ok(expected))
    }
}
