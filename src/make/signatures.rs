use clap::Args;
use simplelog::{debug, info, warn};
use crate::global_args;

#[derive(Args, Debug)]
pub struct SignaturesArgs {
	/// Watch procedure index files for new procedures and update signatures as needed
	#[arg(long, short)]
	watch: bool,
}

pub fn handle(global_arguments: global_args::GlobalArgs, arguments: SignaturesArgs) {
	debug!("{:?}", global_arguments);
	debug!("{:?}", arguments);

	warn!("Not implemented yet 🙁");
	info!("Running from the signatures handler")
}