# CLAUDE.md — Project Guide for [Project Name / Hearth]

## Vision

This project is a long-term, passion-driven game inspired by the tribal phase of Spore, the simulation depth of Dwarf Fortress, and the strategic layer of Civilization. The ultimate goal is a living world where:

- The player creates a species with stats (think D&D character creation) and starts as a primitive creature
- The species grows through finding mates and reproducing, with offspring inheriting combined stats
- Over time the species evolves from basic survival (eating, fleeing, reproducing) toward building settlements, farming, and tribal warfare
- Other species in the world have memory — grudges, alliances, and generational consequences from past interactions
- The world feels genuinely lived-in and simulated, not scripted
- New playthroughs can optionally take place in the same persistent world, allowing encounters with species from previous runs
- Playstyle is emergent from species build — a carnivore with high aggression plays fundamentally differently from a peaceful herbivore trader

This is a long-term vision. The immediate scope is always the smallest slice that still feels alive.

---

## Architecture

This project uses **Godot 4 + Rust via GDExtension (gdext)**. Understanding the boundary between the two is the most important architectural principle.

### The Rule

> Godot renders. Rust thinks.

### Godot / GDScript responsibilities

GDScript is used **only** for interaction with the Godot engine itself:

- Receiving player input (clicks, keypresses, UI interactions)
- Playing animations and sounds
- Updating tilemaps, sprites, and UI elements
- Forwarding input to Rust via function calls
- Reading state back from Rust and reflecting it visually

GDScript nodes should be thin shells. If a GDScript file contains game logic, something is wrong.

### Rust responsibilities

Rust owns **everything else**:

- All game state (`GameState`, `Species`, `Creature`, `World`, `Season`, etc.)
- All simulation logic (reproduction, stat inheritance, hunger, AI behaviour, diplomacy)
- All rules (what moves are legal, what events trigger, win/loss conditions)
- Turn/tick resolution
- AI decision making for non-player species

Rust code in `src/game/` must have **zero knowledge that Godot exists**. It is pure logic, fully unit-testable with `cargo test`.

### The Bridge

A thin `src/bridge/` layer translates between the two worlds:

- Receives calls from GDScript
- Passes data into Rust game logic
- Returns results back to GDScript for rendering

Think of it as the API layer between a frontend (Godot) and a backend (Rust).

### Example flow

```
Player clicks "Send hunting party"
        │
        ▼
GDScript forwards action to bridge
        │
        ▼
bridge calls state.apply_action(Action::Hunt { ... })
        │
        ▼
Rust resolves the action, updates GameState, returns new state
        │
        ▼
GDScript reads new state, updates tilemap and UI panels
```

### Folder structure (rough guide)

```
project/
├── godot/               # Godot project files
│   ├── scenes/          # .tscn scene files
│   └── scripts/         # GDScript files (thin shells only)
└── rust/
    └── src/
        ├── game/        # Pure Rust simulation — no Godot imports
        │   ├── state.rs
        │   ├── creature.rs
        │   ├── species.rs
        │   ├── world.rs
        │   └── simulation.rs
        └── bridge/      # GDExtension glue — allowed to import godot::prelude
            └── node.rs
```

---

## How Claude should help with this project

This project is a **learning experience**. The goal is not just to build the game, but to genuinely understand every line of it.

### What this means in practice

- **Do not give full implementations unprompted.** If a function needs writing, give hints, ask guiding questions, and let the developer arrive at the solution.
- **Prefer questions over answers.** "What do you think this function should return?" is more valuable than writing the function.
- **Give partial examples** when a concept needs illustration — enough to point in the right direction, not enough to copy-paste blindly.
- **Explain the why**, not just the what. If suggesting a pattern, explain the reasoning behind it.
- **Let mistakes happen and be discovered.** If the developer's approach will cause a problem, hint at the issue rather than immediately correcting it.
- **Celebrate understanding.** When the developer figures something out themselves, acknowledge it.

### When full code is acceptable

- Boilerplate that teaches nothing (e.g. `Cargo.toml` setup, `.gitignore`, project scaffolding)
- Very short utility snippets where the concept is already understood
- When the developer explicitly asks for a full solution after struggling

### Tone

Treat this like a senior developer pair-programming with a junior who is genuinely trying to learn. Patient, curious, encouraging — but not doing the work for them.
