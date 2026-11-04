# code-artefacts

Every artefact produced by a BGent run, plus the module entry point that makes this
repository usable as `bgent-core`'s `src/games/` directory.

**This repository is mounted as a git submodule at `bgent-core/src/games`.** It is not
a data dump sitting next to the crate; it is part of the crate's module tree. That is
why the contract check, the module check and `cargo build` all cover game code: the
game is compiled by the same build as the harness that judges it.

## Layout

```
mod.rs                  declares `pub mod current;` — the only module compiled
current.rs              the active game; written by scripts/integrate.py
runs/<run-id>/
  game.rs               generated implementation
  RULES.md              distilled rules, served verbatim to players later
  meta.json             original prompt, invariants, models, params, seed
  qa.md                 the clarification log
.github/workflows/      run-simulation.yml
```

`runs/` is never declared as a module, so nothing in it is ever compiled. One broken
historical run cannot break the crate, and compile time does not grow with history.

## Who writes what

| Path | Writer |
|---|---|
| `runs/<run-id>/*` | the Rules Agent, and only it |
| `current.rs` | `scripts/integrate.py`, in `bgent-core` |
| `mod.rs` | nobody — stable, committed, reviewed |

## Branches

Runs go on `run/<run-id>` so `main` stays a clean, compiling baseline. `main` always
builds; a branch only has to build if it is being checked.

## Checking a run

Dispatch `.github/workflows/run-simulation.yml` with `run_id`. It clones
`laspbgent/bgent-core`, installs `runs/<run-id>/game.rs` as `current.rs`, and runs the
full build + harness. The report comes back as an artifact.
