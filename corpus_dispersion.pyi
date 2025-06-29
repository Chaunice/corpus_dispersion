"""
corpus_dispersion: Python bindings for advanced lexical dispersion metrics.

This module provides high-performance Rust implementations of various lexical dispersion metrics,
including classical and modern measures, for corpus linguistics and quantitative text analysis.

Features
--------
- High-performance Rust implementations with Python bindings
- Comprehensive set of dispersion metrics (classical and modern)
- Parallel batch processing for large datasets
- Optimized caching for repeated calculations
- Support for both individual and batch metric calculations

Classes
-------
CorpusWordAnalyzer
    Analyze word frequency distributions across corpus partitions and compute multiple dispersion metrics.

DispersionMetrics
    Container for all computed dispersion metrics with read-only access.

Supported Metrics
-----------------
Classical metrics:
- Range: Number of partitions containing the word
- Standard deviation and coefficient of variation
- Juilland's D: Normalized coefficient of variation
- Carroll's D2: Entropy-based dispersion
- Rosengren's S (adjusted): Weighted frequency distribution
- DP and DP normalized: Deviation of proportions

Modern metrics:
- Kullback-Leibler divergence: Information-theoretic measure
- Jensen-Shannon divergence: Symmetric version of KL divergence
- Hellinger distance: Geometric measure of distributional similarity
- Evenness (DA): Pairwise difference-based evenness
- Mean text frequency (FT): Average normalized frequency
- Pervasiveness (PT): Proportion of partitions with the word
- Adjusted metrics: FT*PT and FT*DA combinations

References
----------
- Gries, S. Th. (2020). Analyzing Dispersion. In: A Practical Handbook of Corpus Linguistics.
- Egbert, J., & Burch, B. (2023). Which words matter most? Operationalizing lexical prevalence for rank-ordered word lists. Applied Linguistics, 44(1), 103–126. <https://doi.org/10.1093/applin/amac030>
- Carroll (1970), Juilland et al. (1970), Rosengren (1971), Biber et al. (2016), etc.

Examples
--------
>>> import corpus_dispersion as cd
>>> # Single word analysis
>>> analyzer = cd.CorpusWordAnalyzer(
...     v=[10, 5, 8, 12, 3],  # word frequencies per partition
...     corpus_part_sizes_words=[1000, 800, 1200, 900, 600],  # partition sizes
...     total_corpus_words=4500
... )
>>> metrics = analyzer.calculate_all_metrics()
>>> print(f"Juilland's D: {metrics.juilland_d:.3f}")
>>>
>>> # Batch processing
>>> frequency_matrix = [[10, 5, 8], [20, 15, 12], [5, 0, 3]]
>>> batch_metrics = cd.CorpusWordAnalyzer.calculate_batch_metrics(
...     frequency_matrix, [1000, 800, 1200], 3000
... )
>>>
>>> # Single metric calculation
>>> jd_value = cd.CorpusWordAnalyzer.calculate_single_metric(
...     [10, 5, 8], [1000, 800, 1200], 3000, "juilland_d"
... )

"""

from typing import List, Optional

__version__ = "0.1.0"
__author__ = "Haobo Zhang"

__all__ = [
    "CorpusWordAnalyzer",
    "DispersionMetrics",
    "PyValueError",
]

class PyValueError(Exception):
    """Raised when input parameters are invalid."""

    ...

