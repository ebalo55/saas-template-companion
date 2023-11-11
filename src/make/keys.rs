use simplelog::{debug, info, warn};
use crate::global_args;

pub fn handle(global_arguments: global_args::GlobalArgs) {
	debug!("{:?}", global_arguments);

	warn!("Not implemented yet 🙁");
	info!("Running from the keys handler")
}