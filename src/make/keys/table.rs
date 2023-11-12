use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Row, Table};
use crate::make::keys::structures::environment_record::EnvironmentRecord;
use crate::make::keys::structures::environment_variables::{ENVIRONMENT_VARIABLES_KEYS, EnvironmentVariables};

/// Pack the environment variables into a vector of rows to be used by the table
fn pack_table_rows(env_variables: &EnvironmentVariables) -> Vec<Row> {
	ENVIRONMENT_VARIABLES_KEYS.iter()
	                          .map(|key| &env_variables[key] as &EnvironmentRecord) // get the value of the key (the struct key hardcoded in the array)
	                          .map(|ev| Row::from(vec![ev.name(), ev.value()]))
	                          .collect()
}

/// Display the environment variables table
pub fn display_environment_variables_table(env_variables: &EnvironmentVariables) {
	let mut table = Table::new();
	table.load_preset(UTF8_FULL)
	     .set_header(vec![
		     Cell::new("Environment variable name").add_attribute(Attribute::Bold),
		     Cell::new("Value").add_attribute(Attribute::Bold),
	     ])
	     .add_rows(pack_table_rows(env_variables));

	println!("{table}");
}