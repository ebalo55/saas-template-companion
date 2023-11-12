use crate::structures::file_mode::FileMode;

pub struct FileModeBuilder {
	mode: u8,
}

impl FileModeBuilder {
	/// Create a new builder with the default mode
	pub fn default() -> Self {
		Self {
			mode: FileMode::None as u8,
		}
	}

	/// Create a new builder with the given mode
	pub fn from(mode: u8) -> Self {
		Self {
			mode,
		}
	}

	/// File can be read
	pub fn read(mut self) -> Self {
		self.mode |= FileMode::Read;
		self
	}

	/// File can be written
	pub fn write(mut self) -> Self {
		self.mode |= FileMode::Write;
		self
	}

	/// File can be created
	pub fn create(mut self) -> Self {
		self.mode |= FileMode::Create;
		self
	}

	/// File can be truncated
	pub fn truncate(mut self) -> Self {
		self.mode |= FileMode::Truncate;
		self
	}

	/// Build the mode
	pub fn build(self) -> u8 {
		self.mode
	}

	/// Check if the file can be read
	pub fn can_read(&self) -> bool {
		self.mode & FileMode::Read == FileMode::Read
	}

	/// Check if the file can be written
	pub fn can_write(&self) -> bool {
		self.mode & FileMode::Write == FileMode::Write
	}

	/// Check if the file can be created
	pub fn can_create(&self) -> bool {
		self.mode & FileMode::Create == FileMode::Create
	}

	/// Check if the file can be truncated
	pub fn can_truncate(&self) -> bool {
		self.mode & FileMode::Truncate == FileMode::Truncate
	}
}

#[test]
fn test_file_modes_building() {
	let mode = FileMode::builder().read().write().build();
	assert_eq!(FileMode::Read & mode, FileMode::Read);
	assert_eq!(FileMode::Write & mode, FileMode::Write);
	assert_eq!(FileMode::Create & mode, FileMode::None);
	assert_eq!(FileMode::Truncate & mode, FileMode::None);

	let mode = FileMode::builder().read().build();
	assert_eq!(FileMode::Read & mode, FileMode::Read);
	assert_eq!(FileMode::Write & mode, FileMode::None);
	assert_eq!(FileMode::Create & mode, FileMode::None);
	assert_eq!(FileMode::Truncate & mode, FileMode::None);

	let mode = FileMode::builder().write().build();
	assert_eq!(FileMode::Read & mode, FileMode::None);
	assert_eq!(FileMode::Write & mode, FileMode::Write);
	assert_eq!(FileMode::Create & mode, FileMode::None);
	assert_eq!(FileMode::Truncate & mode, FileMode::None);

	let mode = FileMode::builder().create().build();
	assert_eq!(FileMode::Read & mode, FileMode::None);
	assert_eq!(FileMode::Write & mode, FileMode::None);
	assert_eq!(FileMode::Create & mode, FileMode::Create);
	assert_eq!(FileMode::Truncate & mode, FileMode::None);

	let mode = FileMode::builder().truncate().build();
	assert_eq!(FileMode::Read & mode, FileMode::None);
	assert_eq!(FileMode::Write & mode, FileMode::None);
	assert_eq!(FileMode::Create & mode, FileMode::None);
	assert_eq!(FileMode::Truncate & mode, FileMode::Truncate);
}