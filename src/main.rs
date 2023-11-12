use clap::{Parser, Subcommand};
use log4rs::append::console::ConsoleAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::Encode;
use log4rs::encode::json::JsonEncoder;
use log4rs::encode::pattern::PatternEncoder;
use saas_template_companion::{authors, cleanup, global_args, make, version};

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

/// Set up the cli wide logger, messages can be logged using the `log` crate.
///
/// Log formats samples:
///  - json: [reference](https://docs.rs/log4rs/1.2.0/log4rs/encode/json/index.html#contents)
///  - text: `[2023-11-12 05:29:04.294446 +01:00] [TRACE] <message>`
fn setup_logger(cli: &CLI) -> anyhow::Result<()> {
	let encoder: Box<dyn Encode> = if cli.global_args.json {
		Box::new(JsonEncoder::new())
	} else {
		Box::new(PatternEncoder::new("[{d(%Y-%m-%d %H:%M:%S%.6f %Z)}] [{h({l})}] {m}{n}"))
	};

	let stdout: ConsoleAppender = ConsoleAppender::builder()
		.encoder(encoder)
		.build();

	let log_config = Config::builder()
		.appender(
			Appender::builder()
				.build("stdout", Box::new(stdout))
		)
		.build(
			Root::builder()
				.appender("stdout")
				.build(cli.global_args.verbose.log_level_filter())
		)?;
	log4rs::init_config(log_config)?;

	Ok(())
}

fn main() -> anyhow::Result<()> {
	let cli = CLI::parse();

	setup_logger(&cli)?;

	match cli.command {
		Command::Cleanup(options) => {
			cleanup::handle(&cli.global_args, &options)
		}
		Command::Make(options) => {
			make::handle(&cli.global_args, &options)
		}
		Command::Version => {
			version::handle()
		}
		Command::Authors => {
			authors::handle()
		}
	}
}
