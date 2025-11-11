use rand::rngs::OsRng;
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;

/// Encrypts a message from bytes using a public key
fn encrypt(message: &[u8], public_key: &RsaPublicKey) -> Vec<u8> {
    let mut rng = OsRng;
    let padding = Oaep::new::<Sha256>();
    public_key
        .encrypt(&mut rng, padding, message)
        .expect("Failed to Encrypt")
}

/// Encrypts a message from bytes using a private key
fn decrypt(encrypted: &[u8], private_key: &RsaPrivateKey) -> Vec<u8> {
    let padding = Oaep::new::<Sha256>();
    private_key
        .decrypt(padding, encrypted)
        .expect("Failed to Decrypt")
}

/// Converts a string to a vector of bytes
fn convert_to_utf(message: &str) -> Vec<u8> {
    message.as_bytes().to_vec()
}

/// Converts a vector of bytes to a string
fn convert_to_string(message: &[u8]) -> String {
    String::from_utf8_lossy(message).to_string()
}

/// Encrypts a message from a String using a public key
fn encrypt_string(message: &String, public_key: &RsaPublicKey) -> Vec<u8> {
    encrypt(message.as_bytes(), public_key)
}

/// Decrypts a message from bytes using a private key, returns a String
fn decrypt_string(encrypted: &[u8], private_key: &RsaPrivateKey) -> String {
    let decrypted_bytes = decrypt(encrypted, private_key);
    convert_to_string(&decrypted_bytes)
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::rngs::OsRng;
    use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
    use sha2::Sha256;

    fn generate_keys() -> (RsaPrivateKey, RsaPublicKey) {
        let mut rng = OsRng;
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Key Generation");
        let pub_key = RsaPublicKey::from(&priv_key);
        (priv_key, pub_key)
    }

    #[test]
    fn test_encrypt() {
        let (_priv, pubk) = generate_keys();
        let message = b"Encrypt Test Message"; // Testing directly on a byte string 
        let mut rng = OsRng;
        let padding = Oaep::new::<Sha256>();
        let encrypted = pubk
            .encrypt(&mut rng, padding, message)
            .expect("encryption failed");
        assert_ne!(&encrypted[..], &message[..]); // Ensure its actually encrypted and does not match
    }

    #[test]
    fn test_decrypt() {
        let (privk, pubk) = generate_keys();
        let message = b"Decrypt Test Message"; // Testing directly on a byte string 
        let mut rng = OsRng;
        let padding = Oaep::new::<Sha256>();
        let encrypted = pubk
            .encrypt(&mut rng, padding, message)
            .expect("encryption failed");
        let padding_decrypt = Oaep::new::<Sha256>();
        let decrypted = privk
            .decrypt(padding_decrypt, &encrypted)
            .expect("decryption failed");
        assert_eq!(&decrypted, message); // Ensure decrypted message matches original, tests both phases
    }

    #[test]
    fn test_string_to_byte() {
        let s = String::from("byte test");
        let bytes = s.as_bytes().to_vec();
        assert_eq!(bytes, vec![98, 121, 116, 101, 32, 116, 101, 115, 116]);
    }

    #[test]
    fn test_byte_to_string() {
        let bytes = vec![114, 117, 115, 116];
        let s = String::from_utf8(bytes.clone()).expect("UTF-8 conversion");
        assert_eq!(s, "rust");
    }

    #[test]
    fn test_string_encrypt() {
        let (privk, pubk) = generate_keys();
        let message = "String Encrypt"; // Testing on a string thats not bytes
        let mut rng = OsRng;
        let padding = Oaep::new::<Sha256>();
        let encrypted = pubk
            .encrypt(&mut rng, padding, message.as_bytes())
            .expect("encryption failed");
        assert_ne!(&encrypted[..], message.as_bytes());
    }

    #[test]
    fn test_string_decrypt() {
        let (privk, pubk) = generate_keys();
        let message = "String Decrypt"; // Testing on a string thats not bytes
        let mut rng = OsRng;
        let padding = Oaep::new::<Sha256>();
        let encrypted = pubk
            .encrypt(&mut rng, padding, message.as_bytes())
            .expect("encryption failed"); // Encrypts then decrypts to test both sides
        let padding_decrypt = Oaep::new::<Sha256>();
        let decrypted = privk
            .decrypt(padding_decrypt, &encrypted)
            .expect("decryption failed");
        let decrypted_str = String::from_utf8(decrypted).expect("utf8 conversion");
        assert_eq!(decrypted_str, message);
    }
}