use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Seek, Write};
use std::ops::Index;

use alkali::{asymmetric::kx, symmetric::cipher};
use anyhow::Context;
use comfy_table::{Attribute, Cell, Row, Table};
use comfy_table::presets::UTF8_FULL;
use log::{debug, info, trace};
use serde::Serialize;

use crate::global_args;
use crate::helpers::base64_url;
use crate::make::keys::environment_record::EnvironmentRecord;
use crate::make::keys::environment_variables::{ENVIRONMENT_VARIABLES_KEYS, EnvironmentVariables};

mod constants;
mod environment_record;
mod environment_variables;

/// Pack the environment variables into a vector of rows to be used by the table
fn pack_table_rows(env_variables: &EnvironmentVariables) -> Vec<Row> {
	ENVIRONMENT_VARIABLES_KEYS.iter()
	                          .map(|key| &env_variables[key] as &EnvironmentRecord) // get the value of the key (the struct key hardcoded in the array)
	                          .map(|ev| Row::from(vec![ev.name(), ev.value()]))
	                          .collect()
}

pub fn handle(global_arguments: &global_args::GlobalArgs) -> anyhow::Result<()> {
	trace!("{:?}", global_arguments);

	info!("Generating asymmetric encryption keys");
	let keypair = kx::Keypair::generate()?;
	let b64_public = base64_url(keypair.public_key.as_slice())?;
	let b64_secret = base64_url(keypair.private_key.as_slice())?;

	debug!("Asymmetric public key = {b64_public}");
	debug!("Asymmetric secret key = {b64_secret}");

	info!("Generating symmetric encryption keys");
	let b64_key = base64_url(cipher::Key::generate()?.as_slice())?;
	debug!("Symmetric key = {b64_key}");

	let mut environment_variables = EnvironmentVariables {
		next_auth_secret: EnvironmentRecord::new(
			constants::ENV_VARIABLE__NEXTAUTH_SECRET,
			b64_key.as_str(),
		),
		asymmetric_encryption_public_key: EnvironmentRecord::new(
			constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PUBLIC_KEY,
			b64_public.as_str(),
		),
		asymmetric_encryption_private_key: EnvironmentRecord::new(
			constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PRIVATE_KEY,
			b64_secret.as_str(),
		),
	};

	// print the table with the values of the keys
	if !global_arguments.json {
		info!("Encryption keys created successfully");

		let mut table = Table::new();
		table.load_preset(UTF8_FULL)
		     .set_header(vec![
			     Cell::new("Environment variable name").add_attribute(Attribute::Bold),
			     Cell::new("Value").add_attribute(Attribute::Bold),
		     ])
		     .add_rows(pack_table_rows(&environment_variables));

		println!("{table}");
	} else {
		log_mdc::insert("variables", environment_variables);
		info!("Encryption keys created successfully");
	}

	if !global_arguments.dry_run {
		let cwd = std::env::current_dir()
			.with_context(|| "Failed to get the current working directory")?;

		let mut env_file = OpenOptions::new()
			.write(true)
			.read(true)
			.create(true)
			.open(".env")
			.with_context(|| format!("Failed to open or create '{}/.env'", cwd.display()))?;

		let mut reader = BufReader::new(&env_file);
		let mut line = String::new();
		let mut updated_content = String::new();

		// read line by line and immediately replace it
		loop {
			line.clear();
			let line_length = reader.read_line(&mut line)
			                        .with_context(|| "Failed while trying to read the file")?;
			debug!("Found line with length of {} bytes", line_length);

			// eof reached
			if line_length.eq(&0) {
				debug!("Empty line detected, EOF reached");
				break;
			}

			if line.starts_with(constants::ENV_VARIABLE__NEXTAUTH_SECRET) {
				updated_content.push_str(&*format!("{}=\"{}\"\n", constants::ENV_VARIABLE__NEXTAUTH_SECRET, b64_key));
			} else if line.starts_with(constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PUBLIC_KEY) {
				updated_content.push_str(&*format!("{}=\"{}\"\n", constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PUBLIC_KEY, b64_public));
			} else if line.starts_with(constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PRIVATE_KEY) {
				updated_content.push_str(&*format!("{}=\"{}\"\n", constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PRIVATE_KEY, b64_secret));
			} else {
				updated_content.push_str(&*line);
			}
		}
		env_file.rewind()
		        .context("Cannot move cursor back to file start, does the file still exist?")?;
		env_file.write_all(updated_content.as_bytes())?;
	}

	Ok(())
}