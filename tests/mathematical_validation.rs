//! Mathematical validation tests
//!
//! These tests verify the mathematical correctness of all dispersion metrics
//! using hand-calculated expected values and known mathematical properties.

use corpus_dispersion::*;

mod common;

/// Mathematical correctness verification tests
#[test]
fn test_mathematical_correctness_juilland_d() {
    // 已知的测试案例：均匀分布应该有较高的 Juilland D 值
    let v = vec![10.0, 10.0, 10.0, 10.0];
    let sizes = vec![100.0, 100.0, 100.0, 100.0];
    let total = 400.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();

    let jd = analyzer.get_juilland_d().unwrap();
    // 均匀分布的 Juilland D 应该接近 1.0
    assert!(
        jd > 0.9,
        "Uniform distribution should have high Juilland D, got {jd}"
    );
}

#[test]
fn test_mathematical_correctness_carroll_d2() {
    // 测试极端不均匀分布的 Carroll D2
    let v = vec![100.0, 0.0, 0.0, 0.0];
    let sizes = vec![100.0, 100.0, 100.0, 100.0];
    let total = 400.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();

    let cd2 = analyzer.get_carroll_d2().unwrap();
    // 极端不均匀分布的 Carroll D2 应该接近 0
    assert!(
        cd2 < 0.5,
        "Highly skewed distribution should have low Carroll D2, got {cd2}"
    );
}

#[test]
fn test_mathematical_correctness_range() {
    // 验证范围计算的准确性
    let v = vec![5.0, 0.0, 3.0, 0.0, 7.0];
    let sizes = vec![10.0, 10.0, 10.0, 10.0, 10.0];
    let total = 50.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();

    // 应该有 3 个非零频率
    assert_eq!(analyzer.get_range(), 3);
}

#[test]
fn test_mathematical_correctness_evenness_da() {
    // 测试完全均匀分布的 evenness
    let v = vec![5.0, 5.0, 5.0];
    let sizes = vec![25.0, 25.0, 25.0];
    let total = 75.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();

    let da = analyzer.get_evenness_da().unwrap();
    // 完全均匀分布的 evenness 应该是 1.0
    assert!(
        (da - 1.0).abs() < 0.001,
        "Perfect evenness should be 1.0, got {da}"
    );
}

#[test]
fn test_mathematical_correctness_dp() {
    // 测试 DP 指标的数学性质
    let v = vec![10.0, 20.0, 30.0];
    let sizes = vec![20.0, 20.0, 20.0];
    let total = 60.0;
    let analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();

    let dp = analyzer.get_dp().unwrap();
    // DP 值应该在 0 到 1 之间
    assert!(
        (0.0..=1.0).contains(&dp),
        "DP should be between 0 and 1, got {dp}"
    );

    // 手动计算验证
    // v_i/f = [10/60, 20/60, 30/60] = [1/6, 1/3, 1/2]
    // s_i = [20/60, 20/60, 20/60] = [1/3, 1/3, 1/3]
    // |v_i/f - s_i| = [|1/6 - 1/3|, |1/3 - 1/3|, |1/2 - 1/3|] = [1/6, 0, 1/6]
    // DP = 0.5 * (1/6 + 0 + 1/6) = 0.5 * 1/3 = 1/6 ≈ 0.1667
    let expected_dp = 1.0 / 6.0;
    assert!(
        (dp - expected_dp).abs() < 0.001,
        "DP calculation incorrect, expected {expected_dp}, got {dp}"
    );
}

#[test]
fn test_mathematical_correctness_pervasiveness() {
    // 测试 pervasiveness 的计算
    let v = vec![1.0, 0.0, 2.0, 0.0, 3.0];
    let sizes = vec![10.0, 10.0, 10.0, 10.0, 10.0];
    let total = 50.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();

    let pt = analyzer.get_pervasiveness_pt().unwrap();
    // 3 个非零频率 / 5 个部分 = 0.6
    assert!(
        (pt - 0.6).abs() < 0.001,
        "Pervasiveness calculation incorrect, expected 0.6, got {pt}"
    );
}

#[test]
fn test_mathematical_correctness_standard_deviation() {
    // 测试标准差计算的准确性
    let v = vec![2.0, 4.0, 6.0];
    let sizes = vec![10.0, 10.0, 10.0];
    let total = 30.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();

    let sd = analyzer.get_sd_population().unwrap();
    // 手动计算：mean = 4, variance = ((2-4)^2 + (4-4)^2 + (6-4)^2) / 3 = (4 + 0 + 4) / 3 = 8/3
    // sd = sqrt(8/3) ≈ 1.633
    let expected_sd = (8.0_f64 / 3.0_f64).sqrt();
    assert!(
        (sd - expected_sd).abs() < 0.001,
        "Standard deviation calculation incorrect, expected {expected_sd}, got {sd}"
    );
}

