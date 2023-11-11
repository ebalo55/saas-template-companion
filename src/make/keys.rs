use simplelog::{trace, info, warn};
use crate::global_args;

pub fn handle(global_arguments: global_args::GlobalArgs) {
	trace!("{:?}", global_arguments);

	warn!("Not implemented yet 🙁");
	info!("Running from the keys handler")
}