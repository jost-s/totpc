use base32::Alphabet;

pub fn decode(key: &str) -> Result<Vec<u8>, String> {
    let maybe_key_bytes = base32::decode(Alphabet::RFC4648 { padding: false }, key);
    match maybe_key_bytes {
        None => Err("Error: invalid key encoding (must be Base32)".to_string()),
        Some(bytes) => Ok(bytes),
    }
}
