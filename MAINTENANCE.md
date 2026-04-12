# cuda-instinct-cortex — Maintenance Notes

## Purpose
Higher-level instinct processing layer. The 16 instinct opcodes (0xB0-0xBF) are primitives; this crate is the cortex that interprets them.

## Architecture
- Processing pipeline: stimulus -> activate -> reflex? -> deliberate -> action
- Learning: habituation (0.95x), sensitization (1.05x), generalization (25%), specialization (1.2x)
- Social: converge (25% herd), extinct (<10/255), diffuse (10% spread)
- Homeostasis: energy allocation proportional to urgency, survival mode suppresses non-essential

## Key Constants (TUNABLE)
- Reflex threshold: 0.80 — above this, act immediately without deliberation
- Convergence rate: 0.25 — 25% toward group mean (not 50% — preserves individuality)
- Extinction threshold: 10/255 — very low, allows even weak instincts to persist
- Habituation decay: 0.95 per activation — ~14 exposures to halve
- Diffusion rate: 0.10 — slow spread, prevents cascade instability

## Why These Specific Values
Each constant represents a tradeoff:
- Reflex 0.80: High enough to skip deliberation in emergencies, low enough to allow thought in normal conditions
- Converge 0.25: If too high, fleet becomes a hive mind. If too low, no coordination benefit.
- Extinct 10/255: At ~4% of max intensity. Any lower and noise would constantly trigger extinction.
- These values should be empirically tuned by observing fleet behavior.

## Related Crates
- cuda-instruction-set: defines the 16 instinct opcodes
- cuda-energy: energy budgets for instinct processing
- cuda-ethics: ethical constraints on instinct-driven actions
- cuda-confidence-math: confidence affects deliberation weight
