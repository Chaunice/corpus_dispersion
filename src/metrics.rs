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
            concat!(
                "DispersionMetrics(",
                "range={}, ",
                "sd_population={}, ",
                "vc_population={}, ",
                "juilland_d={}, ",
                "carroll_d2={}, ",
                "roschengren_s_adj={}, ",
                "dp={}, ",
                "dp_norm={}, ",
                "kl_divergence={}, ",
                "jsd_dispersion={}, ",
                "hellinger_dispersion={}, ",
                "mean_text_frequency_ft={}, ",
                "pervasiveness_pt={}, ",
                "evenness_da={}, ",
                "ft_adjusted_by_pt={}, ",
                "ft_adjusted_by_da={}",
                ")"
            ),
            self.range,
            Self::fmt_opt(self.sd_population),
            Self::fmt_opt(self.vc_population),
            Self::fmt_opt(self.juilland_d),
            Self::fmt_opt(self.carroll_d2),
            Self::fmt_opt(self.roschengren_s_adj),
            Self::fmt_opt(self.dp),
            Self::fmt_opt(self.dp_norm),
            Self::fmt_opt(self.kl_divergence),
            Self::fmt_opt(self.jsd_dispersion),
            Self::fmt_opt(self.hellinger_dispersion),
            Self::fmt_opt(self.mean_text_frequency_ft),
            Self::fmt_opt(self.pervasiveness_pt),
            Self::fmt_opt(self.evenness_da),
            Self::fmt_opt(self.ft_adjusted_by_pt),
            Self::fmt_opt(self.ft_adjusted_by_da),
        )
    }
}

impl DispersionMetrics {
    fn fmt_opt(val: Option<f64>) -> String {
        match val {
            Some(v) => format!("{v:.4}"),
            None => "None".to_string(),
        }
    }
}