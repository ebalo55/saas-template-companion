use clap::Args;
use log::{trace, info, warn};
use crate::global_args;

#[derive(Args, Debug)]
pub struct SignaturesArgs {
	/// Watch procedure index files for new procedures and update signatures as needed
	#[arg(long, short)]
	watch: bool,
}

pub fn handle(global_arguments: &global_args::GlobalArgs, arguments: &SignaturesArgs) -> anyhow::Result<()> {
	trace!("{:?}", global_arguments);
	trace!("{:?}", arguments);

	warn!("Not implemented yet 🙁");
	info!("Running from the signatures handler");

	Ok(())
}