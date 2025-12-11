use crate::HttpClient;
use tauri::State;
use serde::{Deserialize, Serialize};
use rand::rngs::OsRng;
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;

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
#[tauri::command]
pub async fn login(user: String, pass: String, client: State<'_, HttpClient>) -> Result<i32, String> {
    // fail fast if either field is empty or whitespace
    if user.trim().is_empty() || pass.trim().is_empty() {
        return Ok(0);
    }

    let url = format!(
        "http://44.192.82.241/Authenticate/username/{}/password/{}",
        user, pass
        
    );
    println!("{}", url);

    // If request or JSON parsing fails, just treat as failure (0)
    let resp = client.0.get(&url).send().await;
    
    // let restult;
    // match resp
    // {
    //     Ok(r) => restult = r.text().await.map_err(|e| e.to_string())?,//r.json::<String>().await.map_err(|e| e.to_string())?,
    //     Err(_) => return Ok(0),
    // };
    let body = match resp {
        Ok(r) => r.text().await.map_err(|e| e.to_string())?,
        Err(_) => return Ok(0),
    };
    println!("BODY: {:?}", body);  // OK
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    let n = v["Ok"].as_str().unwrap().parse::<i32>().unwrap();
    if n == 0{
        return Ok(0)
    } else {
        return Ok(1)
    }
}

fn generate_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Key Generation");
    let pub_key = RsaPublicKey::from(&priv_key);
    (priv_key, pub_key)
}

#[tauri::command]
pub async fn register(user: String, pass: String, client: State<'_, HttpClient>) -> Result<i32, String> {
    let (privateKey, publicKey) = generate_keys();

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
