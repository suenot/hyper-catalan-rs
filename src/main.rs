use std::io::{self, Write};
use hyper_catalan_rs::{
    HighPrecFloat, HyperCatalanPolynomialSolver, evaluate_polynomial
};

fn main() {
    // Print banner
    println!("Hyper-Catalan Series Polynomial Solver (Rust)");
    println!("Based on 'A Hyper-Catalan Series Solution to Polynomial Equations, and the Geode'");
    println!("by N.J. Wildberger and K.W. Rubin");
    println!("------------------------------------------------");

    // Get polynomial degree
    print!("Enter the degree of polynomial: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let degree: usize = input.trim().parse().expect("Please enter a valid degree");
    
    // Get coefficients
    println!("Enter coefficients from c₀ to c{} (constant term first):", degree);
    
    let mut coefficients = Vec::with_capacity(degree + 1);
    for i in 0..=degree {
        print!("c{}: ", i);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let coeff: HighPrecFloat = input.trim().parse().expect("Please enter a valid number");
        
        coefficients.push(coeff);
    }
    
    // Create solver
    let mut solver = HyperCatalanPolynomialSolver::new(degree, 20);
    
    // Use Hyper-Catalan series
    match solver.solve_polynomial(&coefficients) {
        Ok(series_root) => {
            println!("Root by Hyper-Catalan series: {}", series_root);
            
            // Bootstrap with Newton's method
            print!("Enter initial guess for bootstrap method (default: {}): ", series_root);
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let initial_guess = if input.trim().is_empty() {
                series_root
            } else {
                input.trim().parse().expect("Please enter a valid number")
            };
            
            print!("Enter number of iterations for bootstrap method (default: 10): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let iterations: usize = if input.trim().is_empty() {
                10 // Default value
            } else {
                input.trim().parse().expect("Please enter a valid number")
            };
            
            const EPSILON: HighPrecFloat = 1e-15;
            let bootstrap_root = solver.bootstrap_root(&coefficients, initial_guess, iterations, EPSILON);
            println!("Root by bootstrap method: {}", bootstrap_root);
            
            // Check accuracy
            let error = evaluate_polynomial(&coefficients, bootstrap_root).abs();
            println!("Error: {}", error);
        },
        Err(e) => println!("Error: {}", e),
    }
}
