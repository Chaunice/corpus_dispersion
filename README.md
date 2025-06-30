# corpus_dispersion

A high-performance Python extension (wheel) for advanced lexical dispersion metrics, powered by Rust and PyO3.

## Features

- High-performance Rust backend with Python bindings
- Comprehensive set of classical and modern dispersion metrics
- Parallel batch processing for large datasets
- Optimized caching for repeated calculations
- Support for both individual and batch metric calculations

## Installation

```bash
pip install corpus-dispersion
```

## Usage

```python
import corpus_dispersion as cd

# Single word analysis
v = [1.0, 2.0, 3.0, 4.0, 5.0]  # word frequencies in each part
part_sizes = [9.0, 10.0, 10.0, 10.0, 11.0]  # total words in each part
total_words = sum(part_sizes)

analyzer = cd.CorpusWordAnalyzer(v, part_sizes, total_words)
metrics = analyzer.calculate_all_metrics()

print("Range:", metrics.range)
print("Juilland's D:", metrics.juilland_d)
print("Carroll's D2:", metrics.carroll_d2)
print("Rosengren's S_adj:", metrics.roschengren_s_adj)
print("DP:", metrics.dp)
print("DP_norm:", metrics.dp_norm)
print("KL-divergence:", metrics.kl_divergence)
print("JSD dispersion:", metrics.jsd_dispersion)
print("Hellinger dispersion:", metrics.hellinger_dispersion)
print("Mean Text Frequency (FT):", metrics.mean_text_frequency_ft)
print("Pervasiveness (PT):", metrics.pervasiveness_pt)
print("Evenness (DA):", metrics.evenness_da)
print("FT * PT:", metrics.ft_adjusted_by_pt)
print("FT * DA:", metrics.ft_adjusted_by_da)

# Batch processing for many words (recommended for large datasets)
# freq_matrix: shape (n_words, n_parts)
# part_sizes: shape (n_parts,)
# total_words: float
results = cd.CorpusWordAnalyzer.calculate_batch_metrics(
    [row.tolist() for row in freq_matrix],  # or freq_matrix.tolist() if numpy
    part_sizes.tolist(),
    float(total_words)
)
# results is a list of DispersionMetrics objects

# Single metric calculation (no need to instantiate analyzer)
jd_value = cd.CorpusWordAnalyzer.calculate_single_metric(
    [10, 5, 8], [1000, 800, 1200], 3000, "juilland_d"
)
print("Single Juilland's D:", jd_value)

# Example: analyze a word's distribution across 5 corpus parts
v = [1.0, 2.0, 3.0, 4.0, 5.0]  # word frequencies in each part
part_sizes = [9.0, 10.0, 10.0, 10.0, 11.0]  # total words in each part
total_words = sum(part_sizes)

analyzer = corpus_dispersion.CorpusWordAnalyzer(v, part_sizes, total_words)
metrics = analyzer.calculate_all_metrics()

print("Range:", metrics.range)
print("Juilland's D:", metrics.juilland_d)
print("Carroll's D2:", metrics.carroll_d2)
print("Rosengren's S_adj:", metrics.roschengren_s_adj)
print("DP:", metrics.dp)
print("DP_norm:", metrics.dp_norm)
print("KL-divergence:", metrics.kl_divergence)
print("Mean Text Frequency (FT):", metrics.mean_text_frequency_ft)
print("Pervasiveness (PT):", metrics.pervasiveness_pt)
print("Evenness (DA):", metrics.evenness_da)
print("FT * PT:", metrics.ft_adjusted_by_pt)
print("FT * DA:", metrics.ft_adjusted_by_da)

# Batch processing for many words (recommended for large datasets)
# freq_matrix: shape (n_words, n_parts)
# part_sizes: shape (n_parts,)
# total_words: float
results = corpus_dispersion.CorpusWordAnalyzer.calculate_batch_metrics(
    [row.tolist() for row in freq_matrix],  # or freq_matrix.tolist() if numpy
    part_sizes.tolist(),
    float(total_words)
)
# results is a list of DispersionMetrics objects
```

## API Overview

### Classes

- `CorpusWordAnalyzer(v, part_sizes, total_words)`: Analyze word frequency distributions across corpus partitions and compute multiple dispersion metrics.
- `DispersionMetrics`: Container for all computed dispersion metrics (read-only attributes).

### Methods

- `calculate_all_metrics() -> DispersionMetrics`: Compute and return all supported metrics.
- `get_range()`, `get_sd_population()`, `get_vc_population()`, `get_juilland_d()`, `get_carroll_d2()`, `get_roschengren_s_adj()`, `get_dp()`, `get_dp_norm()`, `get_kl_divergence()`, `get_jsd_dispersion()`, `get_hellinger_dispersion()`, `get_evenness_da()`, `get_mean_text_frequency_ft()`, `get_pervasiveness_pt()`, `ft_adjusted_by_pt`, `ft_adjusted_by_da`.

### Static Methods

- `CorpusWordAnalyzer.calculate_batch_metrics(frequency_matrix, corpus_part_sizes, total_corpus_words) -> List[DispersionMetrics]`: Efficiently compute metrics for multiple words using parallel processing.
- `CorpusWordAnalyzer.calculate_single_metric(frequency_vector, corpus_part_sizes, total_corpus_words, metric_name) -> Optional[float]`: Calculate a single specific metric by name.

### Supported Metrics

- `range`: Number of partitions containing the word
- `sd_population`: Population standard deviation of frequencies
- `vc_population`: Coefficient of variation (standard deviation / mean)
- `juilland_d`: Juilland's D dispersion index
- `carroll_d2`: Carroll's D2 entropy-based dispersion
- `roschengren_s_adj`: Rosengren's adjusted S index
- `dp`: Deviation of Proportions
- `dp_norm`: Normalized DP
- `kl_divergence`: Kullback-Leibler divergence
- `jsd_dispersion`: Jensen-Shannon divergence based dispersion
- `hellinger_dispersion`: Hellinger distance based dispersion
- `mean_text_frequency_ft`: Mean normalized frequency (FT)
- `pervasiveness_pt`: Proportion of partitions with the word (PT)
- `evenness_da`: Evenness index (DA)
- `ft_adjusted_by_pt`: FT * PT
- `ft_adjusted_by_da`: FT * DA

### Exceptions

- `PyValueError`: Raised when input parameters are invalid.

## References

This package implements the main dispersion metrics as described in:

- Th. Gries, S. (2021). Analyzing dispersion. In A practical handbook of corpus linguistics (pp. 99-118). Cham: Springer International Publishing.
- Egbert, J., & Burch, B. (2023). Which words matter most? Operationalizing lexical prevalence for rank-ordered word lists. Applied Linguistics, 44(1), 103–126. <https://doi.org/10.1093/applin/amac030>
- Carroll (1970), Juilland et al. (1970), Rosengren (1971), Biber et al. (2016), etc.

## License

MIT
