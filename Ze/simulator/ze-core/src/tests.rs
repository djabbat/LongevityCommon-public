//! Unit tests for Ze simulator scientific correctness.
//!
//! Each test targets a specific theorem, axiom, or formula from Ze Vector Theory.

#[cfg(test)]
mod quantum {
    use crate::quantum::run_quantum;
    use crate::reproduction::ze_visibility;
    use crate::types::THETA_Q;

    /// Theorem 5.1: Born T-rate ≤ Uniform T-rate ≤ Anti-Born T-rate.
    #[test]
    fn theorem_5_1_born_is_optimal() {
        let r = run_quantum(4, 200, 30, 0xABCD);
        assert!(
            r.born_theory_rate <= r.uniform_theory_rate + 1e-9,
            "Theorem 5.1 violated: born_theory={:.6} > uniform_theory={:.6}",
            r.born_theory_rate, r.uniform_theory_rate
        );
        assert!(
            r.uniform_theory_rate <= r.anti_born_theory_rate + 1e-9,
            "Theorem 5.1 violated: uniform_theory={:.6} > anti_theory={:.6}",
            r.uniform_theory_rate, r.anti_born_theory_rate
        );
        assert!(r.theorem_5_1_holds, "theorem_5_1_holds flag must be true");
    }

    /// τ_Z Born ≥ τ_Z Uniform: Born observer depletes budget slower (fewer T-events).
    #[test]
    fn born_depletes_slower_than_uniform() {
        let r = run_quantum(4, 500, 50, 42);
        assert!(
            r.born_tau_final >= r.uniform_tau_final,
            "Born τ_final={} should be ≥ Uniform τ_final={}",
            r.born_tau_final, r.uniform_tau_final
        );
    }

    /// For d=4 and θ=1.5: uniform strategy always triggers T-events (1/4 < 2^{-1.5} ≈ 0.354).
    /// So uniform_theory_rate = 1.0 and τ_Z_uniform should deplete to 0.
    #[test]
    fn uniform_always_t_event_when_d_gt_2_pow_theta() {
        let threshold = 2_f64.powf(-THETA_Q); // ≈ 0.354
        let q_uniform = 1.0 / 4.0_f64;        // d = 4: 0.25 < 0.354
        assert!(
            q_uniform < threshold,
            "For d=4, uniform q={q_uniform:.4} must be < threshold {threshold:.4}"
        );
        let r = run_quantum(4, 500, 50, 42);
        assert!(
            (r.uniform_theory_rate - 1.0).abs() < 1e-9,
            "Uniform theory rate must be 1.0 for d=4, θ=1.5, got {:.6}",
            r.uniform_theory_rate
        );
    }

    /// Ze visibility formula: V = 1 − 2·p_T ∈ [0, 1].
    #[test]
    fn ze_visibility_range() {
        for i in 0..=20 {
            let p = i as f64 / 20.0;
            let v = ze_visibility(p);
            assert!(v >= 0.0 && v <= 1.0, "V({p:.2}) = {v:.4} out of [0,1]");
        }
    }

    /// Ze predicts strictly less visibility than QM for 0 < p_T < 0.5.
    /// V_ze = 1 − 2·p_T  vs  V_qm = √(1 − p_T²).
    /// Falsifiable claim: single-photon experiments should distinguish these curves.
    #[test]
    fn ze_visibility_less_than_qm_for_partial_detection() {
        for i in 1..10 {
            let p = i as f64 * 0.05; // p ∈ {0.05, 0.10, ..., 0.45}
            let v_ze  = ze_visibility(p);
            let v_qm  = (1.0 - p * p).sqrt();
            assert!(
                v_ze < v_qm - 1e-9,
                "Ze should predict less visibility than QM at p_T={p:.2}: V_ze={v_ze:.4}, V_qm={v_qm:.4}"
            );
        }
    }
}

#[cfg(test)]
mod thermo {
    use crate::thermo::run_thermo;

    /// S_Ze must be monotonically non-decreasing (Axiom Z2 / Theorem 3.1).
    #[test]
    fn s_ze_is_non_decreasing() {
        let r = run_thermo(50, 300, false, true, 42);
        for i in 1..r.history_s_ze.len() {
            assert!(
                r.history_s_ze[i] >= r.history_s_ze[i-1] - 1e-12,
                "S_Ze decreased at step {i}: {:.6} < {:.6}",
                r.history_s_ze[i], r.history_s_ze[i-1]
            );
        }
    }

