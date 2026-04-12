//! Instinct learning: habituate, sensitize, generalize, specialize.

use crate::{InstinctCortex, InstinctId};

const HABITUATION_FACTOR: f64 = 0.95;  // 0x95 → reduces sensitivity
const SENSITIZATION_FACTOR: f64 = 1.05; // increases sensitivity

/// Repeated activation reduces sensitivity. Returns new decay_rate.
pub fn habituate(cortex: &mut InstinctCortex, instinct_id: InstinctId) -> f64 {
    if let Some(instinct) = cortex.instincts.get_mut(&instinct_id) {
        instinct.decay_rate = (instinct.decay_rate * HABITUATION_FACTOR).max(0.1);
        instinct.intensity *= 0.9; // also reduce current intensity
        instinct.intensity = instinct.intensity.clamp(0.0, 1.0);
        instinct.decay_rate
    } else {
        0.95
    }
}

/// Strong stimulus increases sensitivity. Returns new decay_rate.
pub fn sensitize(cortex: &mut InstinctCortex, instinct_id: InstinctId) -> f64 {
    if let Some(instinct) = cortex.instincts.get_mut(&instinct_id) {
        instinct.decay_rate = (instinct.decay_rate * SENSITIZATION_FACTOR).min(0.999);
        instinct.decay_rate
    } else {
        0.95
    }
}

/// Transfer learning between related instincts. Returns overlap score.
pub fn generalize(
    cortex: &mut InstinctCortex,
    from: InstinctId,
    to: InstinctId,
) -> f64 {
    let from_intensity = cortex
        .instincts
        .get(&from)
        .map(|i| i.intensity)
        .unwrap_or(0.0);
    if from_intensity < 0.01 {
        return 0.0;
    }
    let transfer = from_intensity * 0.25;
    if let Some(target) = cortex.instincts.get_mut(&to) {
        target.intensity = (target.intensity + transfer).clamp(0.0, 1.0);
    }
    transfer
}

/// Focus instinct to a specific context (increases intensity, narrows applicability).
/// Returns new intensity.
pub fn specialize(cortex: &mut InstinctCortex, instinct_id: InstinctId, _context: &str) -> f64 {
    if let Some(instinct) = cortex.instincts.get_mut(&instinct_id) {
        // Specialization increases intensity in this context
        instinct.intensity = (instinct.intensity * 1.2).clamp(0.0, 1.0);
        // But increases decay rate — more contextual, shorter-lived
        instinct.decay_rate = (instinct.decay_rate * 0.9).max(0.1);
        instinct.intensity
    } else {
        0.0
    }
}
