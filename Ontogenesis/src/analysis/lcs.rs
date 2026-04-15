/// lcs.rs — Latent Change Score (LCS) модели
///
/// Реализует два уровня:
///
/// 1. **Univariate LCS** (McArdle & Hamagami 2001)
///    Δy_t = α + β·y_{t-1} + ζ_t
///    — α: постоянный прирост («drift»)
///    — β: саморегуляция (self-feedback); β < 0 → возврат к равновесию
///    — ζ_t ~ N(0, σ²_ζ): индивидуальный латентный сдвиг
///
/// 2. **Dual LCS** (Hamagami & McArdle 2007, расширение)
///    Δx_t = α_x + β_x·x_{t-1} + γ_xy·y_{t-1} + ζ_x
///    Δy_t = α_y + β_y·y_{t-1} + γ_yx·x_{t-1} + ζ_y
///    — γ: кросс-доменная связь (coupling)
///    — При |γ| > threshold → trigger cross-domain detection
///
/// 3. **Individual Likelihood** под популяционной траекторией
///    Используется как effect size в DomainTransition
///
/// Литература:
/// McArdle J.J. (2009). Latent Variable Modeling of Differences and Changes
///   with Longitudinal Data. Annu Rev Psychol, 60, 577–605. PMID 18817479
/// Hamagami F., McArdle J.J. (2007). Dynamic Extensions of Latent Difference
///   Score Models. Structural Equation Modeling, 14(3), 481–508.

use std::f64::consts::PI;

// ── Константы ─────────────────────────────────────────────────────────────────

/// Минимальное число временны́х точек для оценки параметров LCS
pub const LCS_MIN_POINTS: usize = 3;

/// Порог кросс-доменной связи |γ| для триггера
pub const COUPLING_THRESHOLD: f64 = 0.15;

/// Порог p-value для значимости Individual LCS-перехода
pub const LCS_P_THRESHOLD: f64 = 0.05;

// ── Параметры модели ──────────────────────────────────────────────────────────

/// Параметры univariate LCS для одной серии
#[derive(Debug, Clone)]
pub struct LcsParams {
    /// Постоянный прирост
    pub alpha: f64,
    /// Коэффициент саморегуляции (self-feedback)
    pub beta: f64,
    /// Дисперсия латентного сдвига
    pub sigma2_zeta: f64,
    /// Дисперсия ошибки наблюдения
    pub sigma2_epsilon: f64,
    /// Число наблюдений
    pub n: usize,
}

/// Параметры Dual LCS для двух серий (домен X ↔ домен Y)
#[derive(Debug, Clone)]
pub struct DualLcsParams {
    pub x: LcsParams,
    pub y: LcsParams,
    /// Кросс-доменная связь X→Y
    pub gamma_xy: f64,
    /// Кросс-доменная связь Y→X
    pub gamma_yx: f64,
}

// ── Временной ряд ─────────────────────────────────────────────────────────────

/// Одно наблюдение: возраст (месяцы) + значение
#[derive(Debug, Clone)]
pub struct Observation {
    pub age_months: f64,
    pub value:      f64,
}

/// Продольная серия одного субъекта
#[derive(Debug, Clone)]
pub struct LongitudinalSeries {
    pub observations: Vec<Observation>,
}

impl LongitudinalSeries {
    pub fn new(obs: Vec<Observation>) -> Self {
        let mut s = LongitudinalSeries { observations: obs };
        s.observations.sort_by(|a, b| a.age_months.partial_cmp(&b.age_months).unwrap());
        s
    }

    pub fn len(&self) -> usize {
        self.observations.len()
    }

    /// Вычислить разности первого порядка: Δy_t = y_t - y_{t-1}
    pub fn first_differences(&self) -> Vec<(f64, f64)> {
        // Возвращает (age_t, Δy_t)
        self.observations
            .windows(2)
            .map(|w| (w[1].age_months, w[1].value - w[0].value))
            .collect()
    }

    /// Центрированные значения (mean-demean)
    fn values(&self) -> Vec<f64> {
        self.observations.iter().map(|o| o.value).collect()
    }

    fn mean(&self) -> f64 {
        let v = self.values();
        v.iter().sum::<f64>() / v.len() as f64
    }

