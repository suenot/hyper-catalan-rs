pub mod subdigon;
pub mod calculator;
pub mod solver;
pub mod tests;

// Re-export commonly used types
pub use solver::HighPrecFloat;
pub use solver::HyperCatalanPolynomialSolver;
pub use calculator::HyperCatalanCalculator;
pub use subdigon::SubdigonType;

// Convenience function to evaluate a polynomial at a specific point
pub fn evaluate_polynomial(coefficients: &[HighPrecFloat], x: HighPrecFloat) -> HighPrecFloat {
    let mut result = 0.0;
    for (i, &coeff) in coefficients.iter().enumerate() {
        result += coeff * x.powi(i as i32);
    }
    result
} 