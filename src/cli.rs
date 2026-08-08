use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(name = "rdu", about = "A directory tree / disk usage analyzer")]
 pub struct Cli {
    /// Path to scan (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Which mode to run in — see Mode enum below
    #[arg(default_value = "tree")]
    pub mode: Mode,

    /// How many results to show in Top mode
    #[arg(short = 'n', long, default_value_t = 10)]
    pub top_n: usize,

    /// Limit recursion depth when printing (None = unlimited)
    #[arg(long)]
    pub max_depth: Option<usize>,

    // Use rayon to parallelize the walk (once implemented)
    // #[arg(long)]
    // pub parallel: bool,
}

#[derive(clap::ValueEnum, Clone)]
pub enum Mode {
    Tree,
    Top,
}
