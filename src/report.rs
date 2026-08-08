// The "which directories eat the most space" mode.

use std::path::{Path, PathBuf};
use std::io;

// Entry point called from main.rs for `Mode::Top`.
//
// fn run(path: &Path, top_n: usize) -> io::Result<()>
//
// v1 (simple, recommended to build first):
//   1. let mut results: Vec<(PathBuf, u64)> = Vec::new();
//   2. walker::dir_size(path, &mut results)?;   // fills results as a side effect
//   3. results.sort_by(|a, b| b.1.cmp(&a.1));    // descending by size
//   4. print the first `top_n` entries with humanize::format(size)
//
// v2 (later optimization, optional):
//   Replace step 1-3 with a bounded structure (e.g. a Vec you manually
//   keep at length <= top_n, or a BinaryHeap<Reverse<(u64, PathBuf)>>)
//   that you update as dir_size discovers each directory, instead of
//   collecting everything and sorting at the end. Same output, lower
//   peak memory for huge trees. Don't reach for this until v1 works.

pub fn run(_path: &Path, _top_n: usize) -> io::Result<()> {
    todo!("walk, collect (path, size) pairs, sort, print top N")
}

// Small helper you'll likely want regardless of v1/v2:
//
// fn print_report_row(path: &Path, size: u64)
//   - pad/align the size column so output looks tabular
//   - use humanize::format(size) rather than raw byte counts