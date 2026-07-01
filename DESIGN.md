# Hearth — Game Design Document

## Vision

Hearth is a real-time species simulation where the player creates a creature with a set of stats and guides a growing population from primitive survival toward tribal dominance. The world is driven by emergent behaviour — creatures act autonomously, shaped by their stats and environment, while the player provides high-level guidance rather than direct control.

The closest references:
- **Spore (tribal phase)** — species identity, growth from small to large scale
- **The Sims** — creatures have autonomy and personality, player nudges rather than commands
- **Dwarf Fortress / RimWorld** — pauseable real-time, the world keeps simulating, player sets priorities
- **Evolution/genetics** — stats are inherited and drift over generations, cross-species breeding possible

---

## Core Pillars

1. **Your species is alive, not a unit.** Creatures act on their own. The player guides, not commands.
2. **Evolution is the win condition.** Success means a thriving, adapted population — not a score.
3. **Always feel fragile.** A string of bad decisions or bad luck should be able to end a run. No safety nets.
4. **Every playthrough tells a different story.** Stats, environment, and rival species create emergent narratives.
5. **Depth through simulation, not scripting.** No hardcoded story beats. Outcomes emerge from systems.

---

## Player Experience Arc

### Early game — Survival
- Player creates a species with a stat spread (strength, speed, aggression, intelligence, etc.)
- Starts with 1–3 creatures in a world with other species
- Goals: find food, avoid predators, find a mate, reproduce
- Player is heavily involved — guiding individual creatures to eat, flee, find mates

### Mid game — Tribe
- Population grows to dozens
- Creatures become more autonomous, player sets group priorities
- Settlements emerge: gathering spots, shelters, farms
- First contact with rival species — trade, conflict, or coexistence
- Stats begin diverging across generations based on environment and breeding

### Late game — Civilisation
- Hundreds of creatures, largely self-managing
- Player focuses on high-level decisions: diplomacy, war, expansion
- Species may have visibly evolved from the starting form
- Rival species have their own histories shaped by past interactions

---

## Core Systems

### Species & Stats
- Player designs a species at game start with a point-buy stat system
- Stats: **Strength**, **Speed**, **Aggression**, **Intelligence**, **Resilience**, **Sociability** (names TBD)
- Stats influence autonomous behaviour — a high-aggression creature will pick fights unprompted; a high-intelligence one will find food more efficiently
- Stats are inherited by offspring with small random variation (mutation)
- Cross-species breeding is possible and produces hybrid stat spreads

### Creature Autonomy
- Creatures act on their own drives: hunger, fear, reproduction, curiosity
- Player cannot override personality — a cowardly creature won't charge an army
- Player guidance: point creatures toward goals (hunt here, build here, talk to that creature)
- Autonomy increases as population grows — early game is more hands-on, late game is more strategic

### Reproduction & Evolution
- Creatures seek mates autonomously when drives are met (fed, safe, social need)
- Offspring inherit combined stats from both parents with small mutation
- Over generations, the population naturally adapts to its environment and playstyle
- No explicit "evolution screen" — change happens gradually and emergently
- **Environment drives evolution:** building near water reinforces swimming traits, constant combat selects for aggression, food scarcity selects for efficiency. The player shapes evolution through decisions, not stat screens
- Which creatures survive and breed is the evolution mechanic — protect your strongest, let your weakest die, and the gene pool shifts accordingly
- Players can nudge breeding by guiding specific creatures together, but cannot force it

### World & Environment
- Grid-based world (tile size TBD — small enough to feel smooth)
- Biomes affect food availability, movement cost, and which species spawn
- Resources: food sources, building materials, water
- World persists between sessions (no offline simulation — world pauses when player is away)

### Other Species
- World populated with other species, each with their own stats and behaviours
- Species have memory: grudges, alliances, and reputations persist across generations
- Interactions: ignore, trade, befriend, raid, war, breed
- Rival species grow and evolve independently — a species ignored for 20 generations may become a serious threat
- Competition for food and territory creates natural pressure even without direct conflict

### Threats & Fragility
- **Disease:** spreads through the population, high-sociability species are more vulnerable (tradeoff with social bonding benefits)
- **Environmental disasters:** drought, harsh winters, predator migrations — the world pushes back
- **Resource depletion:** overhunting or overfarming an area has lasting consequences next season
- **Population bottlenecks:** a bad event can permanently narrow the gene pool, locking out traits forever
- Bad decisions compound — there is no undo, and the world does not wait

### Settlements & Building
- Creatures build structures when guided to do so (or autonomously at high intelligence)
- Structures: shelters, food storage, farms, defensive walls (scope TBD)
- Buildings occupy tiles — placement is meaningful

### Combat
- **Phase 1 (current scope):** Auto-resolved battles based on stat comparison
- **Phase 2 (future, optional):** Tactical combat layer — either Total War style (real-time) or Fire Emblem style (turn-based tactical). Decided later based on what feels right.

---

## Technical Decisions

### Grid-based movement
- All logical positions are tile coordinates (`TilePos { x: i32, y: i32 }`)
- Rust owns tile positions as the source of truth
- Godot visually lerps sprites between tile positions for smooth appearance
- Tile size: likely 16–32px (TBD during implementation)
- Benefits: clean pathfinding (A*), no physics overlap bugs, trivial serialisation, Rust owns all logic

### Rust / Godot split
- **Rust:** all simulation — creature state, AI, stats, combat resolution, world state, pathfinding
- **Godot:** rendering, input, animation, UI, visual lerping between tile positions
- **Bridge:** thin translation layer, converts tile coords to pixel coords, passes player intent to Rust

### Pathfinding
- A* on the grid, implemented in Rust
- Creatures find paths to targets through the tile world
- Walkability determined by tile type (later: structures, water, etc.)

---

## Scope — What's In vs Out

### In scope (core game)
- Species creation with stats
- Autonomous creature behaviour driven by stats
- Reproduction and stat inheritance
- Grid-based world with multiple biomes
- Other species with memory
- Settlements and basic building
- Auto-resolved combat

### Nice to have (post-core)
- Tactical combat layer
- Persistent world across playthroughs
- Cross-species diplomatic relations screen
- Speed-up button for simulation
- Disease system
- Environmental disasters

### Out of scope
- Multiplayer
- Procedural music
- Full 3D

---

## Open Questions

- Tile size — 16px, 32px, 64px? Affects how "zoomed in" the world feels.
- Does the player ever directly control a single creature (Spore-style), or always guide at group level?
- What does the species creation screen look like — point buy, sliders, descriptive choices?
- How does food/hunger work mechanically — resource tiles, hunting zones, farming?
- What triggers a creature to seek a mate — time, hunger threshold, population size?
