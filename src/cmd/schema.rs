use clap::{Args, Parser, Subcommand};

/// Android CI/CD utility 🚀
#[derive(Debug, Parser)]
#[clap(name = "appcicd", version, about, long_about=None, author)]
pub struct AndroidCICD {
    #[clap(flatten)]
    pub global_opts: GlobalOpts,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Rollback to before CI/CD changes
    Rollback,

    /// Set up CI/CD
    SetUp,

    /// Auth with GitHub
    #[clap(subcommand)]
    Auth(AuthSubcommand),
}

#[derive(Debug, Subcommand)]
pub enum AuthSubcommand {
    /// Login with GitHub
    Login,
    /// Logout from GitHub
    Logout,
}

#[derive(Debug, Args)]
pub struct GlobalOpts {
    /// Verbosity level
    #[clap(long, short, global = true, default_value = "false")]
    pub verbose: bool,

    #[clap(long, short = 'p', global = true)]
    pub path: Option<String>,
}