class DispersionMetrics:
    """
    Container for all computed dispersion metrics.

    Attributes
    ----------
    range : int
        Number of partitions in which the word occurs (range).
    sd_population : Optional[float]
        Population standard deviation of frequencies (see Gries 2020, eq. 8).
    vc_population : Optional[float]
        Coefficient of variation (standard deviation / mean).
    juilland_d : Optional[float]
        Juilland's D dispersion index (see Gries 2020, eq. 10).
    carroll_d2 : Optional[float]
        Carroll's D2 entropy-based dispersion index (see Gries 2020, eq. 11).
    roschengren_s_adj : Optional[float]
        Rosengren's adjusted S index (see Gries 2020, eq. 12).
    dp : Optional[float]
        DP (Deviation of Proportions) index (see Gries 2020, eq. 13).
    dp_norm : Optional[float]
        Normalized DP index (DP / (1 - min(s)), see Gries 2020, eq. 13).
    kl_divergence : Optional[float]
        Kullback-Leibler divergence (see Gries 2020, eq. 14).
    jsd_dispersion : Optional[float]
        Jensen-Shannon divergence based dispersion measure (1 - normalized JSD).
    hellinger_dispersion : Optional[float]
        Hellinger distance based dispersion measure (1 - normalized distance).
    mean_text_frequency_ft : Optional[float]
        Mean normalized frequency across partitions (Egbert & Burch FT).
    pervasiveness_pt : Optional[float]
        Proportion of partitions containing the word (Egbert & Burch PT).
    evenness_da : Optional[float]
        Evenness index (Egbert & Burch DA).
    ft_adjusted_by_pt : Optional[float]
        Frequency adjusted by pervasiveness (FT * PT).
    ft_adjusted_by_da : Optional[float]
        Frequency adjusted by evenness (FT * DA).
    """

    range: int
    sd_population: Optional[float]
    vc_population: Optional[float]
    juilland_d: Optional[float]
    carroll_d2: Optional[float]
    roschengren_s_adj: Optional[float]
    dp: Optional[float]
    dp_norm: Optional[float]
    kl_divergence: Optional[float]
    jsd_dispersion: Optional[float]
    hellinger_dispersion: Optional[float]
    mean_text_frequency_ft: Optional[float]
    pervasiveness_pt: Optional[float]
    evenness_da: Optional[float]
    ft_adjusted_by_pt: Optional[float]
    ft_adjusted_by_da: Optional[float]

    def __repr__(self) -> str:
        """Return string representation of the metrics."""
        ...

