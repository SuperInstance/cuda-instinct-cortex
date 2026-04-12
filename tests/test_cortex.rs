use cuda_instinct_cortex::*;

#[test]
fn instinct_id_from_opcode() {
    assert_eq!(InstinctId::from_opcode(0xB0), Some(InstinctId::Load));
    assert_eq!(InstinctId::from_opcode(0xBF), Some(InstinctId::Diff));
    assert_eq!(InstinctId::from_opcode(0x00), None);
}

#[test]
fn instinct_id_all_sixteen() {
    assert_eq!(InstinctId::all().len(), 16);
}

#[test]
fn cortex_creation() {
    let c = InstinctCortex::new();
    assert_eq!(c.instincts.len(), 16);
    assert_eq!(c.total_energy, 1.0);
}

#[test]
fn activate_instinct() {
    let mut c = InstinctCortex::new();
    let r = processing::activate(&mut c, InstinctId::Load, 0.8, 1);
    assert!(r.activated);
    assert!((r.new_intensity - 0.8).abs() < 1e-6);
}

#[test]
fn process_cycle_ranks_by_urgency() {
    let mut c = InstinctCortex::new();
    c.seed(InstinctId::Reflex, 0.9);
    c.seed(InstinctId::Decay, 0.3);
    let urgencies = processing::process_cycle(&c, "test");
    assert!(urgencies.len() >= 2);
    assert_eq!(urgencies[0].instinct, InstinctId::Reflex);
}

#[test]
fn reflex_check_triggers() {
    let mut c = InstinctCortex::new();
    c.seed(InstinctId::Reflex, 0.9);
    let r = processing::reflex_check(&c, 0.6);
    assert!(r.is_some());
    assert_eq!(r.unwrap().instinct, InstinctId::Reflex);
}

#[test]
fn reflex_check_no_trigger() {
    let c = InstinctCortex::new();
    assert!(processing::reflex_check(&c, 0.1).is_none());
}

#[test]
fn deliberation_picks_highest_urgency() {
    let mut c = InstinctCortex::new();
    c.seed(InstinctId::Store, 0.9);
    let options = processing::process_cycle(&c, "test");
    let decision = processing::deliberate(&c, &options);
    assert_eq!(decision.chosen, InstinctId::Store);
    assert!(decision.confidence > 0.5);
}

#[test]
fn habituation_reduces_intensity() {
    let mut c = InstinctCortex::new();
    c.seed(InstinctId::Load, 0.5);
    learning::habituate(&mut c, InstinctId::Load);
    let intensity = c.instincts.get(&InstinctId::Load).unwrap().intensity;
    assert!(intensity < 0.5);
}

#[test]
fn sensitization_increases_decay_rate() {
    let mut c = InstinctCortex::new();
    let rate = learning::sensitize(&mut c, InstinctId::Reflex);
    assert!(rate > 0.95);
}

#[test]
fn converge_toward_group() {
    let mut c = InstinctCortex::new();
    c.seed(InstinctId::Converge, 1.0);
    let result = social::converge(&mut c, InstinctId::Converge, &[0.0]);
    assert!(result < 1.0 && result > 0.0);
}

#[test]
fn extinct_check_prunes_low() {
    let c = InstinctCortex::new(); // all intensities at 0.0
    assert!(social::extinct_check(&c, InstinctId::Decay, None));
}

#[test]
fn homeostasis_states() {
    let mut c = InstinctCortex::new();
    // Balanced by default
    assert_eq!(homeostasis::homeostasis_check(&c), homeostasis::HomeostasisState::Balanced);

    // Crisis
    c.total_energy = 0.05;
    assert_eq!(homeostasis::homeostasis_check(&c), homeostasis::HomeostasisState::Crisis);

    // Surplus
    c.total_energy = 2.0;
    assert_eq!(homeostasis::homeostasis_check(&c), homeostasis::HomeostasisState::Surplus);
}
