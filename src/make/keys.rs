use std::io::{Seek, Write};

use alkali::{asymmetric::kx, symmetric::cipher};
use anyhow::Context;
use log::{debug, info, trace};

use structures::{
	environment_record::EnvironmentRecord,
	environment_variables::EnvironmentVariables,
};

use crate::global_args;
use crate::helpers::base64_url;
use crate::structures::file_mode::FileMode;
use crate::structures::stream_reader::StreamReader;

mod constants;
mod structures;
mod table;

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

pub fn handle(global_arguments: &global_args::GlobalArgs) -> anyhow::Result<()> {
	trace!("{:?}", global_arguments);

	info!("Generating asymmetric encryption keys");
	let keypair = kx::Keypair::generate().with_context(|| "Failed while generating keypair")?;
	let b64_public = base64_url(keypair.public_key.as_slice())
		.with_context(|| "Failed while encoding public key")?;
	let b64_secret = base64_url(keypair.private_key.as_slice())
		.with_context(|| "Failed while encoding secret key")?;

	info!("Generating symmetric encryption keys");
	let b64_key = base64_url(
		cipher::Key::generate()
			.with_context(|| "Failed while generating symmetric key")?
			.as_slice()
	).with_context(|| "Failed while encoding symmetric key")?;

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
		table::display_environment_variables_table(&environment_variables);
	} else {
		log_mdc::insert("variables", environment_variables.clone());
		info!("Encryption keys created successfully");
	}

	if !global_arguments.dry_run {
		let mut stream_reader = StreamReader::new(
			".env",
			FileMode::builder().write().read().create().build(),
		).with_context(|| format!("Failed to open stream reader to .env"))?;

		let mut updated_content = String::new();

		// read line by line and immediately store the updated content
		loop {
			let line = stream_reader.read_line()
			                        .with_context(|| "Failed to read line")?;
			debug!("Found line with length of {} bytes", line.length());

			if line.eof() {
				debug!("EOF reached at .env file");
				break;
			}

			store_or_update_line(&mut updated_content, line.line(), &mut environment_variables);
		}

		stream_reader.file()
		             .rewind()
		             .with_context(|| "Cannot move cursor back to file start, does the file still exist?")?;
		stream_reader.file()
		             .write_all(updated_content.as_bytes())
		             .with_context(|| "Cannot write to file, does the file allows writing?")?;
	}

	Ok(())
}