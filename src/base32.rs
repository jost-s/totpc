use base32::Alphabet;

/// Decode Base32 encoded key to UTF-8 string.
pub fn decode(key: &str) -> Result<Vec<u8>, String> {
    let maybe_key_bytes = base32::decode(Alphabet::RFC4648 { padding: false }, key);
    maybe_key_bytes.ok_or_else(|| "Error: invalid key encoding (must be Base32)".to_string())
}
