use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hyper_catalan_rs::{HyperCatalanCalculator, HyperCatalanPolynomialSolver, SubdigonType};
use num::BigRational;

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
    
    c.bench_function("hyper_catalan_numbers", |b| {
        b.iter(|| {
            for type_vec in &types {
                let result = calculator.calculate(black_box(type_vec));
                black_box(result);
            }
        })
    });
}

fn quadratic_equation_benchmark(c: &mut Criterion) {
    let solver = HyperCatalanPolynomialSolver::new(2, 15);
    
    // Define the quadratic: x² - 4 = 0 (roots: -2, 2)
    let coefficients = vec![
        BigRational::from_integer(-4.into()),   // x⁰
        BigRational::from_integer(0.into()),    // x¹
        BigRational::from_integer(1.into()),    // x²
    ];
    
    c.bench_function("quadratic_equation", |b| {
        b.iter(|| {
            let root = solver.bootstrap_root(
                black_box(&coefficients), 
                black_box(BigRational::from_float(1.0).unwrap()), 
                black_box(10)
            );
            black_box(root);
        })
    });
}

fn cubic_equation_benchmark(c: &mut Criterion) {
    let solver = HyperCatalanPolynomialSolver::new(3, 15);
    
    // Define the cubic: x³ - 6x² + 11x - 6 = 0 (roots: 1, 2, 3)
    let coefficients = vec![
        BigRational::from_integer(-6.into()),   // x⁰
        BigRational::from_integer(11.into()),   // x¹
        BigRational::from_integer(-6.into()),   // x²
        BigRational::from_integer(1.into()),    // x³
    ];
    
    c.bench_function("cubic_equation", |b| {
        b.iter(|| {
            // Series approximation
            let series_root = solver.solve_polynomial(black_box(&coefficients));
            // Refine using bootstrap
            let refined_root = solver.bootstrap_root(
                black_box(&coefficients), 
                black_box(series_root.unwrap()), 
                black_box(10)
            );
            black_box(refined_root);
        })
    });
}

fn higher_degree_equation_benchmark(c: &mut Criterion) {
    let solver = HyperCatalanPolynomialSolver::new(5, 20);
    
    // Define the higher degree equation: x^5 - x - 1 = 0
    let coefficients = vec![
        BigRational::from_integer(-1.into()),   // x⁰
        BigRational::from_integer(-1.into()),   // x¹
        BigRational::from_integer(0.into()),    // x²
        BigRational::from_integer(0.into()),    // x³
        BigRational::from_integer(0.into()),    // x⁴
        BigRational::from_integer(1.into()),    // x⁵
    ];
    
    c.bench_function("higher_degree_equation", |b| {
        b.iter(|| {
            // Series approximation
            let series_root = solver.solve_polynomial(black_box(&coefficients));
            // Refine using bootstrap
            let refined_root = solver.bootstrap_root(
                black_box(&coefficients), 
                black_box(series_root.unwrap()), 
                black_box(10)
            );
            black_box(refined_root);
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