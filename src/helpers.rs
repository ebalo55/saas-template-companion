use alkali::{AlkaliError, encode::base64};

/// Create a base64 url encoded version of the provided bytecodes
pub fn base64_url(bytes: &[u8]) -> Result<String, AlkaliError> {
	base64::encode(bytes, base64::Variant::URLSafe)
}