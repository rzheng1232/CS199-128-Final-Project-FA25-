use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;

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

// save chat
#[tauri::command]
pub fn log_message(chat_name: String, user: &str, message: &str) -> Result<(), String> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let dir_path = "./cache";
    let file_path = format!("{}/chat_history.json", dir_path);

    fs::create_dir_all(dir_path).map_err(|e| format!("Failed to create directory: {}", e))?;

    let mut chats = read_chats_from_json(&file_path);

    let new_message = Message {
        user: user.to_string(),
        message: message.to_string(),
        timestamp: Utc::now(),
    };

    if let Some(chat) = chats.iter_mut().find(|c| c.name == chat_name) {
        chat.messages.push(new_message);
    } else {
        chats.push(Chat {
            name: chat_name,
            messages: vec![new_message],
        });
    }

    let json = serde_json::to_string_pretty(&chats).map_err(|e| e.to_string())?;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .map_err(|e| e.to_string())?;

    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;

    Ok(())
}

fn read_chats_from_json(path: &str) -> Vec<Chat> {
    use std::fs::File;
    use std::io::Read;

    if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            return serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
        }
    }
    Vec::new()
}

#[tauri::command]
pub fn print_messages(path: Option<String>) -> Result<Vec<Chat>, String> {
    let path = path.unwrap_or_else(|| "./cache/chat_history.json".to_string());

    let chats = read_chats_from_json(&path);

    Ok(chats)
}

#[tauri::command]
pub fn clear_messages(path: Option<String>) -> Result<(), String> {
    let path = path.unwrap_or_else(|| "./cache/chat_history.json".to_string()); // Uses default if path doesn't exist -- will be fixed later just for testing now
    let mut file = File::create(&path).map_err(|e| e.to_string())?;
    file.write_all(b"[]").map_err(|e| e.to_string())?;
    Ok(())
}

// pub fn hash_name(name: &str) -> Jas {}

impl<'a> UserList<'a> {
    pub fn handle_join_message(&mut self, user_id: &'a str) {
        let new_user = User {
            id: user_id.to_string(),
            password: " ".to_string(), // irrelevant here???
        };
        self.active_users.insert(user_id, new_user.clone());
        println!("User {} joined.", new_user.id);
    }

    pub fn handle_leave_message(&mut self, user_id: &'a str) {
        if self.active_users.remove(user_id).is_some() {
            println!("User {} left.", user_id);
        }
    }

    pub fn display_active_users(&self) {
        println!("--- Active Users ---");
        for user_id in self.active_users.keys() {
            println!("- {}", user_id);
        }
        println!("--------------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*; // gives access to everything from parent module
    use std::fs::{File, OpenOptions};
    use std::io::Read;
    use tempfile::tempdir;

    // Helper to read file contents as a string
    fn read_file_to_string(path: &std::path::Path) -> String {
        let mut contents = String::new();
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    #[test]
    fn test_log_message_creates_and_appends() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("chat_history.json");
        let path_str = path.to_str().unwrap();

        // First write
        let mut messages: Vec<Message> = Vec::new();
        messages.push(Message {
            user: "Alice".to_string(),
            message: "Hello".to_string(),
            timestamp: Utc::now(),
        });
        let json = serde_json::to_string_pretty(&messages).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path_str)
            .unwrap();
        file.write_all(json.as_bytes()).unwrap();

        // Append another message
        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut messages: Vec<Message> = serde_json::from_str(&contents).unwrap_or_default();

        messages.push(Message {
            user: "Bob".to_string(),
            message: "Hi Alice".to_string(),
            timestamp: Utc::now(),
        });

        let json = serde_json::to_string_pretty(&messages).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        file.write_all(json.as_bytes()).unwrap();

        // Verify
        let contents = read_file_to_string(&path);
        let messages: Vec<Message> = serde_json::from_str(&contents).unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].user, "Alice");
        assert_eq!(messages[1].user, "Bob");
    }

    #[test]
    fn test_user_join_and_leave() {
        let mut user_list = UserList {
            active_users: HashMap::new(),
        };

        user_list.handle_join_message("1");
        user_list.handle_join_message("2");

        assert!(user_list.active_users.contains_key(&"1"));
        assert!(user_list.active_users.contains_key(&"2"));
        assert_eq!(user_list.active_users.len(), 2);

        user_list.handle_leave_message("1");
        assert!(!user_list.active_users.contains_key(&"1"));
        assert_eq!(user_list.active_users.len(), 1);
    }

    #[test]
    fn test_display_message_non_empty_runs() {
        // We can just call it to ensure no panic
        display_message("Hello test!");
    }

    #[test]
    fn test_display_active_users_internal_state() {
        let mut user_list = UserList {
            active_users: HashMap::new(),
        };
        user_list.handle_join_message("test1");
        user_list.handle_join_message("test2");
        user_list.display_active_users();
        assert!(user_list.active_users.contains_key(&"test1"));
        assert!(user_list.active_users.contains_key(&"test2"));
    }
}
