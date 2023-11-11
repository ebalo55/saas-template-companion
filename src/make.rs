use clap::{Args, Subcommand};
use simplelog::trace;

use crate::global_args;

mod keys;
mod signatures;

#[derive(Subcommand, Debug)]
enum MakeSubCommand {
	/// Generate and store new encryption keys
	#[command()]
	Keys,

	/// Remap the pre-generated procedure signatures
	#[command()]
	Signatures(signatures::SignaturesArgs),
}

#[derive(Args, Debug)]
pub struct MakeArgs {
	#[command(subcommand)]
	command: MakeSubCommand,
}

pub fn handle(global_arguments: global_args::GlobalArgs, arguments: MakeArgs) {
	trace!("{:?}", global_arguments);
	trace!("{:?}", arguments);

	match arguments.command {
		MakeSubCommand::Keys => {
			keys::handle(global_arguments)
		}
		MakeSubCommand::Signatures(options) => {
			signatures::handle(global_arguments, options)
		}
	}
}