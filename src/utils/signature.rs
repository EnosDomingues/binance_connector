use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn generate_signature(query_string: &str, secret_key: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
        .expect("HMAC can accept keys of any size");

    mac.update(query_string.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}