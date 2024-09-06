pub trait SaCandidate {
    /// Returns a neighbour of the current candidate
    /// Should also set the fitness/objective value of the neighbour
    fn get_neighbour(&self) -> Self;
}
