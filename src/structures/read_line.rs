use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::Context;

pub struct ReadLine {
	line: String,
	length: usize,
	eof: bool,
}

impl ReadLine {
	/// Read a line from the file
	pub fn from(reader: &mut BufReader<File>) -> anyhow::Result<Self> {
		let mut line = String::new();

		let length = reader.read_line(&mut line)
		                   .with_context(|| "Failed to read line")?;

		// readline returns 0 when the end of the file has been reached
		let eof = length.eq(&0);

		Ok(Self {
			line,
			length,
			eof,
		})
	}

	/// Get the read line
	pub fn line(&self) -> &str {
		&self.line
	}

	/// Get the read line length
	pub fn length(&self) -> usize {
		self.length
	}

	/// Check if the end of the file has been reached
	pub fn eof(&self) -> bool {
		self.eof
	}
}