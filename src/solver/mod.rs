use num::ToPrimitive;

use crate::calculator::HyperCatalanCalculator;
use crate::subdigon::SubdigonType;

/// High precision floating point type alias
pub type HighPrecFloat = f64;

/// Solver for polynomial equations using the Hyper-Catalan series
pub struct HyperCatalanPolynomialSolver {
    max_degree: usize,
    max_terms: usize,
    calculator: HyperCatalanCalculator,
    debug_mode: bool,
}

impl HyperCatalanPolynomialSolver {
    /// Create a new solver with the given maximum degree and terms
    pub fn new(max_degree: usize, max_terms: usize) -> Self {
        HyperCatalanPolynomialSolver {
            max_degree,
            max_terms,
            calculator: HyperCatalanCalculator::new(),
            debug_mode: false,
        }
    }

    /// Create a new solver with debug mode enabled
    pub fn new_with_debug(max_degree: usize, max_terms: usize) -> Self {
        HyperCatalanPolynomialSolver {
            max_degree,
            max_terms,
            calculator: HyperCatalanCalculator::new(),
            debug_mode: true,
        }
    }

    /// Set debug mode
    pub fn set_debug_mode(&mut self, debug: bool) {
        self.debug_mode = debug;
    }

    /// Generate all possible subdigon types with the given total faces and maximum polygon size
    fn generate_types(&self, total_faces: usize, max_polygon_size: usize) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let mut current = vec![0; max_polygon_size];
        self.generate_types_recursive(&mut results, &mut current, total_faces, 0, max_polygon_size);
        results
    }

    /// Recursive helper for generating subdigon types
    fn generate_types_recursive(
        &self,
        results: &mut Vec<Vec<i32>>,
        current: &mut Vec<i32>,
        remaining_faces: usize,
        index: usize,
        max_polygon_size: usize,
    ) {
        // Base case: reached the end of our vector
        if index == max_polygon_size {
            if remaining_faces == 0 {
                results.push(current.clone());
            }
            return;
        }

        // Try each possible count for the current polygon size
        for i in 0..=remaining_faces {
            current[index] = i as i32;
            self.generate_types_recursive(
                results,
                current,
                remaining_faces - i,
                index + 1,
                max_polygon_size,
            );
        }
    }

    /// Solve a polynomial in geometric form: 1 - a + t₂a² + t₃a³ + ... = 0
    fn solve_geometric_form(&mut self, t_coefficients: &[HighPrecFloat]) -> HighPrecFloat {
        let mut result = 0.0;
        let mut term_count = 0;

        if self.debug_mode {
            println!("Geometric form polynomial: 1 - a");
            for i in 2..t_coefficients.len() {
                if t_coefficients[i] != 0.0 {
                    println!(" + {}a^{}", t_coefficients[i], i);
                }
            }
            println!(" = 0");
            println!("Calculating Hyper-Catalan coefficients:");
        }

        // Iterate through all possible types of subdigons up to max_terms
        for total_faces in 0..self.max_terms {
            if self.debug_mode {
                println!("For total_faces = {}:", total_faces);
            }

            let types = self.generate_types(total_faces, self.max_degree - 1);

            if self.debug_mode {
                println!("  Generated {} subdigon types", types.len());
            }

            for type_vec in &types {
                let type_ = SubdigonType::new(type_vec.clone());

                // Calculate Hyper-Catalan number
                let c_m = self.calculator.calculate(&type_);
                
                // Convert the BigRational to f64 for further calculations
                let c_m_float = c_m.to_f64().unwrap_or(0.0);

                // Calculate product t₂^m₂ · t₃^m₃ · t₄^m₄ · ...
                let mut term_product = 1.0;
                for (i, &count) in type_.m.iter().enumerate() {
                    if count > 0 && i + 2 < t_coefficients.len() {
                        term_product *= t_coefficients[i + 2].powi(count);
                    }
                }

                let term = c_m_float * term_product;
                result += term;
                term_count += 1;

                if self.debug_mode && term.abs() > 1e-10 {
                    println!("  C_{} = {}, term = {}", type_.to_string(), c_m, term);
                }
            }
        }

        if self.debug_mode {
            println!("Total terms used: {}", term_count);
            println!("Series calculation result: {}", result);
        }

        result
    }

    /// Solve a general polynomial equation: c₀ + c₁x + c₂x² + ... = 0
    pub fn solve_polynomial(&mut self, coefficients: &[HighPrecFloat]) -> Result<HighPrecFloat, String> {
        if coefficients.len() < 2 {
            return Err("Polynomial must be at least of degree 1".to_string());
        }

        if self.debug_mode {
            println!("Original polynomial:");
            for i in (0..coefficients.len()).rev() {
                if coefficients[i] != 0.0 {
                    if i < coefficients.len() - 1 && coefficients[i] > 0.0 {
                        print!("+");
                    }

                    if i > 0 {
                        if coefficients[i] == 1.0 {
                            print!("x");
                        } else if coefficients[i] == -1.0 {
                            print!("-x");
                        } else {
                            print!("{}x", coefficients[i]);
                        }

                        if i > 1 {
                            print!("^{}", i);
                        }
                    } else {
                        print!("{}", coefficients[i]);
                    }
                    print!(" ");
                }
            }
            println!("= 0");
        }

        // Convert to geometric form: 1 - a + t₂a² + t₃a³ + ... = 0
        let mut geometric_coeffs = vec![0.0; coefficients.len()];
        geometric_coeffs[0] = 1.0; // Constant 1
        geometric_coeffs[1] = -1.0; // Coefficient for a¹

        if coefficients[1] == 0.0 {
            return Err("Coefficient for x^1 cannot be zero for geometric form conversion".to_string());
        }

        for i in 2..coefficients.len() {
            geometric_coeffs[i] = coefficients[i] / coefficients[1];
        }

        if self.debug_mode {
            println!("Conversion to geometric form:");
            println!("t₁ = -1");
            for i in 2..geometric_coeffs.len() {
                println!("t₍{}₎ = {}", i, geometric_coeffs[i]);
            }
        }

        // Solve using Hyper-Catalan series
        let root = self.solve_geometric_form(&geometric_coeffs);

        if root == 0.0 {
            if self.debug_mode {
                println!("Warning: obtained zero root in geometric form, which may lead to division by zero");
            }
            // Return some default value instead of dividing by zero
            return Ok(1.0);
        }

        // Convert back to original polynomial root
        let original_root = -coefficients[0] / (coefficients[1] * root);

        if self.debug_mode {
            println!("Root in geometric form: a = {}", root);
            println!("Root of original polynomial: x = {}", original_root);

            // Check the root
            let mut eval = 0.0;
            for (i, &coeff) in coefficients.iter().enumerate() {
                eval += coeff * original_root.powi(i as i32);
            }
            println!("Verification: P({}) = {}", original_root, eval);
        }

        Ok(original_root)
    }

    /// Bootstrap root approximation using Newton's method
    pub fn bootstrap_root(
        &self,
        coefficients: &[HighPrecFloat],
        initial_guess: HighPrecFloat,
        iterations: usize,
        epsilon: HighPrecFloat,
    ) -> HighPrecFloat {
        if self.debug_mode {
            println!("Starting Newton's method refinement:");
            println!("Initial guess: {}", initial_guess);
        }

        // Create polynomial function
        let polynomial_function = |x: HighPrecFloat| -> HighPrecFloat {
            let mut result = 0.0;
            for (i, &coeff) in coefficients.iter().enumerate() {
                result += coeff * x.powi(i as i32);
            }
            result
        };

        // Create derivative function
        let derivative_function = |x: HighPrecFloat| -> HighPrecFloat {
            let mut result = 0.0;
            for i in 1..coefficients.len() {
                result += (i as HighPrecFloat) * coefficients[i] * x.powi((i as i32) - 1);
            }
            result
        };

        // Apply Newton's method
        let mut x = initial_guess;
        for i in 0..iterations {
            let f_x = polynomial_function(x);
            let df_x = derivative_function(x);

            if df_x.abs() < epsilon {
                if self.debug_mode {
                    println!("Iteration {}: derivative near zero, stopping", i);
                }
                break;
            }

            let delta = f_x / df_x;
            let new_x = x - delta;

            if self.debug_mode {
                println!(
                    "Iteration {}: x = {}, f(x) = {}, f'(x) = {}, delta = {}, new_x = {}",
                    i, x, f_x, df_x, delta, new_x
                );
            }

            x = new_x;

            if f_x.abs() < epsilon {
                if self.debug_mode {
                    println!("Iteration {}: function value near zero, stopping", i);
                }
                break;
            }

            if delta.abs() < epsilon {
                if self.debug_mode {
                    println!("Iteration {}: change too small, stopping", i);
                }
                break;
            }
        }

        if self.debug_mode {
            let final_error = polynomial_function(x).abs();
            println!("Final root value: {}", x);
            println!("Error: {}", final_error);
        }

        x
    }

    /// Find a root using only Newton's method without Hyper-Catalan series
    pub fn newton_root(
        &self,
        coefficients: &[HighPrecFloat],
        initial_guess: HighPrecFloat,
        iterations: usize,
    ) -> HighPrecFloat {
        // Default epsilon value
        const EPSILON: HighPrecFloat = 1e-15;
        self.bootstrap_root(coefficients, initial_guess, iterations, EPSILON)
    }
} 