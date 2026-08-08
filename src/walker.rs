// walker.rs
//
// The heart of the program. Everything here is about ANSWERING QUESTIONS
// about the filesystem (how big is this?) — NOT about printing or
// formatting. If you find yourself writing println! in this file, stop:
// that belongs in tree.rs or report.rs instead.
//
// This separation is what makes adding rayon later a small diff instead
// of a rewrite.

use std::path::{Path, PathBuf};
use std::{fs, io};
use log::{debug, warn};

// ---- Option A: the "keep the tree" node type -------------------------
// Only needed if you decide you want the full tree in memory (e.g. for
// tree.rs's pretty-printing, which benefits from having children handy).
// If you go fully "flat list" (see report.rs), you may not need this at
// all for Top mode — but tree.rs likely still wants something like this.

#[derive(Debug)]
pub struct Node {
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub size: u64,              // for dirs: recursive total of all descendants
    pub children: Vec<Node>,    // empty for files
}

// directory) is a classic bug here.

// following symlinks into a cycle (or double-counting a linked-elsewhere

// ---- Building the tree (for `tree` mode) ------------------------------
//
// fn build_tree(path: &Path) -> io::Result<Node>
//
// Structure (post-order recursion — compute children BEFORE this node's size):
//   1. std::fs::metadata(path) to check: file or dir?
//   2. If file -> return Node { size: metadata.len(), children: vec![], ... }
//   3. If dir  -> std::fs::read_dir(path), recurse on each entry into `children`
//   4. this node's size = children.iter().map(|c| c.size).sum()
//   5. return the Node
//
// Error handling hint: read_dir() and its iterator both yield Result.
// A single unreadable subdirectory (permissions, broken symlink, etc.)
// shouldn't kill the whole scan. Consider: skip it and continue, maybe
// print a warning to stderr, rather than using `?` and bailing out of
// the whole recursion.
//
// Symlink hint: fs::metadata() FOLLOWS symlinks by default;
// fs::symlink_metadata() does NOT. Decide deliberately which you want —

pub fn build_tree(path: &Path) -> anyhow::Result<Node> {
    // Don't follow symlinks
    let metadata = fs::symlink_metadata(path)?;
    let mut node = Node {
        path: path.to_path_buf(),
        is_dir: false,
        is_symlink: false,
        size: metadata.len(),
        children: vec![],
    };

    if metadata.is_symlink() {
        node.is_symlink = true;
        return Ok(node);
    }

    if metadata.is_file() {
        return Ok(node);
    }

    if metadata.is_dir() {
        node.is_dir = true;
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(e) => {
                            warn!("Skipping unreadable entry: {}", e);
                            continue;
                        }
                    };

                    let child_path = entry.path();
                    match build_tree(&child_path) {
                        Ok(child) => node.children.push(child),
                        Err(err) => warn!("Skipping {}: {}", child_path.display(), err),
                    }
                }
            },
            Err(e) => {
                // Error reading a directory, report it and continue.
                warn!("Error reading directory {}", path.display());
                debug!("Underlying error: {}", e);
            }
        }

        // Compute directory size
        node.size = node.children.iter().map(|c| c.size).sum();
    }

    Ok(node)
}


// ---- Flat-list variant (for `top N by size` mode) ---------------------
//
// Alternative to keeping the full tree: recurse purely for the size
// number, and as a side effect push (path, size) pairs for directories
// into a flat Vec that the caller owns. This avoids keeping parent/child
// links around once you're done — see our earlier discussion on
// "compute size" vs "remember everything".
//
// fn dir_size(path: &Path, results: &mut Vec<(PathBuf, u64)>) -> io::Result<u64>
//
// Structure:
//   1. If `path` is a file -> return its size immediately, don't touch `results`.
//   2. If `path` is a dir  -> read_dir(), recurse into each entry,
//      summing the u64 sizes returned.
//   3. Before returning, push (path.to_path_buf(), total_size) into `results`.
//   4. Return total_size to the caller (so THEIR sum includes us).
//
// Note results is a `&mut Vec` threaded through every recursive call —
// this is a good exercise in Rust borrowing across recursion. Once you
// bring in rayon, this signature will need to change (mutable shared
// state doesn't parallelize for free) — worth noticing that tension now.

pub fn dir_size(path: &Path, results: &mut Vec<(PathBuf, u64)>) -> io::Result<u64> {
    let metadata = fs::symlink_metadata(path)?;

    if metadata.is_file() {
        return Ok(metadata.len());
    }

    if metadata.is_dir() {

    }

    todo!("recursive size computation, flat results accumulation")
}


// ---- Parallel variant (v2, after sequential works) ---------------------
//
// Once build_tree works, try a par_build_tree using rayon:
//
//   use rayon::prelude::*;
//
//   fn par_build_tree(path: &Path) -> io::Result<Node> {
//       // same as build_tree, but the step that recurses over each
//       // directory entry uses .par_iter() / .into_par_iter() instead
//       // of a plain for-loop, then .collect() the resulting Vec<Node>.
//       //
//       // Question to think through: build_tree returns io::Result<Node>
//       // per call — collecting a Vec<io::Result<Node>> from parallel
//       // work needs `.collect::<io::Result<Vec<Node>>>()` to short-circuit
//       // on the first error, same idea as sequential ? propagation.
//   }
//
// This is deliberately NOT sketched out in more detail — get sequential
// build_tree fully working and tested first, then revisit this.