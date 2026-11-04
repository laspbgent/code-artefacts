# code-artefacts

Generated game code, one branch and one folder per run.

```
runs/<run-id>/
  game.rs     the generated implementation
  RULES.md    the distilled rules, served verbatim to Player Agents
  meta.json   original prompt, invariants, models, run params, seed
  qa.md       the clarification Q/A log
```

Branches are named `run/<run-id>`. Nothing here is ever compiled unless a run is
explicitly checked out, so a broken historical run cannot break anything.

`.github/workflows/run-simulation.yml` is dispatched with the run id; it clones
`laspbgent/bgent-core`, integrates this run's `game.rs`, builds, and runs the
player-free harness. The report comes back as a workflow artifact.
