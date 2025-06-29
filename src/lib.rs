use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::Bound;
use std::f64::consts::LN_2;

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

#[pyclass]
pub struct CorpusWordAnalyzer {
    v: Vec<f64>,
    corpus_part_sizes_words: Vec<f64>,
    total_corpus_words: f64,
    n: usize,
    f: f64,
    s: Vec<f64>,
    p: Vec<f64>,
    // Cached values for efficiency
    cached_range: Option<i32>,
    cached_mean_v: Option<f64>,
    cached_mean_p: Option<f64>,
    cached_sum_p: Option<f64>,
    cached_min_s: Option<f64>,
}

#[pymethods]
impl CorpusWordAnalyzer {
    #[new]
    pub fn new(
        v: Vec<f64>,
        corpus_part_sizes_words: Vec<f64>,
        total_corpus_words: f64,
    ) -> PyResult<Self> {
        if v.len() != corpus_part_sizes_words.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Frequency vector and part sizes vector must have the same length.",
            ));
        }

        if v.is_empty() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Input vectors cannot be empty.",
            ));
        }

        if total_corpus_words <= 0.0 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Total corpus words must be positive.",
            ));
        }

        let n = v.len();
        let f = v.iter().sum();

        // Pre-compute s and p vectors
        let s: Vec<f64> = corpus_part_sizes_words
            .iter()
            .map(|&size| size / total_corpus_words)
            .collect();

        let p: Vec<f64> = v
            .iter()
            .zip(corpus_part_sizes_words.iter())
            .map(|(&freq, &size)| if size > 0.0 { freq / size } else { 0.0 })
            .collect();

        Ok(CorpusWordAnalyzer {
            v,
            corpus_part_sizes_words,
            total_corpus_words,
            n,
            f,
            s,
            p,
            cached_range: None,
            cached_mean_v: None,
            cached_mean_p: None,
            cached_sum_p: None,
            cached_min_s: None,
        })
    }

    // Optimized range calculation with caching
    pub fn get_range(&mut self) -> i32 {
        if let Some(range) = self.cached_range {
            return range;
        }
        let range = self.v.iter().filter(|&&x| x > 0.0).count() as i32;
        self.cached_range = Some(range);
        range
    }

    // Helper method to get cached mean_v
    fn get_mean_v(&mut self) -> f64 {
        if let Some(mean) = self.cached_mean_v {
            return mean;
        }
        let mean = self.f / self.n as f64;
        self.cached_mean_v = Some(mean);
        mean
    }

    // Helper method to get cached mean_p
    fn get_mean_p(&mut self) -> f64 {
        if let Some(mean) = self.cached_mean_p {
            return mean;
        }
        let mean = self.p.iter().sum::<f64>() / self.n as f64;
        self.cached_mean_p = Some(mean);
        mean
    }

    // Helper method to get cached sum_p
    fn get_sum_p(&mut self) -> f64 {
        if let Some(sum) = self.cached_sum_p {
            return sum;
        }
        let sum = self.p.iter().sum::<f64>();
        self.cached_sum_p = Some(sum);
        sum
    }

    // Helper method to get cached min_s
    fn get_min_s(&mut self) -> f64 {
        if let Some(min_s) = self.cached_min_s {
            return min_s;
        }
        let min_s = self.s.iter().cloned().fold(f64::INFINITY, f64::min);
        self.cached_min_s = Some(min_s);
        min_s
    }

    pub fn get_sd_population(&mut self) -> Option<f64> {
        if self.n == 0 {
            return None;
        }
        if self.f == 0.0 {
            return Some(0.0);
        }
        let mean_v = self.get_mean_v();
        let variance = self.v.iter().map(|&x| (x - mean_v).powi(2)).sum::<f64>() / self.n as f64;
        Some(variance.sqrt())
    }

    pub fn get_vc_population(&mut self) -> Option<f64> {
        let mean_v = self.get_mean_v();
        if mean_v.abs() < 1e-12 {
            return Some(0.0);
        }
        self.get_sd_population().map(|sd| sd / mean_v)
    }

    pub fn get_juilland_d(&mut self) -> Option<f64> {
        if self.n <= 1 {
            return Some(if self.f > 0.0 { 1.0 } else { 0.0 });
        }

        if self.f == 0.0 {
            return Some(0.0);
        }

        let mean_p = self.get_mean_p();
        if mean_p.abs() < 1e-12 {
            return Some(0.0);
        }

        let variance_p = self.p.iter().map(|&x| (x - mean_p).powi(2)).sum::<f64>() / self.n as f64;
        let sd_p = variance_p.sqrt();
        let vc_p = sd_p / mean_p;

        Some(1.0 - vc_p / ((self.n - 1) as f64).sqrt())
    }

    pub fn get_carroll_d2(&mut self) -> Option<f64> {
        if self.n <= 1 {
            return Some(if self.f > 0.0 { 1.0 } else { 0.0 });
        }

        let sum_p = self.get_sum_p();
        if sum_p.abs() < 1e-12 {
            return Some(0.0);
        }

        // Use natural log and convert to log2 for efficiency
        let entropy = self
            .p
            .iter()
            .map(|&p_i| {
                let norm_prop = p_i / sum_p;
                if norm_prop > 1e-12 {
                    -norm_prop * norm_prop.ln()
                } else {
                    0.0
                }
            })
            .sum::<f64>();

        let log2_n = (self.n as f64).ln() / LN_2;
        Some(entropy / (log2_n * LN_2))
    }

    pub fn get_roschengren_s_adj(&self) -> Option<f64> {
        if self.f == 0.0 {
            return Some(0.0);
        }
        let sum_sqrt = self
            .s
            .iter()
            .zip(self.v.iter())
            .map(|(&s_i, &v_i)| (s_i * v_i).sqrt())
            .sum::<f64>();
        Some((sum_sqrt * sum_sqrt) / self.f)
    }

    pub fn get_dp(&self) -> Option<f64> {
        if self.f == 0.0 {
            return Some(0.0);
        }

        let sum_abs_diff = self
            .v
            .iter()
            .zip(self.s.iter())
            .map(|(&v_i, &s_i)| (v_i / self.f - s_i).abs())
            .sum::<f64>();

        Some(0.5 * sum_abs_diff)
    }

    pub fn get_dp_norm(&mut self) -> Option<f64> {
        let dp = self.get_dp()?;
        let min_s = self.get_min_s();
        let denom = 1.0 - min_s;
        if denom.abs() < 1e-12 {
            return Some(0.0);
        }
        Some(dp / denom)
    }

    pub fn get_kl_divergence(&self) -> Option<f64> {
        if self.f == 0.0 {
            return Some(0.0);
        }
        let mut kl = 0.0;
        for (&v_i, &s_i) in self.v.iter().zip(self.s.iter()) {
            let p = if self.f > 0.0 { v_i / self.f } else { 0.0 };
            let q = s_i;
            if p > 0.0 && q > 0.0 {
                kl += p * (p / q).ln() / LN_2;
            }
        }
        Some(kl)
    }

    pub fn get_evenness_da(&mut self) -> Option<f64> {
        if self.n == 0 {
            return None;
        }
        if self.f == 0.0 {
            return Some(0.0);
        }
        if self.n == 1 {
            return Some(1.0);
        }

        let mean_p: f64 = self.get_mean_p();
        if mean_p.abs() < 1e-12 {
            let all_same: bool = self.p.iter().all(|&p| (p - mean_p).abs() < 1e-12);
            return Some(if all_same { 1.0 } else { 0.0 });
        }

        // Optimized pairwise difference calculation
        let mut sum_abs_diff: f64 = 0.0;
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                sum_abs_diff += (self.p[i] - self.p[j]).abs();
            }
        }

        let num_pairs = (self.n * (self.n - 1)) / 2;
        if num_pairs == 0 {
            return Some(1.0);
        }

        let avg_abs_diff = sum_abs_diff / num_pairs as f64;
        let da: f64 = 1.0 - (avg_abs_diff / (2.0 * mean_p));
        Some(da.clamp(0.0, 1.0))
    }

    pub fn get_jsd_dispersion(&self) -> Option<f64> {
        if self.f == 0.0 {
            return Some(0.0);
        }
        let p_dist: Vec<f64> = self.v.iter().map(|&v_i| v_i / self.f).collect();
        let q_dist: &Vec<f64> = &self.s;
        let m_dist: Vec<f64> = p_dist
            .iter()
            .zip(q_dist.iter())
            .map(|(&p, &q)| 0.5 * (p + q))
            .collect();
        let mut kl_pm: f64 = 0.0;
        let mut kl_qm: f64 = 0.0;
        for i in 0..self.n {
            let p: f64 = p_dist[i];
            let q: f64 = q_dist[i];
            let m: f64 = m_dist[i];
            if p > 1e-12 && m > 1e-12 {
                kl_pm += p * (p / m).ln();
            }
            if q > 1e-12 && m > 1e-12 {
                kl_qm += q * (q / m).ln();
            }
        }
        let jsd: f64 = 0.5 * (kl_pm + kl_qm);
        Some(1.0 - (jsd / LN_2).min(1.0))
    }

    pub fn get_hellinger_dispersion(&self) -> Option<f64> {
        if self.f == 0.0 {
            return Some(0.0);
        }
        let p_dist: Vec<f64> = self.v.iter().map(|&v_i| v_i / self.f).collect();
        let q_dist: &Vec<f64> = &self.s;
        let mut bc: f64 = 0.0;
        for i in 0..self.n {
            bc += (p_dist[i] * q_dist[i]).sqrt();
        }
        let bc: f64 = bc.clamp(0.0, 1.0);
        let hellinger_distance: f64 = (1.0 - bc).sqrt();
        Some(1.0 - hellinger_distance)
    }

    pub fn get_mean_text_frequency_ft(&mut self) -> Option<f64> {
        if self.n == 0 {
            return None;
        }
        Some(self.get_mean_p())
    }

    pub fn get_pervasiveness_pt(&mut self) -> Option<f64> {
        if self.n == 0 {
            return None;
        }
        Some(self.get_range() as f64 / self.n as f64)
    }

    pub fn calculate_all_metrics(&mut self) -> DispersionMetrics {
        let ft: Option<f64> = self.get_mean_text_frequency_ft();
        let pt: Option<f64> = self.get_pervasiveness_pt();
        let da: Option<f64> = self.get_evenness_da();

        DispersionMetrics {
            range: self.get_range(),
            sd_population: self.get_sd_population(),
            vc_population: self.get_vc_population(),
            juilland_d: self.get_juilland_d(),
            carroll_d2: self.get_carroll_d2(),
            roschengren_s_adj: self.get_roschengren_s_adj(),
            dp: self.get_dp(),
            dp_norm: self.get_dp_norm(),
            kl_divergence: self.get_kl_divergence(),
            jsd_dispersion: self.get_jsd_dispersion(),
            hellinger_dispersion: self.get_hellinger_dispersion(),
            mean_text_frequency_ft: ft,
            pervasiveness_pt: pt,
            evenness_da: da,
            ft_adjusted_by_pt: match (ft, pt) {
                (Some(f), Some(p)) => Some(f * p),
                _ => None,
            },
            ft_adjusted_by_da: match (ft, da) {
                (Some(f), Some(d)) => Some(f * d),
                _ => None,
            },
        }
    }

    // Enhanced batch processing with better memory management
    #[staticmethod]
    pub fn calculate_batch_metrics(
        frequency_matrix: Vec<Vec<f64>>,
        corpus_part_sizes: Vec<f64>,
        total_corpus_words: f64,
    ) -> PyResult<Vec<DispersionMetrics>> {
        use rayon::prelude::*;

        // Pre-validate inputs to avoid repeated validation in parallel processing
        if frequency_matrix.is_empty() {
            return Ok(Vec::new());
        }

        let expected_len: usize = corpus_part_sizes.len();
        for (i, row) in frequency_matrix.iter().enumerate() {
            if row.len() != expected_len {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Row {} has length {}, expected {}",
                    i,
                    row.len(),
                    expected_len
                )));
            }
        }

        frequency_matrix
            .into_par_iter()
            .map(|v: Vec<f64>| {
                let mut analyzer: CorpusWordAnalyzer =
                    CorpusWordAnalyzer::new(v, corpus_part_sizes.clone(), total_corpus_words)?;
                Ok(analyzer.calculate_all_metrics())
            })
            .collect()
    }

    // Additional utility method for single metric calculation
    #[staticmethod]
    pub fn calculate_single_metric(
        frequency_vector: Vec<f64>,
        corpus_part_sizes: Vec<f64>,
        total_corpus_words: f64,
        metric_name: &str,
    ) -> PyResult<Option<f64>> {
        let mut analyzer =
            CorpusWordAnalyzer::new(frequency_vector, corpus_part_sizes, total_corpus_words)?;

        let result = match metric_name {
            "juilland_d" => analyzer.get_juilland_d(),
            "carroll_d2" => analyzer.get_carroll_d2(),
            "dp" => analyzer.get_dp(),
            "dp_norm" => analyzer.get_dp_norm(),
            "kl_divergence" => analyzer.get_kl_divergence(),
            "jsd_dispersion" => analyzer.get_jsd_dispersion(),
            "hellinger_dispersion" => analyzer.get_hellinger_dispersion(),
            "evenness_da" => analyzer.get_evenness_da(),
            "mean_text_frequency_ft" => analyzer.get_mean_text_frequency_ft(),
            "pervasiveness_pt" => analyzer.get_pervasiveness_pt(),
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Unknown metric: {}",
                    metric_name
                )))
            }
        };

        Ok(result)
    }
}

#[pymodule]
fn corpus_dispersion(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CorpusWordAnalyzer>()?;
    m.add_class::<DispersionMetrics>()?;
    Ok(())
}
