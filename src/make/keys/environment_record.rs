use serde::Serialize;
use crate::json_serialize_to_string;

#[derive(Serialize)]
pub struct EnvironmentRecord<'a> {
	/// Raw environment variable name
	env_name: &'a str,
	/// Value to represent (or represented) in the environment variable
	value: &'a str,
	/// Whether the environment variable was updated or not
	updated: bool,
}
json_serialize_to_string!(EnvironmentRecord<'_>);

impl<'a> EnvironmentRecord<'a> {
	/// Create a new record with the default values
	pub fn new(env_name: &'a str, initial_value: &'a str) -> Self {
		EnvironmentRecord {
			env_name,
			value: initial_value,
			updated: false,
		}
	}

	/// Update the value of the record
	pub fn update(&mut self, value: &'a str) {
		self.value = value;
		self.updated = true;
	}

	/// Get the name of the environment variable
	pub fn name(&self) -> &str {
		self.env_name
	}

	/// Get the value of the environment variable
	pub fn value(&self) -> &str {
		self.value
	}

	/// Whether the environment variable was updated or not
	pub fn updated(&self) -> bool {
		self.updated
	}
}