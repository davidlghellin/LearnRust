use serde::{Serialize, Deserialize};

pub fn add_common(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Second {
    value: u64
}

impl Second {
    // Constructs a new instance of [`Second`].
    // Note this is an associated function - no self.
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
