+++
title = "LLM-assisted Implementation Approaches for a Board Game"
date = 2025-08-20
path = "posts/llm-assisted-implementation-approaches-for-a-board-game"
+++

I've been working on `Omdurman`, a Bevy implementation of *Remember Gordon! — The Battle of Omdurman* (1982). It's a turn-based strategy game originally made on paper, where two very different factions battle it out over the Sudanese desert cities and the Nile.
The game is pretty strictly defined by its manual and the maps, unit counters, and charts that came with it.

The implementation is a WebRTC-based multiplayer Bevy+Egui application which runs natively or in the browser.
The game relies on a deterministic rules engine (randomness for dice via initial seed).
Consequentially, the game state is a datastructure where each game action is applied as an effect.

This enabled some interesting implementation approaches:

* A game replay is a recorded sequence of `GameEffect`s, each applied to the game state in order
* A spectator can scrub through the game like a piece of music
* A new-joiner gets the full history (although this is not needed for a deterministic game, it is valuable for cheat detection)
* A game replay can be inspected later for rule deviations _which the engine permitted_

## Machine-checked Traceability

One of the key design decisions was to map rulebook sections to implementation and test sites bijectively, enforced at four independent levels:

- **Compile time:** a dedicated test crate references every _cited_ symbol as `use path as _;` (renaming breaks build).
- **Test time:** bijectivity + citation coverage checks; `#[rulebook("§6.3")]` macro makes tests self-annotate their covered sections into a JSONL the checker reads.
- **Live editor:** an LSP server gives diagnostics/hover/definitions over `.rs`, `.toml`, and the OCR'd manual — same `checks` as the test, so editor and CI always agree.
- **Generated artifact:** a typst pipeline regenerates [traceability.pdf](/traceability.pdf); `fix_lines` re-syncs stale line numbers.

This means if a rulebook section is referenced in the code, it must exist in the manual, and vice versa. It's a nice way to keep everything in sync.

## LLM Confined to Render/Observe

The LLM is a *derived presentation layer*, never a participant. This is an important distinction — we're not using AI to make game decisions or validate rules.

- The engine accumulates structured `TurnSummary`s, which are emitted as `GameEvent::TurnComplete` — recorded and replayed like any other event.
- Telegrams and newspapers for player delectation are generated from that deterministic summary, not just from LLM reasoning; the newspaper template is selected from the typed game result.
- The log drives the GUI, spectator replay, bot, telegram, and newspaper.

In other words, the LLM takes deterministic game data and makes it readable for humans.

## Three-agent Rule Verification

This is where things get interesting. Two independent per-faction agents play head-to-head; a third audits. The aim of the playing agents is not just to win, but to explore the whole game.

- **Layers:** (1) engine `can_*` predicates reject illegal moves, (2) hard invariants + proptest assert legal states after every effect, (3) an LLM observer reads the whole log against the rulebook for misapplications the invariants can't express (wrong Combat-Results-Table row, missed modifier, phase-order slip).
- **Advisory LLM, deterministic arbiter.** Automated gating never depends on the LLM; findings are surfaced for humans. Probabilistic tools add breadth, never authority.

The point of this is _not_ to train agents; it is to audit the game codebase and to find rule loopholes. It's a way to catch edge cases that might slip through automated testing.

### Agent Memory

The agents need to remember what they've learned across turns, so each LLM side owns a 500 KB scratchpad cache. This serves as inter-turn memory.

Every LLM reply is a single JSON object — typed structs (`PlanResponse`, `ReviewResponse`) with `#[serde(default)]` degrade on malformed output, and `response_format: json_object` at the transport constrains the provider. One shared fence-stripper tolerates stray code fences; no ad-hoc line parser.

A game is too big for one prompt, so the observer reviews turn-sized chunks, carrying running notes between them and deduping findings across chunks. Effects are rendered as prose designed for LLM consumption — dice spelled out, CRT rows, MP arithmetic, engine-authoritative § citations — so the auditor can re-derive rule outcomes without engine access.

## Grounded Reference Corpus

The OCR'd manual is the canonical asset; printed tables are transcribed as one module each. A curated crib sheet and per-side strategy doctrine files are checked-in; every § citation in them must resolve against the traceability matrix.
The LLM's reference corpus is itself versioned and machine-verified.

Observers are told to cite only crib-sheet sections and never invent numbers. This prevents hallucinations from corrupting the audit.

## Spec as Executable Vignettes

We created 24 "tactics scripts" — hand-built states plus ordered `Legal` / `Illegal` / `Assert` steps with pre-rolled dice. These replay deterministically as both a regression suite and the spec the move generator is validated against.

They're essentially executable specifications that double as tests.

## Testing Techniques

The rules engine uses a layered testing strategy to catch both expected and unexpected violations:

- **Property-based testing with `proptest`:** After every `GameEffect` is applied, property tests assert invariants like "all units are on valid hexes", "no faction has negative strength", and "movement points are within budget". These tests generate random sequences of legal moves and verify the engine never reaches an illegal state.
- **Deterministic vignettes:** The 24 tactics scripts from the previous section double as regression tests — they replay identically on every peer, so any divergence is a bug.
- **Compile-time traceability checks:** A dedicated test crate references every cited symbol; renaming or removing a symbol breaks the build, ensuring the rulebook-to-code mapping stays bijective.
- **Snapshot testing:** Combat results, terrain modifiers, and CRT outputs are snapshot-tested against known-good values from the rulebook, catching regressions when logic is refactored.
- **Mockable transport:** The `Completion` trait behind the LLM transport allows the entire observer pipeline to run on canned responses, making agent-audit tests fully deterministic and CI-friendly.

Beyond the property tests, the rules engine is model-checked with [Kani](https://model-checking.github.io/kani/): 63 proof harnesses cover the hex geometry, die arithmetic, chart lookups, and the atomicity and monotonicity of `apply_effect`. Proofs close the domain where tests only sample it — they caught `distance` using the wrong cube axis, contradicting `neighbors` and silently corrupting line-of-sight for 65% of on-board firing pairs, while every test kept passing because they happened to sample a pure-axis case that works under both conventions. The proofs are first-class citizens of the traceability matrix (`proofs = [...]` citations, annotated `// §N`), and the proof suite runs in CI.

## Testability & Offline Operation

The LLM transport is behind a mockable `Completion` trait; the entire observer pipeline runs on canned responses. Env-driven config (`LLM_API_KEY` / `LLM_BASE_URL` / `LLM_MODEL`) shared across app and bot; no-key runs skip cleanly. Every LLM path has a deterministic fallback — the whole stack runs with zero API access.

---

*The lesson: put agents on a deterministic, replayable substrate; confine them to render/observe roles; let them challenge the code at breadth while deterministic tools hold the gate.*

Nonetheless, progress on this game _per token spent_ has plummeted.
