//! Energy homeostasis: balance, allocate, survive.

use crate::{InstinctCortex, InstinctId};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum HomeostasisState {
    Balanced,
    Deficit,
    Surplus,
    Crisis,
}

/// Check current homeostasis state.
pub fn homeostasis_check(cortex: &InstinctCortex) -> HomeostasisState {
    let load = cortex.total_intensity();
    let target = cortex.homeostasis_target;
    let ratio = cortex.total_energy / target.max(0.001);

    if cortex.total_energy < 0.1 {
        HomeostasisState::Crisis
    } else if ratio < 0.7 {
        HomeostasisState::Deficit
    } else if ratio > 1.5 {
        HomeostasisState::Surplus
    } else {
        HomeostasisState::Balanced
    }
}

/// Distribute available energy proportionally among active instincts.
pub fn energy_allocation(
    cortex: &InstinctCortex,
    available_energy: f64,
) -> HashMap<InstinctId, f64> {
    let total_intensity = cortex.total_intensity().max(0.001);
    let mut alloc = HashMap::new();
    for (&id, instinct) in &cortex.instincts {
        if instinct.intensity > 0.01 {
            let share = (instinct.intensity / total_intensity) * available_energy;
            alloc.insert(id, share);
        }
    }
    alloc
}

/// Survival mode: suppress non-essential instincts when energy is critically low.
/// Essential: Load, Reflex, Threshold. Everything else gets suppressed.
pub fn survival_mode(cortex: &mut InstinctCortex) -> Vec<InstinctId> {
    let essential = [InstinctId::Load, InstinctId::Reflex, InstinctId::Threshold];
    let mut suppressed = Vec::new();
    for (&id, instinct) in cortex.instincts.iter_mut() {
        if !essential.contains(&id) && instinct.intensity > 0.0 {
            suppressed.push(id);
            instinct.intensity *= 0.3;
        }
    }
    cortex.total_energy *= 0.5; // emergency energy conservation
    suppressed
}
