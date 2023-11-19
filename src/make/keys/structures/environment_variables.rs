use std::ops::{Index, IndexMut};

use serde::Serialize;

use crate::json_serialize_to_string;
use crate::make::keys::structures::environment_record::EnvironmentRecord;

pub const ENVIRONMENT_VARIABLES_KEYS: [&str; 3] = [
	"next_auth_secret",
	"asymmetric_encryption_public_key",
	"asymmetric_encryption_private_key"
];

#[derive(Serialize, Clone, Debug)]
pub struct EnvironmentVariables<'a> {
	pub next_auth_secret: EnvironmentRecord<'a>,
	pub asymmetric_encryption_public_key: EnvironmentRecord<'a>,
	pub asymmetric_encryption_private_key: EnvironmentRecord<'a>,
}
json_serialize_to_string!(EnvironmentVariables<'_>);

/// Allow indexing by key
impl<'a> Index<&'a str> for EnvironmentVariables<'a> {
	type Output = EnvironmentRecord<'a>;

	fn index(&self, index: &'_ str) -> &Self::Output {
		match index {
			"next_auth_secret" => &self.next_auth_secret,
			"asymmetric_encryption_public_key" => &self.asymmetric_encryption_public_key,
			"asymmetric_encryption_private_key" => &self.asymmetric_encryption_private_key,
			_ => panic!("Unknown key '{}'", index),
		}
	}
}

/// Allow mutable indexing by key
impl<'a> IndexMut<&'a str> for EnvironmentVariables<'a> {
	fn index_mut(&mut self, index: &'a str) -> &mut Self::Output {
		match index {
			"next_auth_secret" => &mut self.next_auth_secret,
			"asymmetric_encryption_public_key" => &mut self.asymmetric_encryption_public_key,
			"asymmetric_encryption_private_key" => &mut self.asymmetric_encryption_private_key,
			_ => panic!("Unknown key '{}'", index),
		}
	}
}

