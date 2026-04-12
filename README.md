# cuda-instinct-cortex

> The instinct processing system for FLUX VM — 16 opcodes (0xB0–0xBF) mapped to a higher-level cortex that processes instinct states and produces behavioral decisions.

**"The Deeper Connection."**

## Overview

FLUX VM vessels don't just compute — they *feel*. The instinct cortex sits above the instruction set, translating raw neural signals into behavioral decisions. Each instinct has intensity, decay, and energy cost. The cortex ranks competing drives and produces actions.

## Instinct Opcodes

| Opcode | Name | Hex | Purpose |
|--------|------|-----|---------|
| 0xB0 | Load | `B0` | Acquire resources |
| 0xB1 | Store | `B1` | Retain accumulated value |
| 0xB2 | Decay | `B2` | Release / wind down |
| 0xB3 | Reflex | `B3` | Immediate reactive response |
| 0xB4 | Modulate | `B4` | Adjust internal state |
| 0xB5 | Threshold | `B5` | Evaluate boundary conditions |
| 0xB6 | Converge | `B6` | Align with group values |
| 0xB7 | Extinct | `B7` | Prune exhausted instincts |
| 0xB8 | Habituate | `B8` | Reduce sensitivity (×0.95) |
| 0xB9 | Sensitize | `B9` | Increase sensitivity (×1.05) |
| 0xBA | Generalize | `BA` | Transfer learning between instincts |
| 0xBB | Specialize | `BB` | Focus instinct to context |
| 0xBC | Diffuse | `BC` | Spread influence to neighbors |
| 0xBD | Inhibit | `BD` | Suppress competing drives |
| 0xBE | Sum | `BE` | Aggregate signals |
| 0xBF | Diff | `BF` | Detect divergence |

## Modules

- **`lib`** — Core types: `Instinct`, `InstinctId`, `InstinctCortex`
- **`processing`** — Activation, cycle processing, reflex, deliberation
- **`learning`** — Habituate (0x95× decay), Sensitize (1.05× boost), Generalize, Specialize
- **`social`** — Converge (25% group move), Extinct check, Diffuse, herd analysis
- **`homeostasis`** — Energy balance, proportional allocation, survival mode

## Quick Start

```rust
use cuda_instinct_cortex::*;

let mut cortex = InstinctCortex::new();
cortex.seed(InstinctId::Reflex, 0.9);

// Process a cycle
let urgencies = processing::process_cycle(&cortex, "threat-detected");

// Check for reflex (bypasses deliberation)
if let Some(reflex) = processing::reflex_check(&cortex, 0.8) {
    // React immediately
}

// Or deliberate
let decision = processing::deliberate(&cortex, &urgencies);
```

## Energy Homeostasis

The cortex tracks total energy and targets a homeostasis setpoint. States:

| State | Condition |
|-------|-----------|
| **Balanced** | Energy within 0.7×–1.5× of target |
| **Deficit** | Energy below 0.7× target |
| **Surplus** | Energy above 1.5× target |
| **Crisis** | Energy below 0.1 |

In crisis, `survival_mode()` suppresses all non-essential instincts (only Load, Reflex, Threshold remain active).

## Cross-Pollination

- **[cuda-instruction-set](https://github.com/nicobailon/cuda-instruction-set)** — The raw opcodes this cortex interprets
- **[cuda-energy](https://github.com/nicobailon/cuda-energy)** — Energy supply that feeds instinct decisions
- **[cuda-ethics](https://github.com/nicobailon/cuda-ethics)** — Ethical constraints on instinct-driven behavior

## License

MIT
