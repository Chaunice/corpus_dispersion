//! Integration tests for the corpus dispersion library
//!
//! These tests verify that all components work together correctly
//! and test the public API from an end-user perspective.

use corpus_dispersion::*;

mod common;
use common::*;

#[test]
fn test_new_analyzer_valid() {
    let v = vec![2.0, 3.0, 5.0];
    let sizes = vec![10.0, 10.0, 10.0];
    let total = 30.0;
    let analyzer = CorpusWordAnalyzer::new(v, sizes, total);
    assert!(analyzer.is_ok());
}

#[test]
fn test_new_analyzer_invalid_length() {
    let v = vec![1.0, 2.0];
    let sizes = vec![10.0];
    let total = 10.0;
    let analyzer = CorpusWordAnalyzer::new(v, sizes, total);
    assert!(analyzer.is_err());
}

#[test]
fn test_new_analyzer_empty_vectors() {
    let v = vec![];
    let sizes = vec![];
    let total = 30.0;
    let analyzer = CorpusWordAnalyzer::new(v, sizes, total);
    assert!(analyzer.is_err());
}

#[test]
fn test_new_analyzer_invalid_total() {
    let v = vec![1.0, 2.0];
    let sizes = vec![10.0, 10.0];
    let total = 0.0;
    let analyzer = CorpusWordAnalyzer::new(v, sizes, total);
    assert!(analyzer.is_err());
}

#[test]
fn test_get_range() {
    let v = vec![1.0, 0.0, 2.0];
    let sizes = vec![10.0, 10.0, 10.0];
    let total = 30.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();
    assert_eq!(analyzer.get_range(), 2);
}

#[test]
fn test_get_range_caching() {
    let mut analyzer = create_basic_analyzer();
    // First call should compute and cache
    let range1 = analyzer.get_range();
    // Second call should use cached value
    let range2 = analyzer.get_range();
    assert_eq!(range1, range2);
    assert_eq!(range1, 3);
}

#[test]
fn test_helper_functions() {
    // Test that our helper functions work correctly
    let mut basic = create_basic_analyzer();
    let mut zero = create_zero_analyzer();
    let mut uniform = create_uniform_analyzer();

    // Verify they create different types of analyzers
    assert!(basic.get_range() > 0);
    assert_eq!(zero.get_range(), 0);
    assert_eq!(uniform.get_range(), 3);
}

#[test]
fn test_assert_approx_eq_helper() {
    // Test our custom assertion helper
    assert_approx_eq(1.0, 1.001, 0.01, "Should be approximately equal");

    // This should panic, but we won't actually run it in the test
    // assert_approx_eq(1.0, 2.0, 0.01, "Should panic");
}

#[test]
fn test_data_generator() {
    let (v, sizes, total) = TestDataGenerator::extreme_skew();
    assert_eq!(v.len(), sizes.len());
    assert!(total > 0.0);

    let (v, sizes, total) = TestDataGenerator::perfect_uniform();
    assert_eq!(v.len(), sizes.len());
    assert!(total > 0.0);

    let (v, sizes, total) = TestDataGenerator::single_occurrence();
    assert_eq!(v.len(), 1);
    assert_eq!(sizes.len(), 1);
    assert!(total > 0.0);
}

#[test]
fn test_get_sd_population() {
    let mut analyzer = create_basic_analyzer();
    let sd = analyzer.get_sd_population();
    assert!(sd.is_some());
    assert!(sd.unwrap() > 0.0);
}

#[test]
fn test_get_sd_population_zero_frequency() {
    let mut analyzer = create_zero_analyzer();
    let sd = analyzer.get_sd_population();
    assert_eq!(sd, Some(0.0));
}

#[test]
fn test_get_vc_population() {
    let mut analyzer = create_basic_analyzer();
    let vc = analyzer.get_vc_population();
    assert!(vc.is_some());
    assert!(vc.unwrap() >= 0.0);
}

#[test]
fn test_get_vc_population_zero_mean() {
    let mut analyzer = create_zero_analyzer();
    let vc = analyzer.get_vc_population();
    assert_eq!(vc, Some(0.0));
}

#[test]
fn test_get_juilland_d() {
    let mut analyzer = create_basic_analyzer();
    let jd = analyzer.get_juilland_d();
    assert!(jd.is_some());
    assert!(jd.unwrap() >= 0.0 && jd.unwrap() <= 1.0);
}

#[test]
fn test_get_juilland_d_single_part() {
    let v = vec![5.0];
    let sizes = vec![10.0];
    let total = 10.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();
    let jd = analyzer.get_juilland_d();
    assert_eq!(jd, Some(1.0));
}

#[test]
fn test_get_juilland_d_zero_frequency() {
    let mut analyzer = create_zero_analyzer();
    let jd = analyzer.get_juilland_d();
    assert_eq!(jd, Some(0.0));
}

#[test]
fn test_get_carroll_d2() {
    let mut analyzer = create_basic_analyzer();
    let cd2 = analyzer.get_carroll_d2();
    assert!(cd2.is_some());
    assert!(cd2.unwrap() >= 0.0 && cd2.unwrap() <= 1.0);
}

