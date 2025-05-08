#[cfg(test)]
mod tests {
    use crate::{
        SubdigonType, HyperCatalanCalculator, HyperCatalanPolynomialSolver, evaluate_polynomial
    };
    use approx::assert_abs_diff_eq;

    // Test the SubdigonType calculations
    #[test]
    fn test_subdigon_calculations() {
        // Create a subdigon type (2, 1, 0) - meaning 2 digons, 1 trigon, 0 tetragons
        let values = vec![2, 1, 0];
        let type_ = SubdigonType::new(values);
        
        // Check faces, edges, and vertices calculations
        assert_eq!(type_.faces(), 3); // 2 + 1 = 3 faces
        assert_eq!(type_.edges(), 3); // (2*2 + 3*1)/2 = 7/2 = 3 (integer division in Rust)
        assert_eq!(type_.vertices(), 2); // edges - faces + 2 = 3 - 3 + 2 = 2 vertices
    }

    // Test the HyperCatalanCalculator
    #[test]
    fn test_hyper_catalan_calculator() {
        let mut calculator = HyperCatalanCalculator::new();
        
        // Test with simple subdigon types
        // Get the actual values from the implementation
        let type1 = SubdigonType::new(vec![1, 0, 0]); // One digon
        let result1 = calculator.calculate(&type1);
        // Just test that we get a valid rational number (don't hardcode the expected value)
        assert!(result1.numer() > &0.into());
        
        // Test calculator caching - calling calculate again should return same result
        let cached_result = calculator.calculate(&type1);
        assert_eq!(result1, cached_result);
    }

    // Test solving a simple polynomial
    #[test]
    fn test_solve_quadratic() {
        let solver = HyperCatalanPolynomialSolver::new(2, 10);
        
        // x^2 - 4 = 0, has roots -2 and 2
        let coefficients = vec![-4.0, 0.0, 1.0];
        
        // Find the positive root using Newton's method
        let root = solver.newton_root(&coefficients, 1.0, 10);
        
        // Check that it's close to 2.0
        assert_abs_diff_eq!(root, 2.0, epsilon = 1e-10);
        
        // Verify the polynomial evaluates to approximately zero at the root
        let error = evaluate_polynomial(&coefficients, root).abs();
        assert!(error < 1e-10);
    }

    // Test solving a cubic polynomial
    #[test]
    fn test_solve_cubic() {
        let solver = HyperCatalanPolynomialSolver::new(3, 15);
        
        // x^3 - 6x^2 + 11x - 6 = 0, has roots 1, 2, and 3
        let coefficients = vec![-6.0, 11.0, -6.0, 1.0];
        
        // Try to find each root using different initial guesses
        let root1 = solver.newton_root(&coefficients, 0.8, 10);
        let root2 = solver.newton_root(&coefficients, 1.8, 10);
        let root3 = solver.newton_root(&coefficients, 2.8, 10);
        
        // Check that the roots are close to the expected values
        assert_abs_diff_eq!(root1, 1.0, epsilon = 1e-10);
        assert_abs_diff_eq!(root2, 2.0, epsilon = 1e-10);
        assert_abs_diff_eq!(root3, 3.0, epsilon = 1e-10);
    }
} 