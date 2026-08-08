// Everything here is about RENDERING a Node tree that walker.rs already
// built. No filesystem calls should happen in this file.

use std::path::Path;
use std::io;
use crate::walker;
use crate::walker::Node;

// Entry point called from main.rs for `Mode::Tree`.
//   1. call walker::build_tree(path) to get the root Node
//   2. optionally sort each level's children (see print_tree below)
//   3. call print_tree(&root, "", true, max_depth, 0)
pub fn run(path: &Path, max_depth: Option<usize>) -> anyhow::Result<()> {
    let root = walker::build_tree(path)?;

    println!("{:?}", root);

    print_tree(&root, "", true, max_depth, 0);

    Ok(())
}

fn print_tree(
    node: &Node,
    prefix: &str,
    is_last: bool,
    max_depth: Option<usize>,
    depth: usize,
) -> () {
    println!("{:?}", node);
}
//
// Classic box-drawing recursive printer. Hints:
//
// - Connector characters: "├── " for a non-last child, "└── " for the
//   last child in a directory's list. Use is_last to pick.
//
// - `prefix` accumulates as you descend: when recursing into children,
//   pass along `prefix + "│   "` (if this node was NOT last) or
//   `prefix + "    "` (if it WAS last) — this is what makes the vertical
//   bars line up correctly at every depth.
//
// - max_depth: if Some(d) and depth >= d, stop recursing (but still
//   print this node). Good place to also print something like
//   "[N more items]" if you truncate.
//
// - Sorting: decide here (not in walker.rs) whether children print
//   alphabetically, or biggest-first. Sort a LOCAL copy of the children
//   slice/Vec right before iterating, don't mutate the tree in place
//   unless you're sure you don't need the original order elsewhere.
//
// - Don't forget to print the human-readable size next to each entry —
//   see humanize.rs.