#[test]
fn test_mathematical_correctness_coefficient_of_variation() {
    // 测试变异系数的计算
    let v = vec![1.0, 2.0, 3.0];
    let sizes = vec![10.0, 10.0, 10.0];
    let total = 30.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();

    let vc = analyzer.get_vc_population().unwrap();
    let sd = analyzer.get_sd_population().unwrap();
    let mean = 2.0; // (1+2+3)/3 = 2

    let expected_vc = sd / mean;
    assert!(
        (vc - expected_vc).abs() < 0.001,
        "Coefficient of variation calculation incorrect, expected {expected_vc}, got {vc}"
    );
}

#[test]
fn test_mathematical_properties_symmetry() {
    // 测试对称性：[a,b,c] 和 [c,b,a] 应该产生相同的某些指标
    let v1 = vec![1.0, 5.0, 9.0];
    let v2 = vec![9.0, 5.0, 1.0];
    let sizes = vec![10.0, 10.0, 10.0];
    let total = 30.0;

    let mut analyzer1 = CorpusWordAnalyzer::new(v1, sizes.clone(), total).unwrap();
    let mut analyzer2 = CorpusWordAnalyzer::new(v2, sizes, total).unwrap();

    // 这些指标应该对称
    assert_eq!(analyzer1.get_range(), analyzer2.get_range());
    assert!(
        (analyzer1.get_sd_population().unwrap() - analyzer2.get_sd_population().unwrap()).abs()
            < 0.001
    );
    assert!(
        (analyzer1.get_vc_population().unwrap() - analyzer2.get_vc_population().unwrap()).abs()
            < 0.001
    );
}

#[test]
fn test_mathematical_properties_monotonicity() {
    // 测试单调性：更均匀的分布应该有更高的 Juilland D
    let v_uniform = vec![5.0, 5.0, 5.0];
    let v_skewed = vec![10.0, 2.0, 3.0];
    let sizes = vec![10.0, 10.0, 10.0];
    let total = 30.0;

    let mut analyzer_uniform = CorpusWordAnalyzer::new(v_uniform, sizes.clone(), total).unwrap();
    let mut analyzer_skewed = CorpusWordAnalyzer::new(v_skewed, sizes, total).unwrap();

    let jd_uniform = analyzer_uniform.get_juilland_d().unwrap();
    let jd_skewed = analyzer_skewed.get_juilland_d().unwrap();

    assert!(
        jd_uniform > jd_skewed,
        "Uniform distribution should have higher Juilland D than skewed distribution"
    );
}

#[test]
fn test_mathematical_correctness_roschengren_s_adj() {
    // 测试 Roschengren S 调整指标
    let v = vec![4.0, 9.0, 16.0];
    let sizes = vec![1.0, 4.0, 9.0]; // 不同大小的corpus parts
    let total = 14.0;
    let analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();

    let s_adj = analyzer.get_roschengren_s_adj().unwrap();
    // 手工计算：s = [1/14, 4/14, 9/14], v = [4, 9, 16], f = 29
    // sum_sqrt = sqrt(1/14 * 4) + sqrt(4/14 * 9) + sqrt(9/14 * 16)
    //          = sqrt(4/14) + sqrt(36/14) + sqrt(144/14)
    //          = 2/√14 + 6/√14 + 12/√14 = 20/√14
    // S_adj = (20/√14)^2 / 29 = 400/14 / 29 = 400/(14*29)
    let expected = 400.0 / (14.0 * 29.0);
    assert!(
        (s_adj - expected).abs() < 0.001,
        "Roschengren S calculation incorrect, expected {expected}, got {s_adj}"
    );
}

#[test]
fn test_mathematical_correctness_dp_norm() {
    // 测试标准化 DP 指标
    let v = vec![10.0, 5.0, 15.0];
    let sizes = vec![10.0, 5.0, 15.0]; // 不同大小确保 min_s != 1
    let total = 30.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();

    let dp = analyzer.get_dp().unwrap();
    let dp_norm = analyzer.get_dp_norm().unwrap();

    // min_s = 5/30 = 1/6
    // dp_norm = dp / (1 - 1/6) = dp / (5/6) = dp * 6/5
    let min_s = 5.0 / 30.0;
    let expected_dp_norm = dp / (1.0 - min_s);
    assert!(
        (dp_norm - expected_dp_norm).abs() < 0.001,
        "DP norm calculation incorrect, expected {expected_dp_norm}, got {dp_norm}"
    );
}

