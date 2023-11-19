use clap::{Args, Subcommand};
use log::trace;

use crate::global_args;

pub mod keys;
pub mod signatures;

#[derive(Subcommand, Debug)]
enum MakeSubCommand {
	/// Generate and store new encryption keys
	#[command()]
	Keys(keys::KeysArgs),

	/// Remap the pre-generated procedure signatures
	#[command()]
	Signatures(signatures::SignaturesArgs),
}

#[derive(Args, Debug)]
pub struct MakeArgs {
	#[command(subcommand)]
	command: MakeSubCommand,
}

pub fn handle(global_arguments: &global_args::GlobalArgs, arguments: &MakeArgs) -> anyhow::Result<()> {
	trace!("{:?}", global_arguments);
	trace!("{:?}", arguments);

	match &arguments.command {
		MakeSubCommand::Keys(options) => {
			keys::handle(global_arguments, options)
		}
		MakeSubCommand::Signatures(options) => {
			signatures::handle(global_arguments, options)
		}
	}
}