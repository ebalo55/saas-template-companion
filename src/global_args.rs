#[derive(clap::Args, Debug)]
pub struct GlobalArgs {
    /// Run a command without applying any modification
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Log messages using json objects
    #[arg(long, global = true)]
    pub json: bool,

    /// Set the message verbosity levels
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,
}