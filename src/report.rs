// The "which directories eat the most space" mode.

use crate::humanize;
use crate::walker;
use std::path::{Path, PathBuf};

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
//   Replace step 1-3 with a bounded structure (e.g., a Vec you manually
//   keep at length <= top_n, or a BinaryHeap<Reverse<(u64, PathBuf)>>)
//   that you update as dir_size discovers each directory, instead of
//   collecting everything and sorting at the end. Same output, lower
//   peak memory for huge trees. Don't reach for this until v1 works.

pub fn run(
    path: &Path,
    top_n: usize,
    max_depth: Option<usize>,
    size_unit: humanize::SizeUnit,
) -> anyhow::Result<()> {
    let max_depth = max_depth.unwrap_or(usize::MAX);
    let mut results: Vec<(PathBuf, u64)> = vec![];

    walker::dir_size(path, &mut results)?;

    results.retain(|(entry_path, _)| {
        entry_path.strip_prefix(path)
            .map(|stripped| stripped.components().count() <= max_depth)
            .unwrap_or(false) // Shouldn't happen normally
    });

    results.sort_by(|a, b| b.1.cmp(&a.1));
    results.truncate(top_n); // no-op if `top_n >= results.len()`, doesn't panic

    // Find the longest path of the top
    let max_path_len = results
        .iter()
        .map(|(path, _)| path.display().to_string().len())
        .max()
        .unwrap_or(0);

    let sizes: Vec<String> = results
        .iter()
        .map(|(_, size)| humanize::format(*size, size_unit))
        .collect();
    let max_size_len = sizes.iter().map(|s| s.len()).max().unwrap_or(0);

    for ((path, _), size_str) in results.iter().zip(sizes.iter()) {
        println!(
            "{:<path_width$}\t{:>size_width$}",
            path.display(),
            size_str,
            path_width = max_path_len,
            size_width = max_size_len
        );
    }

    Ok(())
}

// Small helper you'll likely want regardless of v1/v2:
//
// fn print_report_row(path: &Path, size: u64)
//   - pad/align the size column so output looks tabular
//   - use humanize::format(size) rather than raw byte counts
