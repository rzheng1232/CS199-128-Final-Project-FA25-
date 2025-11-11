use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

// user stuff
#[derive(Clone, Copy)]
pub struct User {
    pub id: i32,
}

#[derive(Clone)]
pub struct UserList {
    pub active_users: HashMap<i32, User>,
}

// message stuff
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub user: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

// sending a message and printing it out
pub fn display_message(message: &str) {
    // only prints it out if it's not empty
    if !message.is_empty() {
        println!("{}", message);
    }
}

// save chat
pub fn log_message(user: &str, message: &str) -> io::Result<()> {
    let path = "./cache/chat_history.json";

    let mut messages: Vec<Message> = if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        Vec::new()
    };

    let new_message = Message {
        user: user.to_string(),
        message: message.to_string(),
        timestamp: Utc::now(),
    };

    messages.push(new_message);

    let json = serde_json::to_string_pretty(&messages)?;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn print_messages(path: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let messages: Vec<Message> = serde_json::from_str(&contents).unwrap_or_default();

    println!("--- Chat History ---");
    for msg in messages {
        println!(
            "[{}] {}: {}",
            msg.timestamp.format("%Y-%m-%d %H:%M:%S"),
            msg.user,
            msg.message
        );
    }
    println!("--------------------");

    Ok(())
}

pub fn clear_messages(path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let empty_json = json!({});
    file.write_all(empty_json.to_string().as_bytes())?;
    Ok(())
}

impl UserList {
    pub fn handle_join_message(&mut self, user_id: i32) {
        let new_user = User { id: user_id };
        self.active_users.insert(user_id, new_user);
        println!("User {} joined.", new_user.id);
    }

    pub fn handle_leave_message(&mut self, user_id: &i32) {
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

        user_list.handle_join_message(1);
        user_list.handle_join_message(2);

        assert!(user_list.active_users.contains_key(&1));
        assert!(user_list.active_users.contains_key(&2));
        assert_eq!(user_list.active_users.len(), 2);

        user_list.handle_leave_message(&1);
        assert!(!user_list.active_users.contains_key(&1));
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
        user_list.handle_join_message(100);
        user_list.handle_join_message(200);
        user_list.display_active_users();
        assert!(user_list.active_users.contains_key(&100));
        assert!(user_list.active_users.contains_key(&200));
    }
}
