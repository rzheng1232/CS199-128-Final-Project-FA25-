use crate::HttpClient;
use tauri::State;
use serde::{Deserialize, Serialize};

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

#[tauri::command]
pub async fn register(user: String, pass: String, client: State<'_, HttpClient>) -> Result<i32, String> {
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

    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
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
        return Ok(0)
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
// pub async fn list_chats (user: String, client: State<'_, HttpClient>) -> Result<Vec<String>, String> {
//     let url = format!(
//         "http://44.192.82.241/listchats/username/{}}",
//         user
//     );
//     println!("{}", url);
//     let resp = client.0.get(&url).send().await?
//     .json::<ChatInfo>()  // parse JSON here
//     .await?;
// ;

//     let body = match resp {
//         Ok(r) => r.text().await.map_err(|e| e.to_string())?,
//         Err(_) => return Ok(Vec::new()),
//     };
//     let mut chat_names: Vec<String> = Vec::new();
//     for chat in body {
//         chat_names.push(chat.id);
//         println!("Chat id: {}", chat.id);
//         println!("Users: {:?}", chat.users);
//     }
//     return Ok(chat_names);

// }