class CorpusWordAnalyzer:
    """
    Analyze word frequency distributions across corpus partitions and compute multiple dispersion metrics.

    This class provides comprehensive lexical dispersion analysis with optimized caching for repeated
    calculations. It supports both individual metric computation and batch processing of multiple words.

    Parameters
    ----------
    v : List[float]
        Frequency of the target word in each partition (must be non-negative).
    corpus_part_sizes_words : List[float]
        Total number of words in each partition (must be positive).
        Length must match the frequency vector.
    total_corpus_words : float
        Total number of words in the entire corpus (must be positive).
        Should equal the sum of corpus_part_sizes_words.

    Raises
    ------
    ValueError
        If input vectors have different lengths, are empty, or contain invalid values.

    Notes
    -----
    The analyzer uses internal caching to optimize repeated metric calculations.
    Metrics returning None indicate computational issues (e.g., division by zero).

    All dispersion metrics are computed based on the comparison between observed
    word distribution and expected distribution proportional to partition sizes.

    Methods
    -------
    get_range() -> int
        Number of partitions in which the word occurs (range >= 0).
    get_sd_population() -> Optional[float]
        Population standard deviation of frequencies across partitions.
    get_vc_population() -> Optional[float]
        Coefficient of variation (standard deviation / mean frequency).
    get_juilland_d() -> Optional[float]
        Juilland's D dispersion index (0 = maximally dispersed, 1 = perfectly even).
    get_carroll_d2() -> Optional[float]
        Carroll's D2 entropy-based dispersion index (0-1 scale).
    get_roschengren_s_adj() -> Optional[float]
        Rosengren's adjusted S index, weighted by partition sizes.
    get_dp() -> Optional[float]
        DP (Deviation of Proportions) index (0-1 scale, 0 = perfect match with expected).
    get_dp_norm() -> Optional[float]
        Normalized DP index, adjusted for minimum possible dispersion.
    get_kl_divergence() -> Optional[float]
        Kullback-Leibler divergence from expected to observed distribution.
    get_jsd_dispersion() -> Optional[float]
        Jensen-Shannon divergence based dispersion (1 - normalized JSD).
    get_hellinger_dispersion() -> Optional[float]
        Hellinger distance based dispersion (1 - normalized distance).
    get_evenness_da() -> Optional[float]
        Evenness index based on pairwise differences (Egbert & Burch DA).
    get_mean_text_frequency_ft() -> Optional[float]
        Mean normalized frequency across partitions (Egbert & Burch FT).
    get_pervasiveness_pt() -> Optional[float]
        Proportion of partitions containing the word (Egbert & Burch PT).
    calculate_all_metrics() -> DispersionMetrics
        Compute and return all supported dispersion metrics in a single call.

    Static Methods
    --------------
    calculate_batch_metrics(frequency_matrix, corpus_part_sizes, total_corpus_words) -> List[DispersionMetrics]
        Efficiently compute metrics for multiple words using parallel processing.

        Parameters:
        - frequency_matrix: List[List[float]], each row is a word's frequency vector
        - corpus_part_sizes: List[float], partition sizes (same for all words)
        - total_corpus_words: float, total corpus size

        Returns list of DispersionMetrics objects, one per input word.

    calculate_single_metric(frequency_vector, corpus_part_sizes, total_corpus_words, metric_name) -> Optional[float]
        Calculate a single specific metric without creating a full analyzer instance.

        Parameters:
        - frequency_vector: List[float], word frequencies per partition
        - corpus_part_sizes: List[float], partition sizes
        - total_corpus_words: float, total corpus size
        - metric_name: str, one of: 'juilland_d', 'carroll_d2', 'dp', 'dp_norm',
          'kl_divergence', 'jsd_dispersion', 'hellinger_dispersion', 'evenness_da',
          'mean_text_frequency_ft', 'pervasiveness_pt'

        Returns the computed metric value or None if computation fails.
    """

    def __init__(
        self,
        v: List[float],
        corpus_part_sizes_words: List[float],
        total_corpus_words: float,
    ) -> None:
        """
        Initialize the analyzer with word frequencies and corpus structure.

        Raises
        ------
        PyValueError
            If vectors have different lengths, are empty, or contain invalid values.
        """
        ...

    def get_range(self) -> int:
        """Get the number of partitions containing the word (cached)."""
        ...

    def get_sd_population(self) -> Optional[float]:
        """Get population standard deviation of frequencies."""
        ...

    def get_vc_population(self) -> Optional[float]:
        """Get coefficient of variation (sd/mean)."""
        ...

    def get_juilland_d(self) -> Optional[float]:
        """Get Juilland's D dispersion index."""
        ...

    def get_carroll_d2(self) -> Optional[float]:
        """Get Carroll's D2 entropy-based dispersion."""
        ...

    def get_roschengren_s_adj(self) -> Optional[float]:
        """Get Rosengren's adjusted S index."""
        ...

    def get_dp(self) -> Optional[float]:
        """Get DP (Deviation of Proportions) index."""
        ...

    def get_dp_norm(self) -> Optional[float]:
        """Get normalized DP index."""
        ...

    def get_kl_divergence(self) -> Optional[float]:
        """Get Kullback-Leibler divergence."""
        ...

    def get_jsd_dispersion(self) -> Optional[float]:
        """Get Jensen-Shannon divergence based dispersion."""
        ...

    def get_hellinger_dispersion(self) -> Optional[float]:
        """Get Hellinger distance based dispersion."""
        ...

    def get_evenness_da(self) -> Optional[float]:
        """Get evenness index (DA) based on pairwise differences."""
        ...

    def get_mean_text_frequency_ft(self) -> Optional[float]:
        """Get mean text frequency (FT) - average normalized frequency."""
        ...

    def get_pervasiveness_pt(self) -> Optional[float]:
        """Get pervasiveness (PT) - proportion of partitions with the word."""
        ...

    def calculate_all_metrics(self) -> DispersionMetrics:
        """Calculate and return all dispersion metrics."""
        ...

    @staticmethod
    def calculate_batch_metrics(
        frequency_matrix: List[List[float]],
        corpus_part_sizes: List[float],
        total_corpus_words: float,
    ) -> List[DispersionMetrics]:
        """
        Calculate metrics for multiple words using parallel processing.

        Raises
        ------
        PyValueError
            If matrix rows have inconsistent lengths or invalid values.
        """
        ...

    @staticmethod
    def calculate_single_metric(
        frequency_vector: List[float],
        corpus_part_sizes: List[float],
        total_corpus_words: float,
        metric_name: str,
    ) -> Optional[float]:
        """
        Calculate a single metric without full analyzer initialization.

        Raises
        ------
        PyValueError
            If metric_name is unknown or inputs are invalid.
        """
        ...
