use clap::Parser;

/// Android CI/CD utility 🚀
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path of Android project
    #[arg(short, long)]
    pub path: Option<String>,

    /// Rollback changes
    #[arg(short, long)]
    pub rollback: bool,
}
