/// This macro is used to implement the `Into<String>` trait for a type that can be serialized to JSON.
/// Note that this macro is only available when the `serde_json` crate is loaded.
#[macro_export]
macro_rules! json_serialize_to_string {
    ( $x:ty ) => {
            impl Into<String> for $x {
				fn into(self) -> String {
					serde_json::to_string(&self).unwrap()
				}
			}
    };
}