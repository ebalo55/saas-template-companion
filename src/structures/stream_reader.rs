use std::fs::{File, OpenOptions};
use std::io::BufReader;

use anyhow::Context;

use crate::structures::file_mode_builder::FileModeBuilder;
use crate::structures::read_line::ReadLine;

pub struct StreamReader<'a> {
	filename: &'a str,
	file: File,
	pub reader: BufReader<File>,
}

impl<'a> StreamReader<'a> {
	/// Create a new StreamReader instance
	/// # Example
	/// ```rust
	/// let mut stream_reader = StreamReader::new(
	///     ".env",
	///     FileMode::builder().write().read().create().build(),
	/// ).with_context(|| format!("Failed to open stream reader to .env"))?;
	/// ```
	pub fn new(filename: &'a str, mode: u8) -> anyhow::Result<Self> {
		let cwd = std::env::current_dir()
			.with_context(|| "Failed to get the current working directory")?;

		let mode = FileModeBuilder::from(mode);

		let file = OpenOptions::new()
			.write(mode.can_write())
			.read(mode.can_read())
			.create(mode.can_create())
			.truncate(mode.can_truncate())
			.open(filename)
			.with_context(|| format!("Failed to open file '{}' (current working directory: {})", filename, cwd.display()))?;

		let reader = BufReader::new(file.try_clone().with_context(|| format!("Cannot clone file handler to {}", filename))?);

		Ok(Self {
			filename,
			reader,
			file,
		})
	}

	/// Read a line from the file
	pub fn read_line(&mut self) -> anyhow::Result<ReadLine> {
		ReadLine::from(&mut self.reader)
			.with_context(|| format!("Failed to read line from file '{}'", self.filename))
	}

	/// Get the raw file descriptor
	pub fn file(&self) -> &File {
		&self.file
	}
}

#[test]
fn can_read_lines_one_by_one() {
	use assert_fs::prelude::*;
	use crate::structures::file_mode::FileMode;

	// create a temporary file
	let file = assert_fs::NamedTempFile::new(".env").unwrap();
	file.write_str("LINE_1=1\nLINE_2=this-is-simple\nLINE_3=\"This is pretty complex\"").unwrap();

	let mut stream_reader = StreamReader::new(
		file.path().to_str().unwrap(),
		FileMode::builder()
			.read()
			.build(),
	).unwrap();

	assert_eq!(stream_reader.read_line().unwrap().line(), "LINE_1=1\n");
	assert_eq!(stream_reader.read_line().unwrap().line(), "LINE_2=this-is-simple\n");
	assert_eq!(stream_reader.read_line().unwrap().line(), "LINE_3=\"This is pretty complex\"");
}