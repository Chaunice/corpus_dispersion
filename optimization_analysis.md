# Rust 代码优化分析报告

## 发现的问题

### 1. Rust 最佳实践问题

#### 错误处理

- **问题**: 使用通用的 `PyValueError` 而不是自定义错误类型
- **影响**: 错误信息不够具体，调试困难
- **改进**: 创建自定义 `AnalysisError` 枚举类型

#### 魔数使用

- **问题**: 硬编码的浮点数比较阈值 `1e-12`
- **影响**: 代码可维护性差，难以统一调整精度
- **改进**: 定义 `FLOAT_EPSILON` 常量

#### 代码重复

- **问题**: `mean_p` 和方差计算在多个方法中重复
- **影响**: 代码冗余，维护成本高
- **改进**: 使用 `PrecomputedStats` 结构体缓存计算结果

### 2. 计算效率问题

#### O(n²) 复杂度问题

- **问题**: `get_evenness_da()` 方法中的嵌套循环

```rust
for i in 0..self.n {
    for j in (i + 1)..self.n {
        sum_abs_diff += (self.p[i] - self.p[j]).abs();
        num_pairs += 1;
    }
}
```

- **影响**: 时间复杂度 O(n²)，对大型语料库效率低下
- **改进**: 使用数学公式优化为 O(n log n)

#### 重复计算

- **问题**: 每次调用方法都重新计算基础统计数据
- **影响**: 计算资源浪费
- **改进**: 延迟计算和缓存机制

#### 内存分配效率

- **问题**: 批处理中每次迭代都克隆 `corpus_part_sizes`
- **影响**: 不必要的内存分配和复制
- **改进**: 使用 `Arc<Vec<f64>>` 共享数据

## 主要优化改进

### 1. 性能优化

#### 预计算统计数据

```rust
struct PrecomputedStats {
    mean_v: f64,
    mean_p: f64,
    variance_v: f64,
    variance_p: f64,
    sum_p: f64,
    min_s: f64,
    range: i32,
}
```

- **效果**: 避免重复计算，显著提升性能
- **适用场景**: 需要计算多个指标时

#### 优化的平均绝对差算法

```rust
fn compute_mean_absolute_difference_optimized(values: &[f64]) -> f64 {
    // 使用公式: MAD = (2 * sum(i * v[i]) - n * sum(v[i])) / (n * (n-1))
    // 复杂度从 O(n²) 降低到 O(n log n)
}
```

- **效果**: 将 `get_evenness_da()` 的时间复杂度从 O(n²) 降低到 O(n log n)
- **性能提升**: 对于 1000 个文本部分，从 500,000 次操作降低到 ~10,000 次

#### 共享数据结构

```rust
let corpus_part_sizes = Arc::new(corpus_part_sizes);
```

- **效果**: 减少内存分配和复制开销
- **适用场景**: 批处理大量数据时

### 2. 代码质量改进

#### 自定义错误类型

```rust
#[derive(Debug)]
pub enum AnalysisError {
    InvalidInput(String),
    ComputationError(String),
}
```

- **效果**: 更精确的错误分类和处理
- **维护性**: 更容易调试和维护

#### 常量定义

```rust
const FLOAT_EPSILON: f64 = 1e-12;
```

- **效果**: 统一浮点数比较精度
- **可维护性**: 便于调整和修改

#### 延迟计算

```rust
fn compute_stats(&mut self) {
    if self.stats.is_some() {
        return; // 已经计算过
    }
    // 计算统计数据...
}
```

- **效果**: 只在需要时计算，避免不必要的开销

## 性能基准测试建议

### 测试场景

1. **小规模数据**: 10-100 个文本部分
2. **中等规模数据**: 1,000-10,000 个文本部分  
3. **大规模数据**: 100,000+ 个文本部分

### 预期性能提升

- **单次计算**: 20-30% 性能提升（通过缓存统计数据）
- **批处理**: 40-60% 性能提升（通过共享数据和并行处理）
- **大规模 evenness 计算**: 90%+ 性能提升（O(n²) → O(n log n)）

## 使用建议

### 1. 何时使用原版本

- 小规模数据处理（<100 个文本部分）
- 只需要计算单一指标
- 内存使用优先于性能

### 2. 何时使用优化版本

- 大规模数据处理（>1000 个文本部分）
- 需要计算多个指标
- 批处理场景
- 性能优先的应用

### 3. 迁移建议

- API 兼容性: 优化版本保持了相同的 Python 接口
- 测试: 建议在实际数据上进行性能测试
- 逐步迁移: 可以先在批处理场景中测试优化版本

## 进一步优化建议

### 1. SIMD 向量化

- 使用 SIMD 指令优化数值计算
- 特别适用于方差和均值计算

### 2. 更高级的并行化

- 在单个分析器内部使用并行计算
- 适用于超大规模语料库

### 3. 内存池

- 预分配内存池避免频繁分配
- 适用于高频率的批处理场景

### 4. 近似算法

- 对于超大规模数据，考虑使用近似算法
- 在精度和性能之间找到平衡

## 新增功能实现

### Jensen-Shannon Divergence (JSD) 离散度
```rust
pub fn get_jsd_dispersion(&self) -> Option<f64> {
    // 计算观察频率分布 P 和期望频率分布 Q
    // 使用 M = 0.5 * (P + Q) 作为平均分布
    // JS散度 = 0.5 * (KL(P||M) + KL(Q||M))
}
```
- **特点**: 对称的散度度量，比 KL 散度更稳定
- **范围**: [0, 1]，其中 0 表示完全均匀分布，1 表示最大分散

### Hellinger Distance 离散度
```rust
pub fn get_hellinger_dispersion(&self) -> Option<f64> {
    // Hellinger 距离 = sqrt(0.5 * sum((sqrt(p_i) - sqrt(q_i))^2))
    // 归一化到 [0,1] 范围
}
```
- **特点**: 基于概率分布的几何距离
- **范围**: [0, 1]，归一化后的 Hellinger 距离
- **优势**: 对极值不敏感，数值稳定性好

## 完整功能列表

优化版本现在提供了完整的 15 个离散度指标：

1. **基础统计指标**:
   - Range (范围)
   - Standard Deviation (标准差)
   - Variation Coefficient (变异系数)

2. **经典离散度指标**:
   - Juilland's D
   - Carroll's D2
   - Rosengren's S (adjusted)

3. **分布距离指标**:
   - DP (Deviation of Proportions)
   - DP normalized
   - KL Divergence
   - **JSD Dispersion** ✅ 新增
   - **Hellinger Dispersion** ✅ 新增

4. **频率和分布指标**:
   - Mean Text Frequency (Ft)
   - Pervasiveness (Pt)
   - Evenness (Da)
   - Ft adjusted by Pt
   - Ft adjusted by Da

## 总结

优化后的代码在保持 API 兼容性的同时，显著提升了性能和代码质量。主要改进包括：

1. **算法优化**: 将关键算法从 O(n²) 优化到 O(n log n)
2. **缓存机制**: 避免重复计算，提升整体性能
3. **内存优化**: 减少不必要的数据复制
4. **代码质量**: 更好的错误处理和代码结构
5. **完整实现**: 补充了 JSD 和 Hellinger 离散度指标

这些改进使得代码更适合处理大规模语料库分析任务，同时提供了完整的离散度分析工具集，保持了良好的可维护性。