#[test]
fn test_get_carroll_d2_single_part() {
    let v = vec![5.0];
    let sizes = vec![10.0];
    let total = 10.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();
    let cd2 = analyzer.get_carroll_d2();
    assert_eq!(cd2, Some(1.0));
}

#[test]
fn test_get_roschengren_s_adj() {
    let analyzer = create_basic_analyzer();
    let s_adj = analyzer.get_roschengren_s_adj();
    assert!(s_adj.is_some());
    assert!(s_adj.unwrap() >= 0.0 && s_adj.unwrap() <= 1.0);
}

#[test]
fn test_get_roschengren_s_adj_zero_frequency() {
    let analyzer = create_zero_analyzer();
    let s_adj = analyzer.get_roschengren_s_adj();
    assert_eq!(s_adj, Some(0.0));
}

#[test]
fn test_get_dp() {
    let analyzer = create_basic_analyzer();
    let dp = analyzer.get_dp();
    assert!(dp.is_some());
    assert!(dp.unwrap() >= 0.0 && dp.unwrap() <= 1.0);
}

#[test]
fn test_get_dp_zero_frequency() {
    let analyzer = create_zero_analyzer();
    let dp = analyzer.get_dp();
    assert_eq!(dp, Some(0.0));
}

#[test]
fn test_get_dp_norm() {
    let mut analyzer = create_basic_analyzer();
    let dp_norm = analyzer.get_dp_norm();
    assert!(dp_norm.is_some());
    assert!(dp_norm.unwrap() >= 0.0);
}

#[test]
fn test_get_kl_divergence() {
    let analyzer = create_basic_analyzer();
    let kl = analyzer.get_kl_divergence();
    assert!(kl.is_some());
    assert!(kl.unwrap() >= 0.0);
}

#[test]
fn test_get_kl_divergence_zero_frequency() {
    let analyzer = create_zero_analyzer();
    let kl = analyzer.get_kl_divergence();
    assert_eq!(kl, Some(0.0));
}

#[test]
fn test_get_evenness_da() {
    let mut analyzer = create_basic_analyzer();
    let da = analyzer.get_evenness_da();
    assert!(da.is_some());
    assert!(da.unwrap() >= 0.0 && da.unwrap() <= 1.0);
}

#[test]
fn test_get_evenness_da_uniform() {
    let mut analyzer = create_uniform_analyzer();
    let da = analyzer.get_evenness_da();
    assert!(da.is_some());
    // Uniform distribution should have high evenness
    assert!(da.unwrap() > 0.9);
}

#[test]
fn test_get_evenness_da_zero_frequency() {
    let mut analyzer = create_zero_analyzer();
    let da = analyzer.get_evenness_da();
    assert_eq!(da, Some(0.0));
}

#[test]
fn test_get_evenness_da_single_part() {
    let v = vec![5.0];
    let sizes = vec![10.0];
    let total = 10.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();
    let da = analyzer.get_evenness_da();
    assert_eq!(da, Some(1.0));
}

#[test]
fn test_get_jsd_dispersion() {
    let analyzer = create_basic_analyzer();
    let jsd = analyzer.get_jsd_dispersion();
    assert!(jsd.is_some());
    assert!(jsd.unwrap() >= 0.0 && jsd.unwrap() <= 1.0);
}

#[test]
fn test_get_jsd_dispersion_zero_frequency() {
    let analyzer = create_zero_analyzer();
    let jsd = analyzer.get_jsd_dispersion();
    assert_eq!(jsd, Some(0.0));
}

#[test]
fn test_get_hellinger_dispersion() {
    let analyzer = create_basic_analyzer();
    let hellinger = analyzer.get_hellinger_dispersion();
    assert!(hellinger.is_some());
    assert!(hellinger.unwrap() >= 0.0 && hellinger.unwrap() <= 1.0);
}

#[test]
fn test_get_hellinger_dispersion_zero_frequency() {
    let analyzer = create_zero_analyzer();
    let hellinger = analyzer.get_hellinger_dispersion();
    assert_eq!(hellinger, Some(0.0));
}

#[test]
fn test_get_mean_text_frequency_ft() {
    let mut analyzer = create_basic_analyzer();
    let ft = analyzer.get_mean_text_frequency_ft();
    assert!(ft.is_some());
    assert!(ft.unwrap() > 0.0);
}

#[test]
fn test_get_pervasiveness_pt() {
    let mut analyzer = create_basic_analyzer();
    let pt = analyzer.get_pervasiveness_pt();
    assert!(pt.is_some());
    assert!(pt.unwrap() >= 0.0 && pt.unwrap() <= 1.0);
}

