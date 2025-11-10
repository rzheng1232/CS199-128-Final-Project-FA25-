use rsa::{PublicKey, RsaPublicKey, RsaPrivateKey, PaddingScheme};
use rand::rngs::OsRng;

/// Encrypts a message from bytes using a public key
fn encrypt(message: &[u8], public_key: &RsaPublicKey) -> Vec<u8> {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    public_key.encrypt(&mut OsRng, padding, message).expect("failed to encrypt")
}

/// Encrypts a message from bytes using a private key
fn decrypt(encrypted: &[u8], private_key: &RsaPrivateKey) -> Vec<u8> {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    private_key.decrypt(padding, encrypted).expect("failed to decrypt")
}

fn convert_to_utf(message: &str) -> Vec<u8> {
    message.as_bytes().to_vec()
}

fn convert_to_string(message: &[u8]) -> String {
    String::from_utf8_lossy(message).to_string()
}

/// Encrypts a message from a String using a public key
fn encrypt(message: &String, public_key: &RsaPublicKey) -> Vec<u8> {
    encrypt(message.as_bytes(), public_key)
}

/// Decrypts a message from bytes using a private key, returns a String
fn decrypt(encrypted: &[u8], private_key: &RsaPrivateKey) -> String {
    let decrypted_bytes = decrypt(encrypted, private_key);
    bytes_to_string(&decrypted_bytes)
}