#[test]
fn test_mathematical_correctness_kl_divergence() {
    // 测试 KL 散度计算
    let v = vec![2.0, 4.0, 6.0];
    let sizes = vec![4.0, 4.0, 4.0]; // 均匀的参考分布
    let total = 12.0;
    let analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();

    let kl = analyzer.get_kl_divergence().unwrap();
    // 手工计算：p = [2/12, 4/12, 6/12] = [1/6, 1/3, 1/2]
    // q = [4/12, 4/12, 4/12] = [1/3, 1/3, 1/3]
    // KL = Σ p_i * log2(p_i / q_i)
    //    = 1/6 * log2(1/6 / 1/3) + 1/3 * log2(1/3 / 1/3) + 1/2 * log2(1/2 / 1/3)
    //    = 1/6 * log2(1/2) + 1/3 * log2(1) + 1/2 * log2(3/2)
    //    = 1/6 * (-1) + 1/3 * 0 + 1/2 * log2(1.5)
    #[allow(clippy::suboptimal_flops)]
    // Clear mathematical expression is more important than micro-optimization
    let expected_kl = -1.0 / 6.0 + 0.0 + 0.5 * (1.5_f64.log2());
    assert!(
        (kl - expected_kl).abs() < 0.001,
        "KL divergence calculation incorrect, expected {expected_kl}, got {kl}"
    );
}

#[test]
fn test_mathematical_correctness_jsd_dispersion() {
    // 测试 Jensen-Shannon 散度计算
    let v = vec![6.0, 2.0, 4.0];
    let sizes = vec![4.0, 4.0, 4.0]; // 均匀参考分布
    let total = 12.0;
    let analyzer = CorpusWordAnalyzer::new(v, sizes.clone(), total).unwrap();

    let jsd = analyzer.get_jsd_dispersion().unwrap();
    // JSD 应该在 0-1 之间，且对于不同分布应该 > 0
    assert!(
        jsd > 0.0 && jsd <= 1.0,
        "JSD dispersion should be between 0 and 1, got {jsd}"
    );

    // 测试完全相同分布的 JSD 应该接近 1
    let v_uniform = vec![4.0, 4.0, 4.0];
    let analyzer_uniform = CorpusWordAnalyzer::new(v_uniform, sizes, total).unwrap();
    let jsd_uniform = analyzer_uniform.get_jsd_dispersion().unwrap();
    assert!(
        jsd_uniform > 0.99,
        "JSD for identical distributions should be close to 1, got {jsd_uniform}"
    );
}

#[test]
fn test_mathematical_correctness_hellinger_dispersion() {
    // 测试 Hellinger 散度计算
    let v = vec![8.0, 4.0, 0.0];
    let sizes = vec![4.0, 4.0, 4.0];
    let total = 12.0;
    let analyzer = CorpusWordAnalyzer::new(v, sizes.clone(), total).unwrap();

    let hellinger = analyzer.get_hellinger_dispersion().unwrap();
    // Hellinger 散度应该在 0-1 之间
    assert!(
        (0.0..=1.0).contains(&hellinger),
        "Hellinger dispersion should be between 0 and 1, got {hellinger}"
    );

    // 测试完全相同分布的 Hellinger 应该接近 1
    let v_uniform = vec![4.0, 4.0, 4.0];
    let analyzer_uniform = CorpusWordAnalyzer::new(v_uniform, sizes, total).unwrap();
    let hellinger_uniform = analyzer_uniform.get_hellinger_dispersion().unwrap();
    assert!(
        hellinger_uniform > 0.99,
        "Hellinger for identical distributions should be close to 1, got {hellinger_uniform}"
    );
}

#[test]
fn test_mathematical_correctness_mean_text_frequency() {
    // 测试平均文本频率 (FT)
    let v = vec![3.0, 6.0, 9.0];
    let sizes = vec![10.0, 20.0, 30.0];
    let total = 60.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();

    let ft = analyzer.get_mean_text_frequency_ft().unwrap();
    // FT 应该等于 p 向量的平均值
    // p = [3/10, 6/20, 9/30] = [0.3, 0.3, 0.3]
    // mean_p = (0.3 + 0.3 + 0.3) / 3 = 0.3
    let expected_ft = 0.3;
    assert!(
        (ft - expected_ft).abs() < 0.001,
        "Mean text frequency calculation incorrect, expected {expected_ft}, got {ft}"
    );
}

