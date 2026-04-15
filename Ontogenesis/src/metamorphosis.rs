/// metamorphosis.rs — Возрастной метаморфоз
///
/// Возрастной метаморфоз = синхронный значимый переход в ≥2 доменах
/// в пределах временного окна WINDOW_MONTHS.
///
/// Алгоритм:
/// 1. Для каждого домена (Morphology, Physiology, Psychology, Sociology)
///    вычислить LCS-переход в каждой временной точке
/// 2. Применить FDR-коррекцию по всем тестам (BH)
/// 3. Детектировать метаморфоз: ≥2 доменов с переходом в одном окне
/// 4. При детекции перехода в домене D → trigger LCS-тест в остальных 3
///
/// Ожидаемое число метаморфозов: ~12 (эмпирически, не постулировано)

use std::collections::HashMap;

// ── Константы ─────────────────────────────────────────────────────────────────

/// Ширина временного окна кластеризации (месяцы)
pub const WINDOW_MONTHS: f64 = 6.0;

/// Минимальное число доменов для объявления метаморфоза
pub const MIN_DOMAINS: usize = 2;

/// FDR порог (Benjamini-Hochberg q*)
pub const FDR_Q: f64 = 0.05;

/// Бонферрони порог (sensitivity analysis)
/// 24 параметра × 120 лет × 12 месяцев ≈ 2880 тестов
pub const BONFERRONI_ALPHA: f64 = 0.05 / 2880.0; // 1.736e-5

// ── Типы ──────────────────────────────────────────────────────────────────────

/// Четыре домена платформы
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Domain {
    Morphology,
    Physiology,
    Psychology,
    Sociology,
}

impl Domain {
    pub fn all() -> [Domain; 4] {
        [
            Domain::Morphology,
            Domain::Physiology,
            Domain::Psychology,
            Domain::Sociology,
        ]
    }

    pub fn others(&self) -> Vec<Domain> {
        Domain::all()
            .into_iter()
            .filter(|d| d != self)
            .collect()
    }
}

/// Пять нейробиологических фаз (Nature Comm 2025)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    I,   // 0–9 лет: Детство
    II,  // 9–32 лет: Подростково-молодёжная
    III, // 32–66 лет: Взрослая
    IV,  // 66–83 лет: Раннее старение
    V,   // 83–120 лет: Позднее старение
}

impl Phase {
    /// Определить фазу по возрасту в годах
    pub fn from_age(age_years: f64) -> Phase {
        match age_years as u32 {
            0..=8    => Phase::I,
            9..=31   => Phase::II,
            32..=65  => Phase::III,
            66..=82  => Phase::IV,
            _        => Phase::V,
        }
    }
}

/// Переход в одном домене в одной временной точке
#[derive(Debug, Clone)]
pub struct DomainTransition {
    pub domain:      Domain,
    pub age_months:  f64,
    pub p_value:     f64,   // до коррекции
    pub p_adjusted:  f64,   // после FDR/Bonferroni
    pub effect_size: f64,   // LCS: Individual Likelihood отклонение в SD
    pub significant: bool,  // после FDR q < 0.05
}

