#[derive(clap::Args, Debug)]
pub struct GlobalArgs {
    /// Preview the modification the command should do without applying them
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Set the message verbosity levels
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,
}