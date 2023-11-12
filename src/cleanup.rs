use clap::Args;
use log::{trace, warn};

use crate::global_args;

#[derive(Args, Debug)]
pub struct CleanupArgs {
	/// List of glob patters to remove
	#[arg(long, short)]
	blacklist: Option<Vec<String>>,
}

pub fn handle(global_arguments: &global_args::GlobalArgs, arguments: &CleanupArgs) -> anyhow::Result<()> {
	trace!("{:?}", global_arguments);
	trace!("{:?}", arguments);

	warn!("Not implemented yet 🙁");

	Ok(())
}