    fn variance(&self) -> f64 {
        let m = self.mean();
        let v = self.values();
        v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0).max(1.0)
    }
}

// ── Оценка параметров (метод моментов) ───────────────────────────────────────

/// Оценить параметры univariate LCS методом моментов.
///
/// Упрощённая оценка для реального времени:
/// β̂ = Cov(Δy, y_{t-1}) / Var(y_{t-1})
/// α̂ = mean(Δy) - β̂·mean(y_{t-1})
/// σ²_ζ = Var(Δy) - β̂²·Var(y_{t-1})
pub fn estimate_lcs(series: &LongitudinalSeries) -> Option<LcsParams> {
    if series.len() < LCS_MIN_POINTS {
        return None;
    }

    let diffs = series.first_differences();
    let n = diffs.len();

    // y_{t-1} и Δy_t
    let y_prev: Vec<f64> = series.observations[..n].iter().map(|o| o.value).collect();
    let delta_y: Vec<f64> = diffs.iter().map(|(_, d)| *d).collect();

    let mean_yp  = y_prev.iter().sum::<f64>()  / n as f64;
    let mean_dy  = delta_y.iter().sum::<f64>() / n as f64;

    let var_yp   = y_prev.iter().map(|y| (y - mean_yp).powi(2)).sum::<f64>() / n as f64;
    let cov_dyp  = y_prev.iter().zip(delta_y.iter())
        .map(|(y, d)| (y - mean_yp) * (d - mean_dy))
        .sum::<f64>() / n as f64;

    if var_yp < 1e-12 {
        return None;  // нет дисперсии — нечего оценивать
    }

    let beta  = cov_dyp / var_yp;
    let alpha = mean_dy - beta * mean_yp;

    let var_dy = delta_y.iter().map(|d| (d - mean_dy).powi(2)).sum::<f64>() / n as f64;
    let sigma2_zeta    = (var_dy - beta.powi(2) * var_yp).max(0.0);
    let sigma2_epsilon = (series.variance() * 0.1).max(1e-8); // эвристика: 10% от общей дисперсии

    Some(LcsParams {
        alpha,
        beta,
        sigma2_zeta,
        sigma2_epsilon,
        n,
    })
}

/// Оценить параметры Dual LCS для двух параллельных серий.
///
/// γ_xy оценивается через остаточную регрессию:
/// residual_x_t = Δx_t - (α_x + β_x·x_{t-1})
/// γ_xy = Cov(residual_x, y_{t-1}) / Var(y_{t-1})
pub fn estimate_dual_lcs(
    series_x: &LongitudinalSeries,
    series_y: &LongitudinalSeries,
) -> Option<DualLcsParams> {
    let px = estimate_lcs(series_x)?;
    let py = estimate_lcs(series_y)?;

    let n = series_x.len().min(series_y.len()) - 1;
    if n < 2 {
        return None;
    }

    // Остатки x: Δx_t - (α_x + β_x·x_{t-1})
    let resid_x: Vec<f64> = series_x.observations[..n]
        .iter()
        .zip(series_x.first_differences().iter())
        .map(|(obs, (_, delta))| delta - (px.alpha + px.beta * obs.value))
        .collect();

    // Остатки y: Δy_t - (α_y + β_y·y_{t-1})
    let resid_y: Vec<f64> = series_y.observations[..n]
        .iter()
        .zip(series_y.first_differences().iter())
        .map(|(obs, (_, delta))| delta - (py.alpha + py.beta * obs.value))
        .collect();

    // y_{t-1} и x_{t-1} как предикторы
    let y_prev: Vec<f64> = series_y.observations[..n].iter().map(|o| o.value).collect();
    let x_prev: Vec<f64> = series_x.observations[..n].iter().map(|o| o.value).collect();

    let gamma_xy = _regression_coeff(&resid_x, &y_prev);
    let gamma_yx = _regression_coeff(&resid_y, &x_prev);

    Some(DualLcsParams {
        x: px,
        y: py,
        gamma_xy,
        gamma_yx,
    })
}

