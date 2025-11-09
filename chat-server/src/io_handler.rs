mod io_handler {
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use serde_json::json;
use serde_json::Value;


// user stuff
#[derive(Clone, Copy)]
struct User{
    id: i32,
}

#[derive(Clone)]
struct UserList{
    active_users: HashMap<i32, User>,
}


// message stuff
#[derive(Serialize, Deserialize, Debug)]
struct Message {
    user: String,
    message: String,
    timestamp: DateTime<Utc>, 
}

// sending a message and printing it out
fn display_message(message: &str){ // only prints it out if its not empty
    if message.len() != 0 {
       println!("{}", message);
    }
}

// save chat 
pub fn log_message(user: &str, message: &str) -> io::Result<()>{
    // read existing messages?? gonna rewrite over them with appended
    let path = "./cache/chat_history.txt";
    
    let mut messages: Vec<Message> = if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        Vec::new()
    };

    // create new message
    let new_message = Message{
        user: user.to_string(),
        message: message.to_string(),
        timestamp: chrono::Utc::now(),
    };

    messages.push(new_message);

    // overwrite the entire json
    let json = serde_json::to_string_pretty(&messages)?;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    file.write_all(json.as_bytes())?;
    
    Ok(())
}

impl UserList{
    fn handle_join_message(&mut self, user_id: i32) {
        let new_user = User { id: user_id.clone() };
        self.active_users.insert(user_id, new_user);
        println!("User {} joined.", new_user.id);
    }

    fn handle_leave_message(&mut self, user_id: &i32) {
        if self.active_users.remove(user_id).is_some() {
            println!("User {} left.", user_id);
        }
    }

    fn display_active_users(&self) {
        println!("--- Active Users ---");
        for user_id in self.active_users.keys() {
            println!("- {}", user_id);
        }
        println!("--------------------");
    }
}
}