# rdu

A small directory tree / disk usage analyzer, written in Rust as a learning
project for recursion, `PathBuf`, sorting, and (eventually) easy data
parallelism with `rayon`.

Think `tree` + `du`, minus the years of C history.

## Features

- **Tree mode** — print a directory's structure as a tree, similar to the
  `tree` command.
- **Top mode** — recursively compute directory sizes and report the N
  biggest space users, similar to `du | sort -rh | head`.
- Skips symlinks rather than following them (no infinite loops, no
  double-counted disk usage).
- Keeps scanning even if some subdirectories are unreadable (permission
  errors are logged and skipped, not fatal).

## Status

🚧 Work in progress — built step by step as a Rust learning exercise.
Sequential recursion first, `rayon`-based parallel walking planned as a
follow-up once the sequential version is solid.

## Installation

Requires a recent Rust toolchain ([rustup.rs](https://rustup.rs)).

```bash
git clone git@github.com:Dylamn/rdu.git
cd rdu
cargo build --release
```

> The compiled binary will be at `target/release/rdu`.

## Usage

```bash
# Print a tree of the current directory
rdu

# Print a tree of a specific path
rdu /path/to/folder

# Show the 10 biggest directories under a path
rdu /path/to/folder top

# Show the 20 biggest directories
rdu /path/to/folder top -n 20

# Limit tree output to 2 levels deep
rdu /path/to/folder tree --max-depth 2
```

> Exact flags depend on the final `cli.rs` — run `rdu --help` for the
> up-to-date list.

### Logging

`rdu` uses [`env_logger`](https://docs.rs/env_logger) for warnings about
unreadable files/directories. By default, these are suppressed; enable them
with:

```bash
RUST_LOG=warn rdu /path/to/folder
```

Use `RUST_LOG=debug` for more detail on skipped entries.

## Project structure

```
rdu/
├── Cargo.toml
└── src/
    ├── main.rs        — CLI entry point, dispatches to tree/report modes
    ├── cli.rs         — argument definitions (clap)
    ├── walker.rs      — recursive filesystem walking, size computation
    ├── tree.rs        — tree-style rendering
    ├── report.rs      — "top N by size" rendering
    └── humanize.rs    — byte counts → human-readable sizes (KB/MB/GB)
```

The core design principle: **walking/computing is separate from
printing**. `walker.rs` never prints anything and returns plain data;
`tree.rs` and `report.rs` only render data they're handed. This keeps the
recursive logic easy to reason about and makes it straightforward to
parallelize the walk later without touching any output code.

## Roadmap

- [x] Sequential recursive walker with post-order size aggregation
- [x] Symlink handling (skip, don't follow)
- [x] Graceful handling of unreadable directories
- [x] Tree-mode pretty printing with box-drawing characters
- [x] Top-N report mode
- [x] `--max-depth` support
- [ ] Parallel walking with `rayon` (`par_iter` over directory entries)
- [x] Human-readable size formatting with tests

## Why "rdu"?

`r` (Rust) + `du`. Minimal, mirrors how people already type `du`.

## License

MIT (or whatever you prefer — this is a personal learning project).