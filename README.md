# cuda-skill

Skill system — proficiency levels, prerequisite trees, power-law practice, synergy bonuses, skill sharing (Rust)

Part of the Cocapn cognitive layer — how agents think, decide, and learn.

## What It Does

### Key Types

- `Skill` — core data structure
- `SkillTree` — core data structure
- `SkillSummary` — core data structure
- `SkillShare` — core data structure

## Quick Start

```bash
# Clone
git clone https://github.com/Lucineer/cuda-skill.git
cd cuda-skill

# Build
cargo build

# Run tests
cargo test
```

## Usage

```rust
use cuda_skill::*;

// See src/lib.rs for full API
// 13 unit tests included
```

### Available Implementations

- `Proficiency` — see source for methods
- `Skill` — see source for methods
- `SkillTree` — see source for methods
- `SkillShare` — see source for methods

## Testing

```bash
cargo test
```

13 unit tests covering core functionality.

## Architecture

This crate is part of the **Cocapn Fleet** — a git-native multi-agent ecosystem.

- **Category**: cognition
- **Language**: Rust
- **Dependencies**: See `Cargo.toml`
- **Status**: Active development

## Related Crates

- [cuda-confidence-cascade](https://github.com/Lucineer/cuda-confidence-cascade)
- [cuda-deliberation](https://github.com/Lucineer/cuda-deliberation)
- [cuda-reflex](https://github.com/Lucineer/cuda-reflex)
- [cuda-goal](https://github.com/Lucineer/cuda-goal)
- [cuda-fusion](https://github.com/Lucineer/cuda-fusion)
- [cuda-attention](https://github.com/Lucineer/cuda-attention)
- [cuda-emotion](https://github.com/Lucineer/cuda-emotion)
- [cuda-narrative](https://github.com/Lucineer/cuda-narrative)
- [cuda-learning](https://github.com/Lucineer/cuda-learning)

## Fleet Position

```
Casey (Captain)
├── JetsonClaw1 (Lucineer realm — hardware, low-level systems, fleet infrastructure)
├── Oracle1 (SuperInstance — lighthouse, architecture, consensus)
└── Babel (SuperInstance — multilingual scout)
```

## Contributing

This is a fleet vessel component. Fork it, improve it, push a bottle to `message-in-a-bottle/for-jetsonclaw1/`.

## License

MIT

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
