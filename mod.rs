//! BGent game module.
//!
//! This repository *is* `bgent-core`'s `src/games/` module. It is mounted there as a
//! git submodule, so everything in this directory is part of the crate's module tree
//! and the crate cannot build without it.
//!
//! Only `current` is compiled. `runs/` holds every artefact ever produced and is never
//! declared here, so a broken historical run cannot break the build. That is the whole
//! reason the history lives in a separate repository.
//!
//! ## What a run must provide
//!
//! `current.rs` must define:
//!
//! ```rust,ignore
//! pub struct State;   // Clone + Eq + Hash + Debug + Pure
//! pub struct Move;    // idem
//! pub struct Game;    // idem
//! pub fn new_game() -> Game;
//! impl CombinatorialGame<State, Move> for Game { /* all nine methods */ }
//! ```
//!
//! Type aliases and enums work too — the checker only needs to be able to *name*
//! `State`, `Move` and `Game`, because the contract uses generic parameters rather
//! than associated types.
//!
//! `scripts/integrate.py` in `bgent-core` copies a run into `current.rs` and compiles.
//! Nothing here is ever edited by hand.

pub mod current;
