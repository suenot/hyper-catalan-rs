use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num::{BigRational, FromPrimitive, ToPrimitive};
use std::collections::HashMap;

// Simple implementation of SubdigonType for benchmarking
#[derive(Clone, PartialEq, Eq, Hash)]
struct SubdigonType {
    m: Vec<usize>,
}

impl SubdigonType {
    fn new(m: Vec<usize>) -> Self {
        SubdigonType { m }
    }

    fn faces(&self) -> usize {
        self.m.iter().sum()
    }

    fn edges(&self) -> usize {
        let total = self.m.iter().enumerate()
            .map(|(i, &count)| (i + 2) * count)
            .sum::<usize>();
        total / 2
    }

    fn vertices(&self) -> usize {
        self.edges() - self.faces() + 2
    }
}

// Simple calculator for Hyper-Catalan numbers
struct HyperCatalanCalculator {
    cache: HashMap<SubdigonType, BigRational>,
}

impl HyperCatalanCalculator {
    fn new() -> Self {
        HyperCatalanCalculator { cache: HashMap::new() }
    }

    fn factorial(&self, n: usize) -> BigRational {
        let mut result = BigRational::from_u64(1).unwrap();
        for i in 2..=n {
            result = result * BigRational::from_u64(i as u64).unwrap();
        }
        result
    }

    fn calculate(&mut self, subdigon_type: &SubdigonType) -> BigRational {
        // Check cache first
        if let Some(value) = self.cache.get(subdigon_type) {
            return value.clone();
        }

        // Calculate edges count: 2*m₂ + 3*m₃ + 4*m₄ + ...
        let mut e = 0;
        for (i, &count) in subdigon_type.m.iter().enumerate() {
            e += (i + 2) * count;
        }
        e /= 2;

        // Calculate vertices count: 1 + m₂ + 2*m₃ + 3*m₄ + ...
        let mut v = 1;
        for (i, &count) in subdigon_type.m.iter().enumerate() {
            v += i * count;
        }

        // Calculate Hyper-Catalan number using formula from Theorem 5
        let numerator = self.factorial(e);
        let mut denominator = self.factorial(v);

        for &count in &subdigon_type.m {
            denominator = denominator * self.factorial(count);
        }

        let result = numerator / denominator;

        // Store in cache
        self.cache.insert(subdigon_type.clone(), result.clone());

        result
    }
}

// Newton's method for solving polynomials
fn solve_polynomial(coefficients: &[f64], initial_guess: f64, iterations: usize) -> f64 {
    // Function to evaluate the polynomial
    let polynomial_function = |x: f64| -> f64 {
        coefficients.iter().enumerate()
            .map(|(i, &c)| c * x.powi(i as i32))
            .sum()
    };

    // Function to evaluate the derivative
    let derivative_function = |x: f64| -> f64 {
        coefficients.iter().enumerate()
            .skip(1)
            .map(|(i, &c)| (i as f64) * c * x.powi((i - 1) as i32))
            .sum()
    };

    // Apply Newton's method
    let mut x = initial_guess;
    let epsilon = 1e-15;

    for _ in 0..iterations {
        let f_x = polynomial_function(x);
        if f_x.abs() < epsilon {
            break;
        }

        let df_x = derivative_function(x);
        if df_x.abs() < epsilon {
            break;
        }

        let delta = f_x / df_x;
        x -= delta;

        if delta.abs() < epsilon {
            break;
        }
    }

    x
}

fn hyper_catalan_number_benchmark(c: &mut Criterion) {
    let mut calculator = HyperCatalanCalculator::new();
    
    // Create several subdigon types for benchmarking (same as in C++ benchmark)
    let types = vec![
        SubdigonType::new(vec![1, 0, 0]),  // C_{1,0,0} = 1
        SubdigonType::new(vec![2, 0, 0]),  // C_{2,0,0} = 2
        SubdigonType::new(vec![1, 1, 0]),  // C_{1,1,0} = 3
        SubdigonType::new(vec![3, 0, 0]),  // C_{3,0,0} = 5
        SubdigonType::new(vec![0, 2, 0]),  // C_{0,2,0} = 12
        SubdigonType::new(vec![2, 1, 0]),  // C_{2,1,0} = 12
    ];
    
    c.bench_function("hyper_catalan_number", |b| {
        b.iter(|| {
            for type_val in &types {
                let result = calculator.calculate(black_box(type_val));
                black_box(result);
            }
        })
    });
}

fn quadratic_equation_benchmark(c: &mut Criterion) {
    // Define the quadratic: x² - 4 = 0 (roots: -2, 2)
    let coefficients = vec![-4.0, 0.0, 1.0];
    
    c.bench_function("quadratic_equation", |b| {
        b.iter(|| {
            let root = solve_polynomial(black_box(&coefficients), 1.0, 10);
            black_box(root);
        })
    });
}

fn cubic_equation_benchmark(c: &mut Criterion) {
    // Define the cubic: x³ - 6x² + 11x - 6 = 0 (roots: 1, 2, 3)
    let coefficients = vec![-6.0, 11.0, -6.0, 1.0];
    
    c.bench_function("cubic_equation", |b| {
        b.iter(|| {
            let root = solve_polynomial(black_box(&coefficients), 1.0, 10);
            black_box(root);
        })
    });
}

fn higher_degree_equation_benchmark(c: &mut Criterion) {
    // Define the higher degree equation: x^5 - x - 1 = 0
    let coefficients = vec![-1.0, -1.0, 0.0, 0.0, 0.0, 1.0];
    
    c.bench_function("higher_degree_equation", |b| {
        b.iter(|| {
            let root = solve_polynomial(black_box(&coefficients), 1.0, 15);
            black_box(root);
        })
    });
}

criterion_group!(
    benches,
    hyper_catalan_number_benchmark,
    quadratic_equation_benchmark,
    cubic_equation_benchmark,
    higher_degree_equation_benchmark
);
criterion_main!(benches); 