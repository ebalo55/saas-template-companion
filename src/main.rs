use clap::{Args, Parser, Subcommand};
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};

mod version;
mod authors;
mod cleanup;
mod global_args;
mod make;

/// SaaS Template Companion, helps you in the management and run of common operations to ease the
/// setup and creation of your SaaS
#[derive(Parser, Debug)]
#[command(name = "saas-template-companion", author, about, long_about = None, disable_help_subcommand = true, arg_required_else_help = true)]
struct CLI {
	/// Configuration file to load for a given command
	#[arg(short, long, global = true)]
	config: Option<std::path::PathBuf>,

	#[command(flatten)]
	global_args: global_args::GlobalArgs,

	#[command(subcommand)]
	command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
	/// Cleanup (remove) files or folders using glob patterns
	#[command()]
	Cleanup(cleanup::CleanupArgs),

	/// Make or generate something
	#[command(visible_alias = "generate")]
	Make(make::MakeArgs),

	/// Print version and exit
	#[command()]
	Version,

	/// Print authors and exit
	#[command()]
	Authors,
}

fn main() {
	let cli = CLI::parse();
	TermLogger::init(
		cli.global_args.verbose.log_level_filter(),
		Config::default(),
		TerminalMode::Mixed,
		ColorChoice::Auto,
	).unwrap();

	match cli.command {
		Command::Cleanup(options) => {
			cleanup::handle(cli.global_args, options)
		}
		Command::Make(options) => {
			make::handle(cli.global_args, options)
		}
		Command::Version => {
			version::handle()
		}
		Command::Authors => {
			authors::handle()
		}
	}
}
