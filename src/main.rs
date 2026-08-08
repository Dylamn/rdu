// main.rs
//
// Responsibilities of this file, and ONLY this file:
//   1. Parse CLI args (delegated to cli.rs)
//   2. Dispatch to the right mode (tree printing vs top-N report)
//   3. Handle top-level errors (print a nice message, exit non-zero)
//
// Nothing about walking directories, computing sizes, or formatting
// output should live here. main.rs should read almost like pseudocode:
// "parse args -> call the right function -> handle errors."

use clap::Parser;

use rdu::{Cli, Mode, report, tree};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Cli::parse();

    // 2. Match on the chosen mode:
    //    - Mode::Tree  -> call into tree.rs's entry function
    //    - Mode::Top   -> call into report.rs's entry function
    //
    //    Both of these will internally call walker.rs to do the actual
    //    filesystem recursion. main.rs shouldn't call walker.rs directly.
    let result = match args.mode {
        Mode::Tree => tree::run(&args.path, args.max_depth),
        Mode::Top => report::run(&args.path, args.top_n),
    };

    // 3. Handle the Result returned by whichever mode you called.
    //    Hint: if you propagate io::Result all the way up to main, you can
    //    return `Result<(), std::io::Error>` from main() itself instead of
    //    unwrapping — Rust lets main() return a Result, and it'll print
    //    the Debug output and exit(1) automatically on Err.
    //
    //    That said, a hand-written eprintln! + std::process::exit(1) often
    //    gives a friendlier error message for a CLI tool.
    match result {
        Ok(_) => {}
        Err(_) => {}
    };

    // todo!("parse args, dispatch to tree or report mode")
    Ok(())
}