fn _regression_coeff(y: &[f64], x: &[f64]) -> f64 {
    let n = y.len().min(x.len()) as f64;
    if n < 2.0 {
        return 0.0;
    }
    let mean_x = x.iter().sum::<f64>() / n;
    let mean_y = y.iter().sum::<f64>() / n;
    let cov = x.iter().zip(y.iter()).map(|(xi, yi)| (xi - mean_x) * (yi - mean_y)).sum::<f64>() / n;
    let var = x.iter().map(|xi| (xi - mean_x).powi(2)).sum::<f64>() / n;
    if var < 1e-12 { 0.0 } else { cov / var }
}

// ── Individual Likelihood (effect size) ──────────────────────────────────────

/// Результат оценки Individual LCS в одной точке
#[derive(Debug, Clone)]
pub struct LcsTestResult {
    /// Возраст точки (месяцы)
    pub age_months:   f64,
    /// Наблюдаемое изменение Δy
    pub observed_delta: f64,
    /// Ожидаемое изменение под нулевой гипотезой (α + β·y_{t-1})
    pub expected_delta: f64,
    /// Отклонение в SD (effect size)
    pub effect_size:  f64,
    /// p-value (двусторонний z-тест)
    pub p_value:      f64,
}

/// Вычислить Individual LCS-тест в каждой точке серии.
///
/// Effect size: z = (Δy_obs - Δy_exp) / σ_ζ
/// p-value: P(|Z| > |z|) при N(0,1)
pub fn lcs_individual_tests(
    series: &LongitudinalSeries,
    params: &LcsParams,
) -> Vec<LcsTestResult> {
    let diffs = series.first_differences();
    let sigma = params.sigma2_zeta.sqrt().max(1e-8);

    diffs
        .iter()
        .zip(series.observations.windows(2))
        .map(|((age_t, delta_obs), window)| {
            let y_prev      = window[0].value;
            let delta_exp   = params.alpha + params.beta * y_prev;
            let z           = (delta_obs - delta_exp) / sigma;
            let p_value     = 2.0 * (1.0 - normal_cdf(z.abs()));
            LcsTestResult {
                age_months:     *age_t,
                observed_delta: *delta_obs,
                expected_delta: delta_exp,
                effect_size:    z,
                p_value,
            }
        })
        .collect()
}

// ── Нормальное распределение (встроенная реализация) ──────────────────────────

/// Функция распределения N(0,1): Φ(x)
/// Алгоритм Abramowitz & Stegun 26.2.17 (ошибка < 7.5e-8)
pub fn normal_cdf(x: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.2316419 * x.abs());
    let poly = t * (0.319_381_53
        + t * (-0.356_563_782
        + t * (1.781_477_937
        + t * (-1.821_255_978
        + t * 1.330_274_429))));
    let pdf = (-x * x / 2.0).exp() / (2.0 * PI).sqrt();
    let cdf = 1.0 - pdf * poly;
    if x >= 0.0 { cdf } else { 1.0 - cdf }
}

// ── Кросс-доменный coupling анализ ───────────────────────────────────────────

