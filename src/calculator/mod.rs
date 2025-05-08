use std::collections::HashMap;
use num::BigRational;
use num_bigint::BigInt;
use num_traits::One;

use crate::subdigon::SubdigonType;

/// Calculator for Hyper-Catalan numbers
#[derive(Debug, Default)]
pub struct HyperCatalanCalculator {
    cache: HashMap<SubdigonType, BigRational>,
}

impl HyperCatalanCalculator {
    /// Create a new calculator with an empty cache
    pub fn new() -> Self {
        HyperCatalanCalculator {
            cache: HashMap::new(),
        }
    }

    /// Calculate the factorial as a BigInt
    fn factorial(&self, n: i32) -> BigInt {
        if n <= 1 {
            return BigInt::one();
        }
        
        let mut result = BigInt::one();
        for i in 2..=n {
            result *= i;
        }
        result
    }

    /// Calculate the Hyper-Catalan number for a given subdigon type
    pub fn calculate(&mut self, type_: &SubdigonType) -> BigRational {
        // Check the cache first
        if let Some(cached) = self.cache.get(type_) {
            return cached.clone();
        }

        // Calculate the number of edges: 2*m₂ + 3*m₃ + 4*m₄ + ... divided by 2
        let mut e = 0;
        for (i, &count) in type_.m.iter().enumerate() {
            e += (i as i32 + 2) * count;
        }
        // Divide by 2 to account for each edge being counted twice
        e /= 2;

        // Calculate the number of vertices: 1 + m₂ + 2*m₃ + 3*m₄ + ...
        let mut v = 1;
        for (i, &count) in type_.m.iter().enumerate() {
            v += (i as i32) * count;
        }

        // Calculate the Hyper-Catalan number using the formula from Theorem 5
        let numerator = self.factorial(e);
        let mut denominator = self.factorial(v);

        // Multiply by factorial of each m_i
        for &count in &type_.m {
            if count > 0 {
                denominator *= self.factorial(count);
            }
        }

        // Create the rational number result
        let result = BigRational::new(numerator, denominator);

        // Store in cache
        self.cache.insert(type_.clone(), result.clone());

        result
    }

    /// Print the contents of the cache (for debugging)
    pub fn print_cache(&self) {
        println!("Cache contains {} entries:", self.cache.len());
        for (k, v) in &self.cache {
            println!("C_{} = {}", k.to_string(), v);
        }
    }
} 