#[test]
fn test_mathematical_correctness_adjusted_metrics() {
    // 测试调整后的指标计算
    let v = vec![4.0, 8.0, 12.0];
    let sizes = vec![10.0, 20.0, 30.0];
    let total = 60.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();

    let metrics = analyzer.calculate_all_metrics();

    let ft = metrics.mean_text_frequency_ft.unwrap();
    let pt = metrics.pervasiveness_pt.unwrap();
    let da = metrics.evenness_da.unwrap();

    // 验证调整后的指标计算
    let expected_ft_by_pt = ft * pt;
    let expected_ft_by_da = ft * da;

    assert!(
        (metrics.ft_adjusted_by_pt.unwrap() - expected_ft_by_pt).abs() < 0.001,
        "FT adjusted by PT calculation incorrect"
    );
    assert!(
        (metrics.ft_adjusted_by_da.unwrap() - expected_ft_by_da).abs() < 0.001,
        "FT adjusted by DA calculation incorrect"
    );
}

#[test]
fn test_mathematical_correctness_extreme_cases() {
    // 测试极端情况的数学正确性

    // 情况1：完全集中在一个部分
    let v_concentrated = vec![100.0, 0.0, 0.0];
    let sizes = vec![10.0, 10.0, 10.0];
    let total = 30.0;
    let mut analyzer_concentrated =
        CorpusWordAnalyzer::new(v_concentrated, sizes.clone(), total).unwrap();

    assert_eq!(analyzer_concentrated.get_range(), 1);
    // Exact mathematical result expected, so direct float comparison is appropriate
    let pervasiveness = analyzer_concentrated.get_pervasiveness_pt().unwrap();
    assert!((pervasiveness - 1.0 / 3.0).abs() < f64::EPSILON);

    // 情况2：完全均匀分布
    let v_uniform = vec![10.0, 10.0, 10.0];
    let mut analyzer_uniform = CorpusWordAnalyzer::new(v_uniform, sizes, total).unwrap();

    assert_eq!(analyzer_uniform.get_range(), 3);
    // Exact mathematical result expected, so direct float comparison is appropriate
    let pervasiveness_uniform = analyzer_uniform.get_pervasiveness_pt().unwrap();
    assert!((pervasiveness_uniform - 1.0).abs() < f64::EPSILON);
    assert!((analyzer_uniform.get_evenness_da().unwrap() - 1.0).abs() < 0.001);
}

#[test]
fn test_mathematical_correctness_all_metrics_consistency() {
    // 测试所有指标的一致性和合理性
    let v = vec![5.0, 10.0, 15.0, 20.0];
    let sizes = vec![10.0, 10.0, 10.0, 10.0];
    let total = 40.0;
    let mut analyzer = CorpusWordAnalyzer::new(v, sizes, total).unwrap();

    let metrics = analyzer.calculate_all_metrics();

    // 验证所有指标都在合理范围内
    assert_eq!(metrics.range, 4);
    assert!(metrics.sd_population.unwrap() > 0.0);
    assert!(metrics.vc_population.unwrap() > 0.0);
    assert!(metrics.juilland_d.unwrap() >= 0.0 && metrics.juilland_d.unwrap() <= 1.0);
    assert!(metrics.carroll_d2.unwrap() >= 0.0 && metrics.carroll_d2.unwrap() <= 1.0);
    assert!(metrics.roschengren_s_adj.unwrap() >= 0.0 && metrics.roschengren_s_adj.unwrap() <= 1.0);
    assert!(metrics.dp.unwrap() >= 0.0 && metrics.dp.unwrap() <= 1.0);
    assert!(metrics.dp_norm.unwrap() >= 0.0);
    assert!(metrics.kl_divergence.unwrap() >= 0.0);
    assert!(metrics.jsd_dispersion.unwrap() >= 0.0 && metrics.jsd_dispersion.unwrap() <= 1.0);
    assert!(
        metrics.hellinger_dispersion.unwrap() >= 0.0
            && metrics.hellinger_dispersion.unwrap() <= 1.0
    );
    assert!(metrics.mean_text_frequency_ft.unwrap() > 0.0);
    assert!(metrics.pervasiveness_pt.unwrap() > 0.0 && metrics.pervasiveness_pt.unwrap() <= 1.0);
    assert!(metrics.evenness_da.unwrap() >= 0.0 && metrics.evenness_da.unwrap() <= 1.0);
    assert!(metrics.ft_adjusted_by_pt.is_some());
    assert!(metrics.ft_adjusted_by_da.is_some());
}
