mod io_handler;
use io_handler::*;
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

fn main() {
    let history_path = "./cache/chat_history.json";
    if !Path::new(history_path).exists() {
        // Make sure the ./cache directory exists
        fs::create_dir_all("./cache").unwrap();
        // Create an empty JSON array in the file
        fs::write(history_path, "[]").unwrap();
    }

    let mut user_list = UserList {
        active_users: std::collections::HashMap::new(),
    };

    println!("=== chat ===");

    // Ask for username
    print!("Enter your username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim().to_string();

    print_messages("./cache/chat_history.json").unwrap();
    println!("Welcome, {}! Type your messages below.", username);
    println!("(Type '/users' to see who's active, '/quit' to exit)\n");

    // Simulate user joining
    let user_id = rand::random::<i32>().abs(); // random ID BUT MAKE IT NOT RANDOM! MAKE IT A HASH OF THE USER
    user_list.handle_join_message(user_id);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let message = input.trim();

        if message.eq_ignore_ascii_case("/quit") {
            user_list.handle_leave_message(&user_id);
            println!("Goodbye!");
            break;
        } else if message.eq_ignore_ascii_case("/users") {
            user_list.display_active_users();
        } else if message.eq_ignore_ascii_case("/clear") {
            clear_messages("./cache/chat_history.json");
        } else if !message.is_empty() {
            // Log and display message
            if let Err(e) = log_message(&username, message) {
                eprintln!("Error saving message: {}", e);
            } else {
                display_message(&format!("{}: {}", username, message));
            }
        }
    }
}
