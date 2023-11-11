use clap::Args;
use simplelog::{debug, warn};

use crate::global_args;

#[derive(Args, Debug)]
pub struct CleanupArgs {
	/// List of glob patters to remove
	#[arg(long, short)]
	blacklist: Option<Vec<String>>,
}

pub fn handle(global_arguments: global_args::GlobalArgs, arguments: CleanupArgs) {
	debug!("{:?}", global_arguments);
	debug!("{:?}", arguments);

	warn!("Not implemented yet 🙁");
}