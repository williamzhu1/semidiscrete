use std::f32::EPSILON;
use std::collections::{HashSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};

// Define a struct for ApproxEq
#[derive(Clone, Copy)]
pub struct ApproxEq(f32);

impl ApproxEq {
    pub fn new(value: f32) -> Self {
        ApproxEq(value)
    }
}

impl PartialEq for ApproxEq {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < EPSILON
    }
}

impl Eq for ApproxEq {}

// Implement Hash for ApproxEq to enable usage in HashSet
impl Hash for ApproxEq {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the value of f32 directly
        self.0.to_bits().hash(state);
    }
}

impl From<f32> for ApproxEq {
    fn from(value: f32) -> Self {
        ApproxEq(value)
    }
}