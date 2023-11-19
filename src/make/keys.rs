use std::io::{Seek, Write};
use std::path::PathBuf;

use alkali::{asymmetric::kx, symmetric::cipher};
use anyhow::Context;
use clap::Args;
use log::{debug, info, trace, warn};

use structures::{
	environment_record::EnvironmentRecord,
	environment_variables::EnvironmentVariables,
};

use crate::global_args;
use crate::helpers::base64_url;
use crate::make::keys::structures::environment_variables::ENVIRONMENT_VARIABLES_KEYS;
use crate::structures::file_mode::FileMode;
use crate::structures::stream_reader::StreamReader;

pub mod constants;
mod structures;
mod table;

#[derive(Args, Debug)]
pub struct KeysArgs {
	/// File to read the environment variables from, starting from the current working directory
	#[arg(long, short, default_value = ".env")]
	env: PathBuf,
}

/// Check if the line should be updated
fn should_update_line(line: &str, environment_variable: &EnvironmentRecord) -> bool {
	line.starts_with(environment_variable.name()) && !environment_variable.updated()
}

/// Update the line with the new value
fn update_line(content_store: &mut String, environment_variable: &mut EnvironmentRecord) {
	content_store.push_str(&*format!("{}=\"{}\"\n", environment_variable.name(), environment_variable.value()));
	environment_variable.set_as_updated();
}

/// Store or update the current .env file line
fn store_or_update_line(content_store: &mut String, line: &str, environment_variables: &mut EnvironmentVariables) {
	if should_update_line(&line, &environment_variables.next_auth_secret) {
		update_line(
			content_store,
			&mut environment_variables.next_auth_secret,
		)
	} else if should_update_line(&line, &environment_variables.asymmetric_encryption_public_key) {
		update_line(
			content_store,
			&mut environment_variables.asymmetric_encryption_public_key,
		)
	} else if should_update_line(&line, &environment_variables.asymmetric_encryption_private_key) {
		update_line(
			content_store,
			&mut environment_variables.asymmetric_encryption_private_key,
		)
	} else {
		content_store.push_str(&*line);
	}
}

/// Generate a new keypair
/// # Returns
/// A tuple with the public and private keys
/// ```text
/// (public_key, private_key)
/// ```
fn make_keypair() -> anyhow::Result<(String, String)> {
	let keypair = kx::Keypair::generate()
		.with_context(|| "Something went wrong while generating asymmetric keypair, does the system support secure cryptography or has enough entropy?")?;
	let b64_public = base64_url(keypair.public_key.as_slice())
		.with_context(|| "Something went wrong while encoding public key")?;
	let b64_secret = base64_url(keypair.private_key.as_slice())
		.with_context(|| "Something went wrong while encoding secret key")?;

	Ok((b64_public, b64_secret))
}

/// Generate a new secret key
fn make_secret_key() -> anyhow::Result<String> {
	let key = cipher::Key::generate()
		.with_context(|| "Something went wrong while generating symmetric key, does the system support secure cryptography or has enough entropy?")?;
	let b64_key = base64_url(key.as_slice())
		.with_context(|| "Something went wrong while encoding symmetric key")?;

	Ok(b64_key)
}

/// Print the environment variables as a table or JSON
fn print_datatable(is_json_context: bool, environment_variables: &EnvironmentVariables) {
	if !is_json_context {
		info!("Encryption keys created successfully");
		table::display_environment_variables_table(&environment_variables);
	} else {
		log_mdc::insert("variables", environment_variables.clone());
		info!("Encryption keys created successfully");
	}
}

/// Store the updated .env content into the file
fn store_updated_env_file(stream_reader: &mut StreamReader, updated_content: &str) -> anyhow::Result<()> {
	stream_reader.file()
	             .rewind()
	             .with_context(|| "Cannot move cursor back to file start, does the file exist and allows reading?")?;
	stream_reader.file()
	             .write_all(updated_content.as_bytes())
	             .with_context(|| "Cannot write to file, does the file exist and allows writing?")?;

	Ok(())
}

/// Ensure that the variable exists or add it to the content store
fn ensure_variables_exists_or_add(content_store: &mut String, environment_variable: &mut EnvironmentRecord) {
	if !content_store.contains(environment_variable.name()) {
		content_store.push_str(&*format!("{}=\"{}\"\n", environment_variable.name(), environment_variable.value()));
		environment_variable.set_as_updated();
	}
}

/// Update the .env file with the new values
fn update_env(environment_variables: &mut EnvironmentVariables, arguments: &KeysArgs) -> anyhow::Result<()> {
	info!("Updating .env file");

	let env = arguments.env.to_str().ok_or(anyhow::anyhow!("Cannot convert environment file path to string"))?;

	let mut stream_reader = StreamReader::new(
		env,
		FileMode::builder().write().read().create().build(),
	).with_context(|| format!("Something went wrong while opening stream reader to {}", env))?;

	let mut updated_content = String::new();

	// read line by line and immediately store the updated content
	loop {
		let line = stream_reader.read_line()
		                        .with_context(|| "Something went wrong while reading a new file line")?;
		debug!("Found line with length of {} bytes", line.length());

		if line.eof() {
			debug!("EOF reached at {} file", env);
			break;
		}

		store_or_update_line(&mut updated_content, line.line(), environment_variables);
	}

	for environment_variable_name in ENVIRONMENT_VARIABLES_KEYS {
		ensure_variables_exists_or_add(&mut updated_content, &mut environment_variables[environment_variable_name]);
	}

	store_updated_env_file(&mut stream_reader, &updated_content)
		.with_context(|| format!("Something went wrong while updating the {} file", env))?;

	info!(".env file update completed");

	Ok(())
}

pub fn handle(global_arguments: &global_args::GlobalArgs, arguments: &KeysArgs) -> anyhow::Result<()> {
	trace!("{:?}", global_arguments);

	info!("Generating asymmetric encryption keys");
	let (b64_public, b64_secret) = make_keypair().with_context(|| "Something went wrong during asymmetric keypair creation")?;

	info!("Generating symmetric encryption keys");
	let b64_key = make_secret_key().with_context(|| "Something went wrong during symmetric key creation")?;

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

	print_datatable(global_arguments.json, &environment_variables);

	if !global_arguments.dry_run {
		update_env(&mut environment_variables, arguments).with_context(|| "Something went wrong while updating the environment file")?;
	} else {
		warn!("Dry run, skipping file update");
	}

	Ok(())
}