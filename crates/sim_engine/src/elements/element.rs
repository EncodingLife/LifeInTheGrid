pub trait ElementProperties {
    fn mass() -> u32; // Measure of inertia - the more mass, the harder the element is to move
    fn size() -> u32; // Measure of the amount space taken up by the element - if the sum of the mass in an area is too high elements may be moved based on their inertia
    fn capacity() -> u32; // Measure of the energy that can be stored in the element - energy can be used to initiate reactions between elements
}