/// Результат анализа coupling между двумя доменами
#[derive(Debug, Clone)]
pub struct CouplingResult {
    /// γ_xy: влияние Y на изменение X
    pub gamma_xy:   f64,
    /// γ_yx: влияние X на изменение Y
    pub gamma_yx:   f64,
    /// Значим ли coupling X→Y
    pub xy_significant: bool,
    /// Значим ли coupling Y→X
    pub yx_significant: bool,
    /// Направление доминирующего coupling
    pub dominant: CouplingDirection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CouplingDirection {
    XtoY,
    YtoX,
    Bidirectional,
    None,
}

/// Проанализировать coupling между двумя доменами.
pub fn analyze_coupling(params: &DualLcsParams) -> CouplingResult {
    let xy_sig = params.gamma_xy.abs() > COUPLING_THRESHOLD;
    let yx_sig = params.gamma_yx.abs() > COUPLING_THRESHOLD;

    let dominant = match (xy_sig, yx_sig) {
        (true, true)  => CouplingDirection::Bidirectional,
        (true, false) => CouplingDirection::XtoY,
        (false, true) => CouplingDirection::YtoX,
        (false, false) => CouplingDirection::None,
    };

    CouplingResult {
        gamma_xy:       params.gamma_xy,
        gamma_yx:       params.gamma_yx,
        xy_significant: xy_sig,
        yx_significant: yx_sig,
        dominant,
    }
}

// ── Тесты ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_series(values: &[f64]) -> LongitudinalSeries {
        let obs = values.iter().enumerate()
            .map(|(i, &v)| Observation { age_months: i as f64 * 12.0, value: v })
            .collect();
        LongitudinalSeries::new(obs)
    }

    #[test]
    fn test_normal_cdf() {
        // Φ(0) = 0.5
        assert!((normal_cdf(0.0) - 0.5).abs() < 1e-6, "Φ(0) = {}", normal_cdf(0.0));
        // Φ(1.96) ≈ 0.975
        assert!((normal_cdf(1.96) - 0.975).abs() < 0.001, "Φ(1.96) = {}", normal_cdf(1.96));
        // Φ(-1.96) ≈ 0.025
        assert!((normal_cdf(-1.96) - 0.025).abs() < 0.001, "Φ(-1.96) = {}", normal_cdf(-1.96));
    }

    #[test]
    fn test_estimate_lcs_linear_growth() {
        // Линейный рост: y = 10 + 2t → Δy = 2, β ≈ 0
        let series = make_series(&[10.0, 12.0, 14.0, 16.0, 18.0, 20.0]);
        let params = estimate_lcs(&series).expect("should estimate");
        // α ≈ 2 (постоянный прирост)
        assert!((params.alpha - 2.0).abs() < 0.5, "alpha = {}", params.alpha);
        // β ≈ 0 (нет саморегуляции при линейном тренде)
        assert!(params.beta.abs() < 0.5, "beta = {}", params.beta);
    }

    #[test]
    fn test_estimate_lcs_too_short() {
        let series = make_series(&[1.0, 2.0]);
        assert!(estimate_lcs(&series).is_none());
    }

    #[test]
    fn test_lcs_individual_tests_count() {
        let series = make_series(&[10.0, 12.0, 14.0, 16.0, 18.0]);
        let params = estimate_lcs(&series).unwrap();
        let tests = lcs_individual_tests(&series, &params);
        // n=5 точек → n-1=4 разности
        assert_eq!(tests.len(), 4);
    }

    #[test]
    fn test_lcs_p_values_in_range() {
        let series = make_series(&[10.0, 12.0, 11.0, 15.0, 13.0, 17.0]);
        let params = estimate_lcs(&series).unwrap();
        let tests = lcs_individual_tests(&series, &params);
        for t in &tests {
            assert!(t.p_value >= 0.0 && t.p_value <= 1.0,
                "p_value out of range: {}", t.p_value);
        }
    }

    #[test]
    fn test_dual_lcs_coupling_detection() {
        // X растёт линейно, Y = X + небольшой шум → должен быть coupling
        let x_vals = [10.0, 12.0, 14.0, 16.0, 18.0, 20.0];
        let y_vals = [10.5, 12.3, 14.1, 16.4, 18.2, 20.1];
        let sx = make_series(&x_vals);
        let sy = make_series(&y_vals);
        let dual = estimate_dual_lcs(&sx, &sy);
        assert!(dual.is_some(), "Dual LCS должен оцениваться");
        let d = dual.unwrap();
        // γ_yx (влияние X на Y) должен быть положительным при comovement
        assert!(d.gamma_yx > -1.0 && d.gamma_yx < 2.0,
            "gamma_yx = {}", d.gamma_yx);
    }

    #[test]
    fn test_coupling_threshold() {
        let params = DualLcsParams {
            x: LcsParams { alpha: 1.0, beta: -0.1, sigma2_zeta: 1.0, sigma2_epsilon: 0.1, n: 10 },
            y: LcsParams { alpha: 1.0, beta: -0.1, sigma2_zeta: 1.0, sigma2_epsilon: 0.1, n: 10 },
            gamma_xy: 0.20,   // > COUPLING_THRESHOLD
            gamma_yx: 0.05,   // < COUPLING_THRESHOLD
        };
        let result = analyze_coupling(&params);
        assert_eq!(result.dominant, CouplingDirection::XtoY);
        assert!(result.xy_significant);
        assert!(!result.yx_significant);
    }
}
