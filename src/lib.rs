//! # cuda-instinct-cortex
//!
//! The instinct processing system for FLUX VM. Maps 16 instinct opcodes (0xB0–0xBF)
//! to a higher-level cortex that processes instinct states and produces behavioral decisions.
//!
//! "The Deeper Connection."

use std::collections::HashMap;

pub mod homeostasis;
pub mod learning;
pub mod processing;
pub mod social;

/// Instinct opcodes 0xB0–0xBF.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstinctId {
    Load = 0xB0,
    Store = 0xB1,
    Decay = 0xB2,
    Reflex = 0xB3,
    Modulate = 0xB4,
    Threshold = 0xB5,
    Converge = 0xB6,
    Extinct = 0xB7,
    Habituate = 0xB8,
    Sensitize = 0xB9,
    Generalize = 0xBA,
    Specialize = 0xBB,
    Diffuse = 0xBC,
    Inhibit = 0xBD,
    Sum = 0xBE,
    Diff = 0xBF,
}

impl InstinctId {
    pub fn from_opcode(opcode: u8) -> Option<Self> {
        match opcode {
            0xB0 => Some(Self::Load),
            0xB1 => Some(Self::Store),
            0xB2 => Some(Self::Decay),
            0xB3 => Some(Self::Reflex),
            0xB4 => Some(Self::Modulate),
            0xB5 => Some(Self::Threshold),
            0xB6 => Some(Self::Converge),
            0xB7 => Some(Self::Extinct),
            0xB8 => Some(Self::Habituate),
            0xB9 => Some(Self::Sensitize),
            0xBA => Some(Self::Generalize),
            0xBB => Some(Self::Specialize),
            0xBC => Some(Self::Diffuse),
            0xBD => Some(Self::Inhibit),
            0xBE => Some(Self::Sum),
            0xBF => Some(Self::Diff),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Load => "Load",
            Self::Store => "Store",
            Self::Decay => "Decay",
            Self::Reflex => "Reflex",
            Self::Modulate => "Modulate",
            Self::Threshold => "Threshold",
            Self::Converge => "Converge",
            Self::Extinct => "Extinct",
            Self::Habituate => "Habituate",
            Self::Sensitize => "Sensitize",
            Self::Generalize => "Generalize",
            Self::Specialize => "Specialize",
            Self::Diffuse => "Diffuse",
            Self::Inhibit => "Inhibit",
            Self::Sum => "Sum",
            Self::Diff => "Diff",
        }
    }

    pub fn all() -> [InstinctId; 16] {
        [
            Self::Load, Self::Store, Self::Decay, Self::Reflex,
            Self::Modulate, Self::Threshold, Self::Converge, Self::Extinct,
            Self::Habituate, Self::Sensitize, Self::Generalize, Self::Specialize,
            Self::Diffuse, Self::Inhibit, Self::Sum, Self::Diff,
        ]
    }
}

/// A single instinct with activation state.
#[derive(Debug, Clone)]
pub struct Instinct {
    pub id: u8,
    pub name: String,
    pub intensity: f64,        // 0.0 – 1.0
    pub decay_rate: f64,       // default 0.95
    pub last_activated: u64,   // tick/cycle number
}

impl Instinct {
    pub fn new(id: InstinctId) -> Self {
        Self {
            id: id as u8,
            name: id.name().to_string(),
            intensity: 0.0,
            decay_rate: 0.95,
            last_activated: 0,
        }
    }

    pub fn with_intensity(id: InstinctId, intensity: f64) -> Self {
        Self {
            id: id as u8,
            name: id.name().to_string(),
            intensity: intensity.clamp(0.0, 1.0),
            decay_rate: 0.95,
            last_activated: 0,
        }
    }

    /// Apply one tick of exponential decay.
    pub fn decay(&mut self) {
        self.intensity *= self.decay_rate;
        self.intensity = self.intensity.clamp(0.0, 1.0);
    }

    /// Activate with a stimulus magnitude (0.0–1.0). Sets intensity to stimulus, clamped.
    pub fn activate(&mut self, stimulus: f64, tick: u64) {
        self.intensity = stimulus.clamp(0.0, 1.0);
        self.last_activated = tick;
    }
}

/// The instinct cortex: holds all instincts, tracks energy, targets homeostasis.
#[derive(Debug, Clone)]
pub struct InstinctCortex {
    pub instincts: HashMap<InstinctId, Instinct>,
    pub total_energy: f64,
    pub homeostasis_target: f64,
}

impl Default for InstinctCortex {
    fn default() -> Self {
        Self::new()
    }
}

impl InstinctCortex {
    pub fn new() -> Self {
        let mut instincts = HashMap::new();
        for id in InstinctId::all() {
            instincts.insert(id, Instinct::new(id));
        }
        Self {
            instincts,
            total_energy: 1.0,
            homeostasis_target: 0.5,
        }
    }

    /// Seed a specific instinct with an initial intensity.
    pub fn seed(&mut self, id: InstinctId, intensity: f64) {
        if let Some(instinct) = self.instincts.get_mut(&id) {
            instinct.intensity = intensity.clamp(0.0, 1.0);
        }
    }

    /// Apply decay to all instincts.
    pub fn decay_all(&mut self) {
        for instinct in self.instincts.values_mut() {
            instinct.decay();
        }
    }

    /// Sum of all instinct intensities.
    pub fn total_intensity(&self) -> f64 {
        self.instincts.values().map(|i| i.intensity).sum()
    }
}