#[test]
fn test_calculate_all_metrics() {
    let mut analyzer = create_basic_analyzer();
    let metrics = analyzer.calculate_all_metrics();

    assert_eq!(metrics.range, 3);
    assert!(metrics.sd_population.is_some());
    assert!(metrics.vc_population.is_some());
    assert!(metrics.juilland_d.is_some());
    assert!(metrics.carroll_d2.is_some());
    assert!(metrics.roschengren_s_adj.is_some());
    assert!(metrics.dp.is_some());
    assert!(metrics.dp_norm.is_some());
    assert!(metrics.kl_divergence.is_some());
    assert!(metrics.jsd_dispersion.is_some());
    assert!(metrics.hellinger_dispersion.is_some());
    assert!(metrics.mean_text_frequency_ft.is_some());
    assert!(metrics.pervasiveness_pt.is_some());
    assert!(metrics.evenness_da.is_some());
    assert!(metrics.ft_adjusted_by_pt.is_some());
    assert!(metrics.ft_adjusted_by_da.is_some());
}

#[test]
fn test_calculate_batch_metrics() {
    let frequency_matrix = vec![
        vec![2.0, 3.0, 5.0],
        vec![1.0, 2.0, 3.0],
        vec![4.0, 2.0, 1.0],
    ];
    let corpus_part_sizes = vec![10.0, 10.0, 10.0];
    let total_corpus_words = 30.0;

    let result = CorpusWordAnalyzer::calculate_batch_metrics(
        frequency_matrix,
        corpus_part_sizes,
        total_corpus_words,
    );

    assert!(result.is_ok());
    let metrics_vec = result.unwrap();
    assert_eq!(metrics_vec.len(), 3);

    // Check that all metrics are computed
    for metrics in &metrics_vec {
        assert!(metrics.range > 0);
        assert!(metrics.juilland_d.is_some());
    }
}

#[test]
fn test_calculate_batch_metrics_empty() {
    let frequency_matrix = vec![];
    let corpus_part_sizes = vec![10.0, 10.0, 10.0];
    let total_corpus_words = 30.0;

    let result = CorpusWordAnalyzer::calculate_batch_metrics(
        frequency_matrix,
        corpus_part_sizes,
        total_corpus_words,
    );

    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_calculate_batch_metrics_invalid_length() {
    let frequency_matrix = vec![
        vec![2.0, 3.0], // Wrong length
        vec![1.0, 2.0, 3.0],
    ];
    let corpus_part_sizes = vec![10.0, 10.0, 10.0];
    let total_corpus_words = 30.0;

    let result = CorpusWordAnalyzer::calculate_batch_metrics(
        frequency_matrix,
        corpus_part_sizes,
        total_corpus_words,
    );

    assert!(result.is_err());
}

#[test]
fn test_calculate_single_metric_juilland_d() {
    let frequency_vector = vec![2.0, 3.0, 5.0];
    let corpus_part_sizes = vec![10.0, 10.0, 10.0];
    let total_corpus_words = 30.0;

    let result = CorpusWordAnalyzer::calculate_single_metric(
        frequency_vector,
        corpus_part_sizes,
        total_corpus_words,
        "juilland_d",
    );

    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value.is_some());
    assert!(value.unwrap() >= 0.0 && value.unwrap() <= 1.0);
}

#[test]
fn test_calculate_single_metric_unknown() {
    let frequency_vector = vec![2.0, 3.0, 5.0];
    let corpus_part_sizes = vec![10.0, 10.0, 10.0];
    let total_corpus_words = 30.0;

    let result = CorpusWordAnalyzer::calculate_single_metric(
        frequency_vector,
        corpus_part_sizes,
        total_corpus_words,
        "unknown_metric",
    );

    assert!(result.is_err());
}

#[test]
fn test_all_supported_single_metrics() {
    let frequency_vector = vec![2.0, 3.0, 5.0];
    let corpus_part_sizes = vec![10.0, 10.0, 10.0];
    let total_corpus_words = 30.0;

    let metrics = [
        "juilland_d",
        "carroll_d2",
        "dp",
        "dp_norm",
        "kl_divergence",
        "jsd_dispersion",
        "hellinger_dispersion",
        "evenness_da",
        "mean_text_frequency_ft",
        "pervasiveness_pt",
    ];

    for metric in &metrics {
        let result = CorpusWordAnalyzer::calculate_single_metric(
            frequency_vector.clone(),
            corpus_part_sizes.clone(),
            total_corpus_words,
            metric,
        );
        assert!(result.is_ok(), "Failed for metric: {metric}");
        assert!(result.unwrap().is_some(), "No value for metric: {metric}");
    }
}

#[test]
fn test_dispersion_metrics_repr() {
    let metrics = DispersionMetrics {
        range: 3,
        sd_population: Some(1.5),
        vc_population: Some(0.5),
        juilland_d: Some(0.8),
        carroll_d2: Some(0.9),
        roschengren_s_adj: Some(0.7),
        dp: Some(0.2),
        dp_norm: Some(0.3),
        kl_divergence: Some(0.1),
        jsd_dispersion: Some(0.95),
        hellinger_dispersion: Some(0.85),
        mean_text_frequency_ft: Some(0.33),
        pervasiveness_pt: Some(1.0),
        evenness_da: Some(0.75),
        ft_adjusted_by_pt: Some(0.33),
        ft_adjusted_by_da: Some(0.25),
    };

    // Test that we can create the struct successfully
    assert_eq!(metrics.range, 3);
    assert_eq!(metrics.juilland_d, Some(0.8));
    assert_eq!(metrics.carroll_d2, Some(0.9));
}
