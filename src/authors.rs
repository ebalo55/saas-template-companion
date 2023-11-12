pub fn handle() -> anyhow::Result<()> {
	println!("Authors: {}", clap::crate_authors!());

	Ok(())
}
