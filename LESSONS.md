# thefuck-rs Conversion — Lessons Learned

## What We Built
- Rust port of nvbn/thefuck (90K star Python CLI)
- 172 rule files generated
- 32ms cold start vs 2000ms Python (50-100x faster)
- Tile-first routing system driving conversion

## What Worked
- Tile system architecture: Tiles → MegaBlox → GitHub → DeepSeek
- 44,896 tile corpus loaded in ~1.2s
- queryCodeTiles() — direct tile lookup by rule name, 90% confidence
- Single-process batch conversion (convert-batch.mjs) — tiles load once, all rules convert fast
- teach-back to learned.jsonl — every DeepSeek response trains the tile db
- DeepSeek direct API replacing agentic-flow (bypassed ANTHROPIC_API_KEY requirement)

## What Was Wrong

### GitHub step was missing
The correct flow is:
1. Extract intent from Python rule (what does match() detect, what does fix() return)
2. Search GitHub for existing Rust code doing the same thing
3. Adapt found code → write rule (0 tokens)
4. Only if nothing on GitHub → DeepSeek

We searched for "thefuck rust" which doesn't exist.
We should have searched by INTENT:
- git_push → search "git push upstream rust cli"
- apt_get → search "apt package manager rust"
- sudo → search "permission denied retry sudo rust"

### Stub tiles masked the problem
113 rules got stub tiles (// TODO: Convert from Python)
These hit at 86% confidence and wrote placeholder files
Build passed but rules aren't real implementations

### claude-flow swarm needs Claude Code
- swarm init works
- swarm start needs --objective not --strategy
- hive-mind spawn creates idle slots only
- Actual execution requires Claude Code agent tool
- On tile system (no Claude Code) → use direct scripts instead

### node arg length limit
Passing full Python source as CLI argument breaks node
Fix: route by rule NAME not by source content
"git_push rule convert to rust" → tile hit
"Convert this Python... [500 lines]..." → crashes

### Per-rule node process is slow
Each ./convert-all.sh call: 44K tiles reload = 1.2s per rule × 60 rules = 72s minimum
Fix: single node process imports router once, converts all rules in loop

## Correct Architecture (Next Time)
/
\
<<-
