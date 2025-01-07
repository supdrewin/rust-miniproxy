use base64::{engine::general_purpose::STANDARD as base64, DecodeError, Engine};
use rand::seq::SliceRandom;

pub fn new() -> String {
    let mut password = (0..=255).collect::<Vec<_>>();
    password.shuffle(&mut rand::thread_rng());

    base64.encode(&password)
}

pub fn encode_password(password: &str) -> String {
    base64.encode(password)
}

pub fn decode_password(password: &str) -> Result<Vec<u8>, DecodeError> {
    base64.decode(password)
}
