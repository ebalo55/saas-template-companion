use alkali::{SODIUM_LIBRARY_VERSION_MAJOR, SODIUM_LIBRARY_VERSION_MINOR};

pub fn handle() -> anyhow::Result<()> {
	println!(
		"{} v{} - with\n\t- Sodium v{}.{}",
		clap::crate_name!(),
		clap::crate_version!(),
		SODIUM_LIBRARY_VERSION_MAJOR,
		SODIUM_LIBRARY_VERSION_MINOR
	);

	Ok(())
}
