use crate::HttpClient;
use tauri::State;
use serde::{Deserialize, Serialize};
use rand::rngs::OsRng;
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, PasswordHash, PasswordVerifier};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, KeyInit};
use std::env;

#[derive(serde::Deserialize)]
pub enum ApiResult {
    Ok(()),
    Err,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatInfo {
    id: String,
    users: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    Ok: Vec<ChatInfo>,
}

fn verify_password(hash_str: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash_str).expect("invalid stored hash");
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

fn decrypt_private_key_for_user(user: &UserRecord, password: &str) -> RsaPrivateKey {
    let enc_key = derive_enc_key(password, &user.enc_salt);

    let key = Key::from_slice(&enc_key);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = Nonce::from_slice(&user.enc_nonce);

    let priv_der = cipher
        .decrypt(nonce, user.encrypted_private_key.as_ref())
        .expect("private key decryption failed");

    RsaPrivateKey::from_pkcs8_der(&priv_der).expect("failed to decode private key")
}

#[tauri::command]
pub async fn login(user: String, pass: String, client: State<'_, HttpClient>) -> Result<i32, String> {
    // fail fast if either field is empty or whitespace
    if user.trim().is_empty() || pass.trim().is_empty() {
        return Ok(0);
    }

    // Get stuff
}

fn generate_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Key Generation");
    let pub_key = RsaPublicKey::from(&priv_key);
    (priv_key, pub_key)
}

fn hash_password_for_auth(password: &str) -> (String, String) {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("argon2 auth hash failed")
        .to_string();
    (hash, salt.to_string())
}

fn derive_enc_key(password: &str, enc_salt: &[u8]) -> [u8; 32] {
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), enc_salt, &mut key)
        .expect("argon2 enc_key failed");
    key
}

fn encrypt_private_key(priv_der: &[u8], enc_key: [u8; 32]) -> (Vec<u8>, [u8; 12]) {
    let key = Key::from_slice(&enc_key);
    let cipher = ChaCha20Poly1305::new(key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, priv_der)
        .expect("encrypt private key failed");

    (ciphertext, nonce_bytes)
}

#[tauri::command]
pub async fn register(user: String, pass: String, client: State<'_, HttpClient>) -> Result<i32, String> {
    let (privateKey, publicKey) = generate_keys(); // Get keys

    // Encode to send to server
    let pub_der = pubk.to_public_key_der().expect("pub encode").as_bytes().to_vec();
    let priv_der = privk.to_pkcs8_der().expect("priv encode").as_bytes().to_vec();

    let (auth_hash, auth_salt) = hash_password_for_auth(&password); // Encrypt password 

    // Encrypt private key
    let mut enc_salt = vec![0u8; 16];
    OsRng.fill_bytes(&mut enc_salt);
    let enc_key = derive_enc_key(&password, &enc_salt); 
    let (encrypted_private_key, enc_nonce) = encrypt_private_key(&priv_der, enc_key);

    // Trims username and password
    if user.trim().is_empty() || pass.trim().is_empty() {
        return Ok(0);
    }

    let url = format!(
        "http://44.192.82.241/createaccount/username/{}/password/{}",
        user, pass
    );

    let resp = client.0.get(&url).send().await;

    let body = match resp {
        Ok(r) => r.text().await.map_err(|e| e.to_string())?,
        Err(_) => return Ok(0),
    };
    println!("{}", body);
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
   
    println!("{}", v);
    let n = v["Ok"].as_str().unwrap().parse::<i32>().unwrap();

    if n == 0{
        return Ok(0)
    } else {
        return Ok(1)
    }
}

#[tauri::command]
pub async fn handleNewChat(currentUser: String, user: String, client: State<'_, HttpClient>) -> Result<i32, String>{
    if user.is_empty() {return Ok(0)}

    let url = format!(
        "http://44.192.82.241/checkuser/username/{}",
        user
    );
    println!("{}", url);
    let resp = client.0.get(&url).send().await;

    let body = match resp {
        Ok(r) => r.text().await.map_err(|e| e.to_string())?,
        Err(_) => return Ok(0),
    };
    println!("BODY: {:?}", body); 

    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    let n = v["Ok"].as_str().unwrap().parse::<i32>().unwrap();

    if n == 0{
        return Ok(0);
    } else {
        let ChatName = format!("{}{}", currentUser, user);
        let url2 = format!(
            "http://44.192.82.241/createchat?name={}&user={}&user={}",
            ChatName, user, currentUser
        );
        let resp2 = client.0.get(&url2).send().await;

        let body2 = match resp2 {
            Ok(r) => r.text().await.map_err(|e| e.to_string())?,
            Err(_) => return Ok(0),
        };

        let v2: serde_json::Value = serde_json::from_str(&body2).unwrap();
        let n2 = v2["Ok"].as_str().unwrap().parse::<i32>().unwrap();

        Ok(1)
    }
}
#[tauri::command]
pub async fn list_chats(
    user: String,
    client: State<'_, HttpClient>
) -> Result<Vec<ChatInfo>, String> {
    let url = format!("http://44.192.82.241/listchats/username/{}", user);
    println!("Requesting URL: {}", url);

    // Make the GET request and parse JSON
    let resp = client.0
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<ChatResponse>()   // parse into top-level wrapper
        .await
        .map_err(|e| e.to_string())?;

    // Extract chat names
    let mut chat_names: Vec<ChatInfo> = Vec::new();
    for chat in resp.Ok {
        println!("Chat id: {}", chat.id);
        println!("Users: {:?}", chat.users);
        chat_names.push(chat);
    }

    Ok(chat_names)
}

#[tauri::command]
pub async fn delete_chat(user: String, id: String, client: State<'_, HttpClient>) -> Result<i32, String> {
    let url = format!("http://44.192.82.241/deletechat/username/{}/chatname/{}", id, user);
    println!("deleting {}", id);
    
    let resp = client.0.get(&url).send().await.map_err(|e| e.to_string())?;
    let body = resp.text().await.map_err(|e| e.to_string())?;

    let v = serde_json::from_str::<serde_json::Value>(&body)
        .map_err(|e| format!("JSON parse error: {}", e))?;
    
    // Safe field access with defaults
    let n = v.get("Ok")
        .and_then(|ok| ok.as_str())
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    
    Ok(n)  // Returns 0 or 1 directly - no if needed
}
