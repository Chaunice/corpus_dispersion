//! Dispersion metrics data structures and implementations

use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct DispersionMetrics {
    #[pyo3(get)]
    pub range: i32,
    #[pyo3(get)]
    pub sd_population: Option<f64>,
    #[pyo3(get)]
    pub vc_population: Option<f64>,
    #[pyo3(get)]
    pub juilland_d: Option<f64>,
    #[pyo3(get)]
    pub carroll_d2: Option<f64>,
    #[pyo3(get)]
    pub roschengren_s_adj: Option<f64>,
    #[pyo3(get)]
    pub dp: Option<f64>,
    #[pyo3(get)]
    pub dp_norm: Option<f64>,
    #[pyo3(get)]
    pub kl_divergence: Option<f64>,
    #[pyo3(get)]
    pub jsd_dispersion: Option<f64>,
    #[pyo3(get)]
    pub hellinger_dispersion: Option<f64>,
    #[pyo3(get)]
    pub mean_text_frequency_ft: Option<f64>,
    #[pyo3(get)]
    pub pervasiveness_pt: Option<f64>,
    #[pyo3(get)]
    pub evenness_da: Option<f64>,
    #[pyo3(get)]
    pub ft_adjusted_by_pt: Option<f64>,
    #[pyo3(get)]
    pub ft_adjusted_by_da: Option<f64>,
}

#[pymethods]
impl DispersionMetrics {
    fn __repr__(&self) -> String {
        format!(
            "DispersionMetrics(range={}, juilland_d={:.3?}, carroll_d2={:.3?}, ...)",
            self.range, self.juilland_d, self.carroll_d2
        )
    }
}