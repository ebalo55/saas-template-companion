use std::ops::{BitAnd, BitOr, BitOrAssign};
use crate::structures::file_mode_builder::FileModeBuilder;

#[derive(Debug, PartialEq)]
pub enum FileMode {
	None = 0b0000,
	Read = 0b0001,
	Write = 0b0010,
	Create = 0b0100,
	Truncate = 0b1000,
}

impl FileMode {
	pub fn builder() -> FileModeBuilder {
		FileModeBuilder::default()
	}

	/// Convert u8 to FileMode
	pub fn from(mode: &u8) -> Self {
		match mode {
			0b0001 => FileMode::Read,
			0b0010 => FileMode::Write,
			0b0100 => FileMode::Create,
			0b1000 => FileMode::Truncate,
			_ => FileMode::None,
		}
	}
}

/// Implicitly convert FileMode to u8
impl Into<u8> for FileMode {
	fn into(self) -> u8 {
		self as u8
	}
}

/// Implement the equality operator for FileMode and u8
/// # Example
/// ```rust
/// assert_eq!(FileMode::Read, 0b0001);
/// assert_eq!(FileMode::Write, 0b0010);
/// assert_eq!(FileMode::Create, 0b0100);
/// assert_eq!(FileMode::Truncate, 0b1000);
/// ```
impl PartialEq<u8> for FileMode {
	fn eq(&self, other: &u8) -> bool {
		*self == FileMode::from(other)
	}
}

/// Implement the equality operator for FileMode and u8
/// # Example
/// ```rust
/// assert_eq!(FileMode::Read, 0b0001);
/// assert_eq!(FileMode::Write, 0b0010);
/// assert_eq!(FileMode::Create, 0b0100);
/// assert_eq!(FileMode::Truncate, 0b1000);
/// ```
impl PartialEq<FileMode> for u8 {
	fn eq(&self, other: &FileMode) -> bool {
		FileMode::from(self) == *other
	}
}

#[test]
fn test_into_u8() {
	assert_eq!(FileMode::Read, 0b0001);
	assert_eq!(FileMode::Write, 0b0010);
	assert_eq!(FileMode::Create, 0b0100);
	assert_eq!(FileMode::Truncate, 0b1000);
}

/// Implement the bitwise and operator for FileMode and u8
/// # Example
/// ```rust
/// let mode = FileMode::builder().read().write().build();
/// assert_eq!(FileMode::Read & mode, FileMode::Read);
/// assert_eq!(FileMode::Write & mode, FileMode::Write);
/// assert_eq!(FileMode::Create & mode, 0);
/// assert_eq!(FileMode::Truncate & mode, 0);
/// ```
impl BitAnd<u8> for FileMode {
	type Output = u8;

	fn bitand(self, rhs: u8) -> Self::Output {
		(self as u8) & rhs
	}
}

#[test]
fn test_bitand_u8() {
	let mode = FileMode::builder().read().write().build();
	assert_eq!(FileMode::Read & mode, FileMode::Read);
	assert_eq!(FileMode::Write & mode, FileMode::Write);
	assert_eq!(FileMode::Create & mode, FileMode::None);
	assert_eq!(FileMode::Truncate & mode, FileMode::None);
}

/// Implement the bitwise and operator for u8 and FileMode
/// # Example
/// ```rust
/// let mode = FileMode::builder().read().write().build();
/// assert_eq!(mode & FileMode::Read, FileMode::Read);
/// assert_eq!(mode & FileMode::Write, FileMode::Write);
/// assert_eq!(mode & FileMode::Create, 0);
/// assert_eq!(mode & FileMode::Truncate, 0);
/// ```
impl BitAnd<FileMode> for u8 {
	type Output = u8;

	fn bitand(self, rhs: FileMode) -> Self::Output {
		self & (rhs as u8)
	}
}

#[test]
fn test_bitand_file_mode() {
	let mode = FileMode::builder().read().write().build();
	assert_eq!(mode & FileMode::Read, FileMode::Read);
	assert_eq!(mode & FileMode::Write, FileMode::Write);
	assert_eq!(mode & FileMode::Create, FileMode::None);
	assert_eq!(mode & FileMode::Truncate, FileMode::None);
}

/// Implement the bitwise or operator for FileModes
/// # Example
/// ```rust
/// let mode = FileMode::builder().read().write().build();
/// assert_eq!(FileMode::Read | FileMode::Write, mode);
/// ```
impl BitOr<FileMode> for FileMode {
	type Output = u8;

	fn bitor(self, rhs: FileMode) -> Self::Output {
		(self as u8) | (rhs as u8)
	}
}

#[test]
fn test_bitor_file_mode() {
	let mode = FileMode::builder().read().write().build();
	assert_eq!(FileMode::Read | FileMode::Write, mode);
}

/// Implement the bitwise or operator for FileModes
/// # Example
/// ```rust
/// let mode = FileMode::builder().read().write().build();
/// assert_eq!(FileMode::Read | FileMode::Write, mode);
/// ```
impl BitOr<FileMode> for u8 {
	type Output = u8;

	fn bitor(self, rhs: FileMode) -> Self::Output {
		self | (rhs as u8)
	}
}

#[test]
fn test_bitor_u8() {
	let mode = FileMode::builder().read().write().create().build();
	assert_eq!(FileMode::Read | FileMode::Write | FileMode::Create, mode);
}

/// Implement the bitwise or assignment operator for FileMode and u8
/// # Example
/// ```rust
/// let mut mode = FileMode::None;
/// mode |= FileMode::Read;
/// assert_eq!(mode, FileMode::Read);
/// mode |= FileMode::Write;
/// assert_eq!(mode, FileMode::Read | FileMode::Write);
/// mode |= FileMode::Create;
/// assert_eq!(mode, FileMode::Read | FileMode::Write | FileMode::Create);
/// mode |= FileMode::Truncate;
/// assert_eq!(mode, FileMode::Read | FileMode::Write | FileMode::Create | FileMode::Truncate);
/// ```
impl BitOrAssign<FileMode> for u8 {
	fn bitor_assign(&mut self, rhs: FileMode) {
		*self |= rhs as u8;
	}
}

#[test]
fn test_bitor_assign_file_mode() {
	let mut mode = FileMode::None as u8;
	mode |= FileMode::Read;
	assert_eq!(mode, FileMode::Read);
	mode |= FileMode::Write;
	assert_eq!(mode, FileMode::Read | FileMode::Write);
	mode |= FileMode::Create;
	assert_eq!(mode, FileMode::Read | FileMode::Write | FileMode::Create);
	mode |= FileMode::Truncate;
	assert_eq!(mode, FileMode::Read | FileMode::Write | FileMode::Create | FileMode::Truncate);
}