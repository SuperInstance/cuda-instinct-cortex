//! Instinct processing pipeline — activation, cycle processing, reflex, deliberation.

use crate::{InstinctCortex, InstinctId};

/// Result of activating an instinct.
#[derive(Debug, Clone)]
pub struct ActivationResult {
    pub instinct: InstinctId,
    pub old_intensity: f64,
    pub new_intensity: f64,
    pub activated: bool,
}

/// A behavioral option produced by the processing cycle.
#[derive(Debug, Clone)]
pub struct BehavioralUrgency {
    pub instinct: InstinctId,
    pub urgency_score: f64,
    pub suggested_action: String,
    pub energy_cost: f64,
}

/// Reflex action — bypasses deliberation.
#[derive(Debug, Clone)]
pub struct ReflexAction {
    pub instinct: InstinctId,
    pub response: String,
    pub magnitude: f64,
}

/// A decision from deliberation.
#[derive(Debug, Clone)]
pub struct Decision {
    pub chosen: InstinctId,
    pub confidence: f64,
    pub action: String,
    pub rejected: Vec<InstinctId>,
}

/// Activate a specific instinct in response to a stimulus.
pub fn activate(
    cortex: &mut InstinctCortex,
    instinct_id: InstinctId,
    stimulus: f64,
    tick: u64,
) -> ActivationResult {
    let old = cortex
        .instincts
        .get(&instinct_id)
        .map(|i| i.intensity)
        .unwrap_or(0.0);
    cortex
        .instincts
        .entry(instinct_id)
        .and_modify(|instinct| instinct.activate(stimulus, tick))
        .or_insert_with(|| {
            let mut i = crate::Instinct::new(instinct_id);
            i.activate(stimulus, tick);
            i
        });
    let new = cortex.instincts.get(&instinct_id).map(|i| i.intensity).unwrap_or(0.0);
    ActivationResult {
        instinct: instinct_id,
        old_intensity: old,
        new_intensity: new,
        activated: new > old,
    }
}

/// Process one full cycle: rank all instincts by urgency.
pub fn process_cycle(cortex: &InstinctCortex, _environment: &str) -> Vec<BehavioralUrgency> {
    let mut urgencies: Vec<BehavioralUrgency> = cortex
        .instincts
        .values()
        .filter(|i| i.intensity > 0.01)
        .map(|instinct| {
            let id = InstinctId::from_opcode(instinct.id).unwrap_or(InstinctId::Load);
            BehavioralUrgency {
                instinct: id,
                urgency_score: instinct.intensity,
                suggested_action: suggest_action(id, instinct.intensity),
                energy_cost: instinct.intensity * 0.1,
            }
        })
        .collect();

    urgencies.sort_by(|a, b| b.urgency_score.partial_cmp(&a.urgency_score).unwrap());
    urgencies
}

/// Check if any instinct triggers a reflex (intensity > 0.8).
pub fn reflex_check(cortex: &InstinctCortex, stimulus: f64) -> Option<ReflexAction> {
    for (&id, instinct) in &cortex.instincts {
        if instinct.intensity > 0.8 && stimulus > 0.5 {
            return Some(ReflexAction {
                instinct: id,
                response: format!("Reflex: {:?} triggered (intensity={:.3})", id, instinct.intensity),
                magnitude: instinct.intensity,
            });
        }
    }
    None
}

/// Deliberate among competing behavioral options.
pub fn deliberate(cortex: &InstinctCortex, options: &[BehavioralUrgency]) -> Decision {
    if options.is_empty() {
        return Decision {
            chosen: InstinctId::Decay,
            confidence: 0.0,
            action: "No viable options".into(),
            rejected: vec![],
        };
    }

    // Weight by urgency × energy affordability
    let weighted: Vec<(usize, f64)> = options
        .iter()
        .enumerate()
        .map(|(i, o)| {
            let affordability = if cortex.total_energy >= o.energy_cost { 1.0 } else { 0.3 };
            (i, o.urgency_score * affordability)
        })
        .collect();

    let best_idx = weighted
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(i, _)| *i)
        .unwrap_or(0);

    let rejected: Vec<InstinctId> = options
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != best_idx)
        .map(|(_, o)| o.instinct)
        .collect();

    Decision {
        chosen: options[best_idx].instinct,
        confidence: options[best_idx].urgency_score,
        action: options[best_idx].suggested_action.clone(),
        rejected,
    }
}

fn suggest_action(id: InstinctId, intensity: f64) -> String {
    match id {
        InstinctId::Load if intensity > 0.5 => "Acquire resources aggressively".into(),
        InstinctId::Load => "Gather available resources".into(),
        InstinctId::Store => "Retain accumulated value".into(),
        InstinctId::Decay => "Release or wind down".into(),
        InstinctId::Reflex => "React immediately".into(),
        InstinctId::Modulate => "Adjust internal state".into(),
        InstinctId::Threshold => "Evaluate boundary conditions".into(),
        InstinctId::Converge => "Align with group".into(),
        InstinctId::Extinct => "Consider pruning".into(),
        InstinctId::Habituate => "Reduce sensitivity to recurring input".into(),
        InstinctId::Sensitize => "Increase alertness".into(),
        InstinctId::Generalize => "Broaden response pattern".into(),
        InstinctId::Specialize => "Narrow focus to context".into(),
        InstinctId::Diffuse => "Spread influence outward".into(),
        InstinctId::Inhibit => "Suppress competing drives".into(),
        InstinctId::Sum => "Aggregate signals".into(),
        InstinctId::Diff => "Detect divergence".into(),
    }
}
