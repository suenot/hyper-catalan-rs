use std::hash::{Hash, Hasher};

/// Represents a subdigon type with counts of each polygon size
/// m[0] is the count of digons (2-gons)
/// m[1] is the count of trigons (3-gons)
/// m[2] is the count of tetragons (4-gons), etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubdigonType {
    pub m: Vec<i32>,
}

impl SubdigonType {
    /// Create a new SubdigonType with the given counts
    pub fn new(values: Vec<i32>) -> Self {
        SubdigonType { m: values }
    }

    /// Calculate the number of faces in the subdigon
    pub fn faces(&self) -> i32 {
        self.m.iter().sum()
    }

    /// Calculate the number of edges in the subdigon
    pub fn edges(&self) -> i32 {
        let sum: i32 = self.m.iter()
            .enumerate()
            .map(|(i, &count)| (i as i32 + 2) * count)
            .sum();
        
        sum / 2
    }

    /// Calculate the number of vertices in the subdigon
    pub fn vertices(&self) -> i32 {
        self.edges() - self.faces() + 2
    }

    /// Get a string representation for debugging
    pub fn to_string(&self) -> String {
        format!("({}))", self.m.iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(","))
    }
}

impl Hash for SubdigonType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for &value in &self.m {
            value.hash(state);
        }
    }
} 