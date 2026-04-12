//! Social instinct processing: converge, extinct, diffuse, herd behavior.

use crate::{InstinctCortex, InstinctId};

/// Move an instinct 25% toward the group average value.
pub fn converge(
    cortex: &mut InstinctCortex,
    instinct_id: InstinctId,
    group_values: &[f64],
) -> f64 {
    if group_values.is_empty() {
        return cortex
            .instincts
            .get(&instinct_id)
            .map(|i| i.intensity)
            .unwrap_or(0.0);
    }
    let group_avg: f64 = group_values.iter().sum::<f64>() / group_values.len() as f64;
    if let Some(instinct) = cortex.instincts.get_mut(&instinct_id) {
        instinct.intensity = instinct.intensity * 0.75 + group_avg * 0.25;
        instinct.intensity = instinct.intensity.clamp(0.0, 1.0);
        instinct.intensity
    } else {
        0.0
    }
}

/// Check if an instinct should be pruned (intensity below threshold).
/// Default threshold: 10/255 ≈ 0.0392.
pub fn extinct_check(cortex: &InstinctCortex, instinct_id: InstinctId, threshold: Option<f64>) -> bool {
    let th = threshold.unwrap_or(10.0 / 255.0);
    cortex
        .instincts
        .get(&instinct_id)
        .map(|i| i.intensity < th)
        .unwrap_or(true)
}

/// Spread instinct influence to neighbors. Returns average diffusion.
pub fn diffuse(
    cortex: &mut InstinctCortex,
    instinct_id: InstinctId,
    neighbors: &mut [&mut InstinctCortex],
) -> f64 {
    let source_intensity = cortex
        .instincts
        .get(&instinct_id)
        .map(|i| i.intensity)
        .unwrap_or(0.0);
    if source_intensity < 0.01 || neighbors.is_empty() {
        return 0.0;
    }
    let share = source_intensity * 0.1 / neighbors.len() as f64;
    for neighbor in neighbors.iter_mut() {
        if let Some(instinct) = neighbor.instincts.get_mut(&instinct_id) {
            instinct.intensity = (instinct.intensity + share).clamp(0.0, 1.0);
        }
    }
    share
}

/// Analysis of group-level instinct dynamics.
#[derive(Debug, Clone)]
pub struct HerdAnalysis {
    pub dominant_instinct: InstinctId,
    pub group_alignment: f64,    // 0.0–1.0, how aligned the group is
    pub avg_intensity: f64,
    pub convergence_pressure: f64,
}

/// Analyze herd-level instinct dynamics across multiple cortices.
pub fn herd_behavior(cortex: &InstinctCortex, all_cortices: &[&InstinctCortex]) -> HerdAnalysis {
    if all_cortices.is_empty() {
        let dominant = cortex
            .instincts
            .iter()
            .max_by(|a, b| a.1.intensity.partial_cmp(&b.1.intensity).unwrap())
            .map(|(&id, _)| id)
            .unwrap_or(InstinctId::Decay);
        return HerdAnalysis {
            dominant_instinct: dominant,
            group_alignment: 0.0,
            avg_intensity: cortex.total_intensity(),
            convergence_pressure: 0.0,
        };
    }

    // Find dominant instinct across all cortices
    let mut totals: std::collections::HashMap<InstinctId, f64> = std::collections::HashMap::new();
    for c in std::iter::once(cortex).chain(all_cortices.iter().copied()) {
        for (&id, instinct) in &c.instincts {
            *totals.entry(id).or_insert(0.0) += instinct.intensity;
        }
    }
    let dominant = totals
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(&id, _)| id)
        .unwrap_or(InstinctId::Decay);

    let n = (all_cortices.len() + 1) as f64;
    let avg_intensity = totals.values().sum::<f64>() / n;

    // Alignment: how much the dominant instinct contributes to total
    let total = totals.values().sum::<f64>().max(0.001);
    let group_alignment = totals.get(&dominant).copied().unwrap_or(0.0) / total;

    // Convergence pressure: variance-based heuristic
    let mut intensity_vecs: Vec<Vec<f64>> = std::iter::once(cortex)
        .chain(all_cortices.iter().copied())
        .map(|c| c.instincts.values().map(|i| i.intensity).collect())
        .collect();
    let means: Vec<f64> = (0..16)
        .map(|idx| {
            intensity_vecs
                .iter()
                .map(|v| v.get(idx).copied().unwrap_or(0.0))
                .sum::<f64>()
                / n
        })
        .collect();
    let variance: f64 = (0..16)
        .map(|idx| {
            let mean = means[idx];
            intensity_vecs
                .iter()
                .map(|v| {
                    let diff = v.get(idx).copied().unwrap_or(0.0) - mean;
                    diff * diff
                })
                .sum::<f64>()
        })
        .sum::<f64>()
        / (n * 16.0);
    let convergence_pressure = 1.0 / (1.0 + variance).min(1.0);

    HerdAnalysis {
        dominant_instinct: dominant,
        group_alignment,
        avg_intensity,
        convergence_pressure,
    }
}
