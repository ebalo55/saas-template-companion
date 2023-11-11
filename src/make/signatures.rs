use clap::Args;
use simplelog::{trace, info, warn};
use crate::global_args;

#[derive(Args, Debug)]
pub struct SignaturesArgs {
	/// Watch procedure index files for new procedures and update signatures as needed
	#[arg(long, short)]
	watch: bool,
}

pub fn handle(global_arguments: global_args::GlobalArgs, arguments: SignaturesArgs) {
	trace!("{:?}", global_arguments);
	trace!("{:?}", arguments);

	warn!("Not implemented yet 🙁");
	info!("Running from the signatures handler")
}