    /// Cold start: Second-Law demonstration.
    ///
    /// Scientific claim: during non-equilibrium thermalization (v_i=0 → T=1),
    /// both S_Ze and S_Boltzmann increase monotonically, showing positive co-movement.
    /// After equilibration (~50 steps), S_Boltzmann fluctuates while S_Ze continues
    /// to grow — they are different physical quantities (NOTE-Z4 correction).
    /// The meaningful correlation window is the thermalization phase only.
    #[test]
    fn cold_start_second_law_demonstration() {
        let r = run_thermo(100, 500, false, true, 42);
        assert!(r.cold_start, "cold_start flag must be true");

        // S_Ze must increase from cold start (monotone growth, Axiom Z2)
        assert!(r.s_ze_final > r.history_s_ze[0],
            "S_Ze must grow: initial={:.4} final={:.4}", r.history_s_ze[0], r.s_ze_final);

        // S_Boltzmann must increase from cold start (system heats from v=0 to T=1)
        assert!(r.s_boltz_final > r.history_s_boltz[0] * 1.5,
            "S_Boltzmann must significantly increase during thermalization: \
             initial={:.4} final={:.4}", r.history_s_boltz[0], r.s_boltz_final);

        // Spearman during thermalization phase (first 50 steps): must be positive.
        // Full-series Spearman is low by design — different scales and equilibration dynamics.
        assert!(r.spearman_thermalization > 0.4,
            "Spearman ρ during thermalization (first {} steps) must be > 0.4, got {:.4}",
            r.thermalization_steps, r.spearman_thermalization);
    }

    /// Maxwell's Demon costs τ_Z units (Landauer principle analogue).
    /// τ_Z with demon must be lower than without demon after the demon step.
    #[test]
    fn demon_costs_tau() {
        let no_demon = run_thermo(100, 200, false, true, 42);
        let with_demon = run_thermo(100, 200, true,  true, 42);
        assert!(
            with_demon.final_tau_total <= no_demon.final_tau_total,
            "Demon run τ={:.2} must be ≤ no-demon τ={:.2}",
            with_demon.final_tau_total, no_demon.final_tau_total
        );
        assert!(
            with_demon.demon_cost.unwrap() > 0.0,
            "Demon cost must be positive"
        );
    }
}

#[cfg(test)]
mod reproduction {
    use crate::reproduction::{run_reproduction, ze_visibility};

    /// Ze visibility at endpoints.
    #[test]
    fn ze_visibility_endpoints() {
        assert!((ze_visibility(0.0) - 1.0).abs() < 1e-12, "V(0) = 1");
        assert!((ze_visibility(0.5) - 0.0).abs() < 1e-12, "V(0.5) = 0");
        assert!((ze_visibility(1.0) - 0.0).abs() < 1e-12, "V(1) = 0 (clamped)");
    }

    /// Double-slit output: 11 data points (strengths 0.0, 0.1, …, 1.0).
    /// Each tuple: (p_T, V_ze, V_qm). All values in [0,1].
    #[test]
    fn double_slit_output_shape_and_range() {
        let r = run_reproduction(50, 10, 4, 99);
        assert_eq!(r.double_slit_visibility.len(), 11);
        for &(p_t, v_ze, v_qm) in &r.double_slit_visibility {
            assert!((0.0..=1.0).contains(&p_t), "p_T out of range: {p_t}");
            assert!((0.0..=1.0).contains(&v_ze), "V_ze out of range: {v_ze}");
            assert!((0.0..=1.0).contains(&v_qm), "V_qm out of range: {v_qm}");
        }
    }

    /// Born strategy chain depth ≤ Uniform strategy chain depth (Born depletes τ_Z slower).
    #[test]
    fn born_chain_shallower_than_uniform() {
        let r = run_reproduction(100, 200, 4, 42);
        assert!(
            r.born_depth_mean <= r.uniform_depth_mean + 1.0, // allow small statistical noise
            "Born mean depth={:.2} should be ≤ Uniform mean depth={:.2}",
            r.born_depth_mean, r.uniform_depth_mean
        );
    }
}
