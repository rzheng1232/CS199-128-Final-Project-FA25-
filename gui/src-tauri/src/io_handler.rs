use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::read_dir;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use crate::HttpClient;
use tauri::State;

// user stuff
#[derive(Clone, Hash)]
pub struct User {
    pub id: String,
    pub password: String,
}

#[derive(Clone)]
pub struct UserList<'a> {
    pub active_users: HashMap<&'a str, User>,
}

// message stuff
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub user: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatHistoryMessage {
    pub username: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    pub name: String,
    pub messages: Vec<Message>,
}

// sending a message and printing it out
pub fn display_message(message: &str) {
    // only prints it out if it's not empty
    if !message.is_empty() {
        println!("{}", message);
    }
}

fn read_messages_from_chat_json(path: &str) -> Vec<Message> {
    if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            return serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
        }
    }
    Vec::new()
}

fn read_chats_from_json(path: &str) -> Vec<Chat> {
    if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            return serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
        }
    }
    Vec::new()
}

// save chat
#[tauri::command]
pub async fn log_message(id: String, username: String, message: String, 
    client: State<'_, HttpClient>,) -> Result<(), String> { // the terminal chat isn't showng up yes i tried it didnt wor
    let url = format!("http://44.192.82.241/newmessage/chatname/{}/username/{}", id, username);
    println!("{}", url); //have u sent anythign ok so obv this isnt running // might be a front end issue never calling this or somehting
    let res = client.0.post(&url).header("Content-Type", "application/json").json(&serde_json::json!({ "content": message })).send().await.map_err(|e| e.to_string())?;
    // println!("Response status: {}", res.status()); // yeah no references found of rl:Loloolo imma go searchog_message lolololo
        
    Ok(())
}

#[tauri::command]
pub async fn print_messages(
    id: String,
    client: State<'_, HttpClient>,
) -> Result<Vec<ChatHistoryMessage>, String> {
    let url = format!("http://44.192.82.241/getchat/chatname/{}", id);
    let messages:Vec<ChatHistoryMessage> = client.0.get(&url).send().await.map_err(|e| e.to_string())?.json::<Vec<ChatHistoryMessage>>().await.map_err(|e| e.to_string())?;
    // wait can u just try compiling huh did 
    ///lmk waht happens
    /// // it compiled but no messages sent obvs still yerp well it could still be a frontend issue
    /// Debugging time!!!
    Ok(messages)
    
}
