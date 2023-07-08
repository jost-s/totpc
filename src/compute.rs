use hmac::{Hmac, Mac};
use sha1::Sha1;

/// Compute a time-based one time password according to RFC 2468 from given
/// plain text key and time step.
pub fn compute(key: &[u8], time_step: u64) -> Result<String, String> {
    let time_step_bytes = time_step.to_be_bytes();

    let mut mac = Hmac::<Sha1>::new_from_slice(&key)
        .map_err(|error| format!("Error: invalid key length - {error}"))?;
    mac.update(&time_step_bytes);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();

    let last_byte = code_bytes.last().ok_or_else(|| {
        "Error: could not compute TOTP (MAC does not contain any bytes)".to_string()
    })?;
    let offset = last_byte & 0xf;
    let offset = offset as usize;

    let dynamic_binary_code = (code_bytes[offset] as u32 & 0x7f) << 24
        | (code_bytes[offset + 1] as u32 & 0xff) << 16
        | (code_bytes[offset + 2] as u32 & 0xff) << 8
        | (code_bytes[offset + 3] as u32 & 0xff);

    let base: u32 = 10;
    let totp_length = 6;
    let modulo_operator = base.pow(totp_length);
    let totp = dynamic_binary_code % modulo_operator;
    let totp_length = totp_length as usize;
    let totp_digits = format!("{:0totp_length$}", totp);

    Ok(totp_digits)
}

#[cfg(test)]
mod tests {
    use crate::compute::compute;

    #[test]
    fn test_encode_step_1_1() {
        let key = Vec::<u8>::from("12345678901234567890");
        let time_step = 1;

        let totp = compute(&key, time_step).unwrap();

        assert_eq!(totp, "287082".to_string());
    }

    #[test]
    fn test_encode_step_1_37037036() {
        let key = Vec::from("12345678901234567890");
        let time_step = 37037036;

        let totp = compute(&key, time_step).unwrap();

        assert_eq!(totp, "081804".to_string());
    }

    // #[test]
    // fn test_encode_step_1_37037037() {
    //     let key = b"12345678901234567890";
    //     let time_step = 37037037;

    //     let totp = compute(key, time_step).unwrap();

    //     assert_eq!(totp, "050471".to_string());
    // }

    // // #[test]
    // // fn test_encode_step_1_256() {
    // //     let key = b"12345678901234567890123456789012";
    // //     println!("key {:x?}", key);
    // //     let time_step = 1;

    // //     let totp = encode(key, time_step);

    // //     assert_eq!(totp, "119246".to_string());
    // // }

    // // #[test]
    // // fn test_encode_step_1_512() {
    // //     let key = b"1234567890123456789012345678901234567890123456789012345678901234";
    // //     let time_step = 1;

    // //     let totp = encode(key, time_step);

    // //     assert_eq!(totp, "693936".to_string());
    // // }

    // #[test]
    // fn test_encode_step_1628693586() {
    //     let key = b"$3cr3tP4$$";
    //     let time = 1628693586;
    //     let time_step_interval = 30;
    //     let time_step = time / time_step_interval;

    //     let totp = compute(key, time_step).unwrap();

    //     assert_eq!(totp, "053630".to_string());
    // }

    // #[test]
    // fn test_encode_step_1661048541_abc() {
    //     let key = b"ABC";
    //     let time = 1661048541;
    //     let time_step_interval = 30;
    //     let time_step = time / time_step_interval;

    //     let totp = compute(key, time_step).unwrap();

    //     assert_eq!(totp, "799774".to_string());
    // }
}