/// Обнаруженный возрастной метаморфоз
#[derive(Debug, Clone)]
pub struct Metamorphosis {
    pub id:          usize,
    pub center_age:  f64,              // возраст центра окна (лет)
    pub phase:       Phase,
    pub domains:     Vec<Domain>,      // домены с переходом
    pub transitions: Vec<DomainTransition>,
    pub strength:    MetamorphosisStrength,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetamorphosisStrength {
    Minor,    // 2 домена
    Major,    // 3 домена
    Complete, // все 4 домена
}

impl MetamorphosisStrength {
    fn from_domain_count(n: usize) -> Self {
        match n {
            2 => MetamorphosisStrength::Minor,
            3 => MetamorphosisStrength::Major,
            _ => MetamorphosisStrength::Complete,
        }
    }
}

// ── FDR Benjamini-Hochberg ────────────────────────────────────────────────────

/// Применить поправку BH к вектору p-значений.
/// Возвращает вектор скорректированных p-значений (q-значений) в том же порядке.
pub fn fdr_bh(p_values: &[f64]) -> Vec<f64> {
    let m = p_values.len();
    if m == 0 {
        return vec![];
    }

    // Сортируем индексы по p-значению
    let mut indexed: Vec<(usize, f64)> = p_values
        .iter()
        .enumerate()
        .map(|(i, &p)| (i, p))
        .collect();
    indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut q_values = vec![1.0f64; m];

    // BH критерий: p_(i) ≤ (i/m) × q*
    // Скорректированное q_(i) = p_(i) × m / i
    let mut min_q = 1.0f64;
    for (rank, &(orig_idx, p)) in indexed.iter().enumerate().rev() {
        let rank1 = rank + 1; // 1-based
        let q = (p * m as f64 / rank1 as f64).min(1.0);
        min_q = min_q.min(q);
        q_values[orig_idx] = min_q;
    }

    q_values
}

// ── Кросс-доменный триггер ────────────────────────────────────────────────────

/// Результат кросс-доменного анализа
#[derive(Debug)]
pub struct CrossDomainResult {
    pub triggered_by: Domain,
    pub tested_domains: Vec<Domain>,
    pub transitions_found: Vec<DomainTransition>,
}

/// При обнаружении перехода в домене `trigger_domain` —
/// запустить LCS-тест в остальных 3 доменах.
/// `lcs_tester` — функция-замыкание, вычисляющая (p_value, effect_size)
/// для заданного домена и возрастной точки.
pub fn cross_domain_trigger<F>(
    trigger_domain: Domain,
    age_months: f64,
    lcs_tester: F,
) -> CrossDomainResult
where
    F: Fn(Domain, f64) -> (f64, f64), // domain, age_months → (p, effect)
{
    let other_domains = trigger_domain.others();
    let raw_results: Vec<(Domain, f64, f64)> = other_domains
        .iter()
        .map(|&d| {
            let (p, eff) = lcs_tester(d, age_months);
            (d, p, eff)
        })
        .collect();

    // FDR по 3 тестам
    let p_vals: Vec<f64> = raw_results.iter().map(|(_, p, _)| *p).collect();
    let q_vals = fdr_bh(&p_vals);

    let transitions_found: Vec<DomainTransition> = raw_results
        .iter()
        .zip(q_vals.iter())
        .map(|((domain, p, eff), q)| DomainTransition {
            domain:      *domain,
            age_months,
            p_value:     *p,
            p_adjusted:  *q,
            effect_size: *eff,
            significant: *q < FDR_Q,
        })
        .collect();

    CrossDomainResult {
        triggered_by: trigger_domain,
        tested_domains: other_domains,
        transitions_found,
    }
}

// ── Детектор метаморфозов ─────────────────────────────────────────────────────

/// Из списка всех переходов по всем доменам
/// выявить метаморфозы (кластеры ≥2 доменов в окне WINDOW_MONTHS).
pub fn detect_metamorphoses(
    all_transitions: &[DomainTransition],
) -> Vec<Metamorphosis> {
    // Только значимые переходы
    let sig: Vec<&DomainTransition> = all_transitions
        .iter()
        .filter(|t| t.significant)
        .collect();

    if sig.is_empty() {
        return vec![];
    }

    // Сортируем по возрасту
    let mut sorted = sig.clone();
    sorted.sort_by(|a, b| a.age_months.partial_cmp(&b.age_months).unwrap());

    let mut metamorphoses: Vec<Metamorphosis> = Vec::new();
    let mut used = vec![false; sorted.len()];

    for i in 0..sorted.len() {
        if used[i] {
            continue;
        }
        let anchor = sorted[i].age_months;
        let window_end = anchor + WINDOW_MONTHS;

        // Собираем все переходы в окне
        let cluster: Vec<usize> = sorted
            .iter()
            .enumerate()
            .filter(|(j, t)| !used[*j] && t.age_months >= anchor && t.age_months <= window_end)
            .map(|(j, _)| j)
            .collect();

        // Уникальные домены в кластере
        let domains_in_cluster: Vec<Domain> = {
            let mut seen: HashMap<Domain, ()> = HashMap::new();
            cluster
                .iter()
                .map(|&j| sorted[j].domain)
                .filter(|&d| seen.insert(d, ()).is_none())
                .collect()
        };

        if domains_in_cluster.len() >= MIN_DOMAINS {
            let transitions: Vec<DomainTransition> =
                cluster.iter().map(|&j| (*sorted[j]).clone()).collect();

            let center_age = transitions.iter().map(|t| t.age_months).sum::<f64>()
                / transitions.len() as f64
                / 12.0; // → годы

            let m = Metamorphosis {
                id: metamorphoses.len() + 1,
                center_age,
                phase: Phase::from_age(center_age),
                strength: MetamorphosisStrength::from_domain_count(domains_in_cluster.len()),
                domains: domains_in_cluster,
                transitions,
            };

            metamorphoses.push(m);

            // Помечаем использованные
            for j in cluster {
                used[j] = true;
            }
        }
    }

    metamorphoses
}

// ── Тесты ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fdr_bh_basic() {
        // m=10, BH с q*=0.05
        // Ранговые пороги: i*0.05/10 = i*0.005
        // rank1: 0.001 ≤ 0.005 ✓  q=0.001*10/1=0.010
        // rank2: 0.008 ≤ 0.010 ✓  q=0.008*10/2=0.040
        // rank3: 0.039 ≤ 0.015 ✗  q=0.039*10/3=0.130
        // → принимаются только p[0]=0.001 и p[1]=0.008
        let p = vec![0.001, 0.008, 0.039, 0.041, 0.042, 0.06, 0.074, 0.205, 0.212, 0.216];
        let q = fdr_bh(&p);
        assert!(q[0] < FDR_Q, "q[0] should pass BH: {}", q[0]);
        assert!(q[1] < FDR_Q, "q[1] should pass BH: {}", q[1]);
        assert!(q[2] >= FDR_Q, "q[2] should fail BH: {}", q[2]);
        assert!(q[5] >= FDR_Q, "q[5] should fail BH: {}", q[5]);
    }

    #[test]
    fn test_phase_assignment() {
        assert_eq!(Phase::from_age(5.0), Phase::I);
        assert_eq!(Phase::from_age(15.0), Phase::II);
        assert_eq!(Phase::from_age(45.0), Phase::III);
        assert_eq!(Phase::from_age(70.0), Phase::IV);
        assert_eq!(Phase::from_age(90.0), Phase::V);
    }

    #[test]
    fn test_detect_metamorphosis_minor() {
        let transitions = vec![
            DomainTransition {
                domain: Domain::Morphology,
                age_months: 132.0, // 11 лет
                p_value: 0.001,
                p_adjusted: 0.001,
                effect_size: 2.5,
                significant: true,
            },
            DomainTransition {
                domain: Domain::Physiology,
                age_months: 135.0, // 11.25 лет
                p_value: 0.002,
                p_adjusted: 0.003,
                effect_size: 3.1,
                significant: true,
            },
            DomainTransition {
                domain: Domain::Sociology,
                age_months: 200.0, // далеко
                p_value: 0.01,
                p_adjusted: 0.01,
                effect_size: 1.8,
                significant: true,
            },
        ];
        let metas = detect_metamorphoses(&transitions);
        assert_eq!(metas.len(), 1);
        assert_eq!(metas[0].domains.len(), 2);
        assert_eq!(metas[0].strength, MetamorphosisStrength::Minor);
    }

    #[test]
    fn test_domain_others() {
        let others = Domain::Morphology.others();
        assert_eq!(others.len(), 3);
        assert!(!others.contains(&Domain::Morphology));
    }

    #[test]
    fn test_bonferroni_value() {
        assert!((BONFERRONI_ALPHA - 1.736e-5).abs() < 1e-7);
    }
}
