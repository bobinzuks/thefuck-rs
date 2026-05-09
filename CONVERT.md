# thefuck-rs Full Conversion via Ruvflow

## Mission
Convert all Python thefuck rules to Rust using the tile-first router system.

## System
- Tile router: hooks/tile-first-router.mjs
- Tiles: 44,896 entries + 53 rust-conversion tiles
- Routing: Tiles → MegaBlox → GitHub → DeepSeek
- Teach-back: Every DeepSeek response writes to learned.jsonl

## Agent Pipeline

AGENT 1 — CONVERTER
For each .py file in source-python/thefuck/rules/:
  1. Query: node tile-first-router.mjs "{rule_name} rule convert to rust"
  2. Extract answer from JSON response
  3. Write to src/rules/{rule_name}.rs
  4. Log: source (tile-code/github/deepseek) + tokens used
  SendMessage findings to AGENT 2

AGENT 2 — VALIDATOR  
For each .rs file written:
  1. Check it compiles: cargo check 2>&1
  2. Fix obvious errors (missing imports, wrong types)
  3. Add to src/rules/mod.rs registry
  SendMessage results to AGENT 3

AGENT 3 — MOD BUILDER
Build complete src/rules/mod.rs:
  1. mod declarations for all rules
  2. get_rules() returning all rules
  3. cargo build --release
  SendMessage to AGENT 4

AGENT 4 — BENCHMARKER
  1. Run hyperfine vs python thefuck
  2. Write results to README.md
  3. git commit + push
  SendMessage to AGENT 5

AGENT 5 — TILE TRAINER
For every DeepSeek response used:
  1. Write tile back to learned.jsonl
  2. Verify tile queryable
  3. Report final tile count

## Constraints
- ALWAYS use tile router first — never call DeepSeek directly
- GNN vectors on every file
- Checkpoint every 10 rules
- Non-stop execution
