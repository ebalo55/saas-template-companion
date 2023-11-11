use clap::Args;
use simplelog::{trace, warn};

use crate::global_args;

#[derive(Args, Debug)]
pub struct CleanupArgs {
	/// List of glob patters to remove
	#[arg(long, short)]
	blacklist: Option<Vec<String>>,
}

pub fn handle(global_arguments: global_args::GlobalArgs, arguments: CleanupArgs) {
	trace!("{:?}", global_arguments);
	trace!("{:?}", arguments);

	warn!("Not implemented yet 🙁");
}