// Everything here is about RENDERING a Node tree that walker.rs already
// built. No filesystem calls should happen in this file.

use crate::humanize::SizeUnit;
use crate::walker::Node;
use crate::{humanize, walker};
use std::path::Path;

// Entry point called from main.rs for `Mode::Tree`.
//   1. call walker::build_tree(path) to get the root Node
//   2. optionally sort each level's children (see print_tree below)
//   3. call print_tree(&root, "", true, max_depth, 0)
pub fn run(path: &Path, max_depth: Option<usize>, size_unit: SizeUnit) -> anyhow::Result<()> {
    let root = walker::build_tree(path)?;

    print_tree(&root, "", true, max_depth, 0, size_unit);

    Ok(())
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

fn print_tree(
    node: &Node,
    prefix: &str,
    is_last: bool,
    max_depth: Option<usize>,
    depth: usize,
    size_unit: SizeUnit,
) {
    let name = node.path.components().last().unwrap().as_os_str();

    if depth == 0 {
        println!("{}", name.display());
    } else {
        let connector = if is_last { "└── " } else { "├── " };
        println!(
            "{}{}{} ({})",
            prefix,
            connector,
            name.display(),
            humanize::format(node.size, size_unit)
        );
    }

    if let Some(max_depth) = max_depth {
        if depth >= max_depth {
            if !node.children.is_empty() {
                let extension = if is_last { "    " } else { "│   " };
                println!(
                    "{}{}└── [{} more items]",
                    prefix,
                    extension,
                    node.children.len()
                );
            }
            return;
        }
    }

    // The root contributes nothing to its children's prefix — there's no
    // "bar continuing down from root" concept, since root has no siblings.
    // Every other node DOES contribute an extension, based on its own is_last.
    let child_prefix = if depth == 0 {
        prefix.to_string()
    } else {
        let extension = if is_last { "    " } else { "│   " };
        format!("{}{}", prefix, extension)
    };

    let total_children = node.children.len();
    for (i, child) in node.children.iter().enumerate() {
        let is_last_child = i + 1 == total_children;
        print_tree(
            child,
            &child_prefix,
            is_last_child,
            max_depth,
            depth + 1,
            size_unit,
        );
    }
}
