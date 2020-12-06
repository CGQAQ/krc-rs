const KEYS: [u8; 16] = [64, 71, 97, 119, 94, 50, 116, 71, 81, 54, 49, 45, 206, 210, 110, 105];


use base64::{decode_config, STANDARD_NO_PAD};
use flate2::read::ZlibDecoder;
use std::io::Read;

pub fn parse(input: &[u8]) -> Result<String, String> {
    let mut base64_decoded = decode_config(input, STANDARD_NO_PAD).expect("input value is invalid");

    if String::from_utf8_lossy(&base64_decoded[..4]) != "krc1" {
        return Err("Input is not a krc file!".to_string());
    }

    let (_, input) = base64_decoded.split_at_mut(4);

    for i in 0..input.len() {
        input[i] ^= KEYS[i % 16];
    }

    let mut decoder = ZlibDecoder::new(&input[..]);
    let mut result = String::new();
    decoder.read_to_string(&mut result).unwrap();

    return Ok(result);
}
