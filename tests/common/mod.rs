//! Common test utilities and helper functions

use corpus_dispersion::*;

/// Create a basic analyzer for testing purposes
#[allow(dead_code)]
pub fn create_basic_analyzer() -> CorpusWordAnalyzer {
    let v = vec![2.0, 3.0, 5.0];
    let sizes = vec![10.0, 10.0, 10.0];
    let total = 30.0;
    CorpusWordAnalyzer::new(v, sizes, total).unwrap()
}

/// Create an analyzer with zero frequencies
#[allow(dead_code)]
pub fn create_zero_analyzer() -> CorpusWordAnalyzer {
    let v = vec![0.0, 0.0, 0.0];
    let sizes = vec![10.0, 10.0, 10.0];
    let total = 30.0;
    CorpusWordAnalyzer::new(v, sizes, total).unwrap()
}

/// Create an analyzer with uniform distribution
#[allow(dead_code)]
pub fn create_uniform_analyzer() -> CorpusWordAnalyzer {
    let v = vec![5.0, 5.0, 5.0];
    let sizes = vec![10.0, 10.0, 10.0];
    let total = 30.0;
    CorpusWordAnalyzer::new(v, sizes, total).unwrap()
}

/// Assert that a value is approximately equal to expected within tolerance
#[allow(dead_code)]
pub fn assert_approx_eq(actual: f64, expected: f64, tolerance: f64, message: &str) {
    assert!(
        (actual - expected).abs() < tolerance,
        "{}: expected {}, got {}, diff {}",
        message,
        expected,
        actual,
        (actual - expected).abs()
    );
}

/// Test data generator for edge cases
#[allow(dead_code)]
pub struct TestDataGenerator;

#[allow(dead_code)]
impl TestDataGenerator {
    /// Generate test case with extreme skew
    pub fn extreme_skew() -> (Vec<f64>, Vec<f64>, f64) {
        (
            vec![100.0, 0.0, 0.0, 0.0],
            vec![100.0, 100.0, 100.0, 100.0],
            400.0,
        )
    }

    /// Generate test case with perfect uniformity
    pub fn perfect_uniform() -> (Vec<f64>, Vec<f64>, f64) {
        (
            vec![10.0, 10.0, 10.0, 10.0],
            vec![100.0, 100.0, 100.0, 100.0],
            400.0,
        )
    }

    /// Generate test case with single occurrence
    pub fn single_occurrence() -> (Vec<f64>, Vec<f64>, f64) {
        (vec![5.0], vec![10.0], 10.0)
    }
}
