//! Corpus Dispersion Analysis Library
//!
//! A high-performance Python extension for advanced lexical dispersion metrics,
//! powered by Rust and PyO3.

use pyo3::Bound;
use pyo3::prelude::*;
use pyo3::types::PyModule;

// Public modules
pub mod analyzer;
pub mod metrics;

// Re-exports for convenience
pub use analyzer::CorpusWordAnalyzer;
pub use metrics::DispersionMetrics;

/// Python module definition
#[pymodule]
fn corpus_dispersion(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CorpusWordAnalyzer>()?;
    m.add_class::<DispersionMetrics>()?;
    Ok(())
}
