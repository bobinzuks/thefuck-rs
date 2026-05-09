# thefuck-rs

A Rust port of [thefuck](https://github.com/nvbn/thefuck) — corrects your last wrong command.

## Benchmarks

| Tool | Cold Start | Language |
|------|-----------|----------|
| thefuck-rs | **42ms** | Rust |
| thefuck | ~2000-4000ms | Python |

**50-100x faster.**

Tested on NixOS, hyperfine 50 runs, warmup 5.

## Usage

```bash
# Add to .bashrc / .zshrc
eval $(fuck --alias)

# Then just type:
fuck
```

## Rules implemented

- git_branch — fixes typos in git subcommands
- git_push — adds --set-upstream when missing

## How it works

1. Shell hook captures last failed command
2. Re-runs it, captures stderr
3. Rules match against output pattern
4. First match executes the fix

## Roadmap

- [ ] sudo rule
- [ ] cd_mkdir rule  
- [ ] GNN router (priority scoring from history)
- [ ] WASM rule modules
