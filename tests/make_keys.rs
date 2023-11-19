// Used for writing assertions
use std::process::Command;

use assert_cmd::prelude::*;
// Run programs
use assert_fs::prelude::*;
// Add methods on commands
use predicates::prelude::*;

#[test]
fn can_make_keys_in_dry_run_mode() -> Result<(), Box<dyn std::error::Error>> {
	let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

	cmd.args(["make", "keys", "--dry-run"]);
	cmd.assert()
	   .success()
	   .stdout(predicate::str::contains("[INFO] Generating asymmetric encryption keys"))
	   .stdout(predicate::str::contains("[INFO] Generating symmetric encryption keys"))
	   .stdout(predicate::str::contains("[INFO] Encryption keys created successfully"))
	   .stdout(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__NEXTAUTH_SECRET))
	   .stdout(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PUBLIC_KEY))
	   .stdout(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PRIVATE_KEY))
	   .stdout(predicate::str::contains("[WARN] Dry run, skipping file update"));

	Ok(())
}

#[test]
fn can_make_keys_and_update_env_file() -> Result<(), Box<dyn std::error::Error>> {
	let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

	let file = assert_fs::NamedTempFile::new(".env").unwrap();
	file.write_str(&*format!(
		"{}={}\n{}={}\n{}={}\nUNMUTATED_VARIABLE=UNMUTATED_VALUE\n",
		saas_template_companion::make::keys::constants::ENV_VARIABLE__NEXTAUTH_SECRET,
		"ENV_VARIABLE__NEXTAUTH_SECRET__SAMPLE_VALUE",  // this value will be replaced
		saas_template_companion::make::keys::constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PUBLIC_KEY,
		"ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PUBLIC_KEY__SAMPLE_VALUE",  // this value will be replaced
		saas_template_companion::make::keys::constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PRIVATE_KEY,
		"ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PRIVATE_KEY__SAMPLE_VALUE",  // this value will be replaced
	)).unwrap();

	cmd.args(["make", "keys", "--env", file.path().to_str().unwrap()]);
	cmd.assert()
	   .success()
	   .stdout(predicate::str::contains("[INFO] Generating asymmetric encryption keys"))
	   .stdout(predicate::str::contains("[INFO] Generating symmetric encryption keys"))
	   .stdout(predicate::str::contains("[INFO] Encryption keys created successfully"))
	   .stdout(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__NEXTAUTH_SECRET))
	   .stdout(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PUBLIC_KEY))
	   .stdout(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PRIVATE_KEY))
		.stdout(predicate::str::contains("[INFO] Updating .env file"))
		.stdout(predicate::str::contains("[INFO] .env file update completed"));

	// check if the file was updated
	file.assert(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__NEXTAUTH_SECRET))
	    .assert(predicate::str::contains("ENV_VARIABLE__NEXTAUTH_SECRET__SAMPLE_VALUE").not())
	    .assert(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PUBLIC_KEY))
	    .assert(predicate::str::contains("ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PUBLIC_KEY__SAMPLE_VALUE").not())
	    .assert(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PRIVATE_KEY))
	    .assert(predicate::str::contains("ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PRIVATE_KEY__SAMPLE_VALUE").not())
	    .assert(predicate::str::contains("UNMUTATED_VARIABLE=UNMUTATED_VALUE"));

	Ok(())
}

#[test]
fn can_make_keys_and_create_env_file() -> Result<(), Box<dyn std::error::Error>> {
	let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

	let file = assert_fs::NamedTempFile::new(".env").unwrap();

	cmd.args(["make", "keys", "--env", file.path().to_str().unwrap()]);
	cmd.assert()
	   .success()
	   .stdout(predicate::str::contains("[INFO] Generating asymmetric encryption keys"))
	   .stdout(predicate::str::contains("[INFO] Generating symmetric encryption keys"))
	   .stdout(predicate::str::contains("[INFO] Encryption keys created successfully"))
	   .stdout(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__NEXTAUTH_SECRET))
	   .stdout(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PUBLIC_KEY))
	   .stdout(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PRIVATE_KEY))
	   .stdout(predicate::str::contains("[INFO] Updating .env file"))
	   .stdout(predicate::str::contains("[INFO] .env file update completed"));

	// check if the file was updated
	file.assert(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__NEXTAUTH_SECRET))
	    .assert(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PUBLIC_KEY))
	    .assert(predicate::str::contains(saas_template_companion::make::keys::constants::ENV_VARIABLE__ASYMMETRIC_ENCRYPTION_PRIVATE_KEY));

	Ok(())
}
