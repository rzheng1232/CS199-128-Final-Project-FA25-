use axum::{
    extract::Path, response::Json, routing::get, routing::post, Router, extract::State,
};
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};use sqlx::{query, SqlitePool};
use chrono;
use tokio;

// Things the central sever processor needs to handle:
//    User prescence: Whether a user is currently online or not
//    User authentication & account login
//    "Ground truth" chat history: keeps a central state of the chat history that users pull from when they login
//    Message queue: Incoming message requests are added to a queue processed and updated to everyone's chats one at a time to prevent conflict
//    Chatroom management: Json 
#[derive(Deserialize)]
struct Message{
    content: String
}
#[derive(Deserialize, Serialize)]
struct ChatHistoryMessage{
    username: String,
    content: String,
    created_at: String,
}
#[derive(Deserialize)]
struct CreateChatParams {
    name: String,
    user: Vec<String>, // ?user=alice&user=bob → vec!["alice", "bob"]
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    const NUM_THREADS:i32 = 1;
    dotenv::dotenv().ok();
    let pool = SqlitePool::connect("sqlite:chat.db").await?;
    let mut thread_handlers = Vec::new();
    for i in 0..NUM_THREADS{
        let thread_pool = pool.clone();
        thread_handlers.push(tokio::spawn(async move {
            message_thread(thread_pool).await;
        }));
    }
    
    let app = Router::new()
        .route("/", get(root))
        .route("/Authenticate/username/{name}/password/{pass}", get(login))
        .route("/createaccount/username/{name}/password/{pass}", get(new_user))
        .route("/createchat", get(new_chat))
        .route("/newmessage/chatname/{chat}/username/{user}", post(incoming_message))
        .route("/getchat/chatname/{chat}", get(get_message_history))
        .with_state(pool.clone());
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
    // 
}

async fn root() -> Json<String>{
    println!("200");
    Json(String::from("Root!"))
}

/// Background thread for message processing tasks, retrieves oldest unprocessed message in the message_queue, processes it, and adds to the chat_history_cache json
/// TODO: Shared state concurency & synchronization when running multiple message_threads on sqlite database
async fn message_thread(pool:SqlitePool){
    let limit:i64 = 5;
    while true {
        let curr_message = match query!(
            "SELECT id, message_id FROM message_queue WHERE status = 'Queued' ORDER BY queued_at ASC LIMIT ?", limit)
            .fetch_optional(&pool)
            .await
            .unwrap() {
            Some(msg) => msg,
            None => {println!("No valid messages, trying agian in 5 seconds... ");tokio::time::sleep(std::time::Duration::from_secs(5)).await; continue;},
        };
        let message_stuff = query!(
            "SELECT content, chat_id, user_id FROM messages WHERE id = ?", curr_message.message_id).
            fetch_one(&pool).await.unwrap();
        let username = query!("SELECT username FROM users WHERE id = ?", message_stuff.user_id)
            .fetch_one(&pool)
            .await.unwrap().username; 
        let message_content = message_stuff.content;
        let chat_id = message_stuff.chat_id;
        // TODO: Do something, maybe filtering bad words or chat moderation
        let json_message_history = query!(
            "SELECT message_history FROM chat_history_cache WHERE chat_id = ?", chat_id).
            fetch_one(&pool).await.unwrap().message_history;
        if let Some(json_string) = json_message_history {
            let mut messages: Vec<ChatHistoryMessage> = serde_json::from_str(&json_string).unwrap();
            messages.push(ChatHistoryMessage{
                username:username,
                content: message_content,
                created_at: chrono::Utc::now().to_rfc3339(),
            });
            let json_history = serde_json::to_string(&messages).unwrap();
            query!(
                "UPDATE chat_history_cache SET message_history = ? WHERE id = ?",
                json_history,
                chat_id
            ).execute(&pool)
            .await.unwrap();
        }
        println!("Updated cache history");
        query!(
            "UPDATE message_queue SET status = ? WHERE message_id = ?",
            "Finished",
            curr_message.message_id,
        ).execute(&pool)
        .await.unwrap();
        query!(
            "UPDATE messages SET status = ? WHERE id = ?",
            "Sent!",
            curr_message.message_id
        ).execute(&pool).await.unwrap();
    }
}
/// Retrieves chat history given chatname
/// # Query format:
/// curl "http://127.0.0.1:3000/getchat/chatname/ChatName" 
/// # Return format:
/// Array of ChatHistoryMessage datatypes, each containing "username", "content", and "created_at" headers
async fn get_message_history(
    Path(chatname):Path<String>, State(pool): State<SqlitePool>)->Result<Json<Vec<ChatHistoryMessage>>, ()>{
    let chat_id = query!(
        "SELECT id FROM chats WHERE name = ?", chatname).
        fetch_one(&pool).await.unwrap().id;
    let json_message_history = query!(
        "SELECT message_history FROM chat_history_cache WHERE chat_id = ?", chat_id).
        fetch_one(&pool).await.unwrap().message_history;
    if let Some(json_string) = json_message_history {
        let  messages: Vec<ChatHistoryMessage> = serde_json::from_str(&json_string).unwrap();
        return Ok(Json(messages));
    } else{
        return Err(());
    }
}
/// Queues incoming messages from users; Messages are added to priority queue (by time created) in sql database and processed by background threads
/// # Query format:
/// curl -X POST \ -H "Content-Type: application/json" \ -d '{"content": "Message here :)"}' \ 'http://127.0.0.1:3000/newmessage/chatname/ChatName/username/UsernameString'
async fn incoming_message(
    Path((chatname, username)):Path<(String,String)>,
    State(pool): State<SqlitePool>,
    Json(msg): Json<Message>,
) -> Json<Result<(), ()>> {
    println!("New message from {} in chat {}: {}", username, chatname, msg.content);
    let chat_id = query!("SELECT id FROM chats WHERE name = ?", chatname)
        .fetch_one(&pool)
        .await
        .unwrap().id;
    let user_id = query!("SELECT id FROM users WHERE username = ?", username)
            .fetch_one(&pool)
            .await.unwrap().id;     
    let status = String::from("Processing");
    let result = query!(
        "INSERT INTO messages (chat_id, user_id, content, created_at, status) VALUES (?, ?, ?, datetime('now'), ?)",
        chat_id,
        user_id,
        msg.content, 
        status
    ).execute(&pool).await.unwrap();
    println!("Processing");
    let message_id = result.last_insert_rowid();
    let status = String::from("Queued");
    let direction: String = String::from("inbound"); // Messages going to server for processing
    query!(
        "INSERT INTO message_queue (message_id, direction, queued_at, processed_at, status) VALUES (?, ?, datetime('now'), NULL, ?)",
        message_id,
        direction, 
        status
    ).execute(&pool).await.unwrap();
    println!("Queued!");
    Json(Ok(()))
}
/// Creates new chat; Chats are connected to users through bipartite graph, one side being the chats the other being the users
/// # Query format:
/// curl "http://127.0.0.1:3000/createchat?name=ChatName&user=username1&user=username2&user=username3..."
async fn new_chat(State(pool): State<SqlitePool>,
Query(params): Query<CreateChatParams>) -> Json<Result<(), ()>>{
    let chat_name = &params.name;
    let users = &params.user;
    for user in users{
        println!("{}", user);
    }
    query!(
        r#"INSERT INTO chats (name, created_at)
        VALUES (?, datetime('now'))"#, chat_name
        ).execute(&pool).await.unwrap();
    let chat_id = query!("SELECT id FROM chats WHERE name = ?", chat_name)
        .fetch_one(&pool)
        .await
        .unwrap().id;
    let chat_history: Vec<ChatHistoryMessage> = vec![
        ChatHistoryMessage{
            username: "Hello World!".to_string(),
            content: "Welcome!".to_string(),
            created_at: "0000-00-00 00:00:00".to_string(),
        }
    ];
    let json_history = serde_json::to_string(&chat_history).unwrap();
    query!(
        r#"INSERT INTO chat_history_cache (chat_id, message_history, updated_at)
        VALUES (?, ?, datetime('now'))"#, chat_id, json_history
        ).execute(&pool).await.unwrap();
    for user in users{
        let user_id = query!("SELECT id FROM users WHERE username = ?", user)
            .fetch_one(&pool)
            .await.unwrap().id;        
        query!(
            r#"INSERT INTO chat_users (chat_id, user_id, is_active, joined_at)
            VALUES (?, ?, 1, datetime('now'))"#, chat_id, user_id
            ).execute(&pool).await.unwrap();
    }
    return Json(Ok(()));
}
/// Creates new user; 
/// # Query format:
/// curl "http://127.0.0.1:3000/createaccount/username/NameString/password/PasswordString"  
async fn new_user(State(pool): State<SqlitePool>, Path((username, password)):Path<(String,String)>) -> Json<Result<(), ()>>{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
    let role: String = String::from("chatter");
    query!(
    r#"INSERT INTO users (username, password, role, created_at)
    VALUES (?, ?, ?, datetime('now'))"#, username, password_hash, role
    ).execute(&pool).await.unwrap();
    Json(Ok(()))
}
/// Authenticates user login
/// # Query format:
/// curl "http://127.0.0.1:3000/Authenticate/username/NameString/password/PasswordString" 
async fn login(State(pool): State<SqlitePool>, Path((username, password)): Path<(String,String)>) -> Json<Result<String, String>>{
    
    let row = sqlx::query!(
        "SELECT password FROM users WHERE username = ?",
        username
    ).fetch_optional(&pool) // returns Option
    .await.unwrap(); 
    if let Some(row) = row.as_ref() {
        match PasswordHash::new(&row.password) {
            Ok(parsed_hash) => {
                if Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok()
                {
                    return Json(Ok(String::from("Login Success")));
                } else {
                    return Json(Ok(String::from("Login error")));
                }
            }
            Err(_) => {
                // The hash in the database is invalid
                println!("Stored password hash is invalid!");
                return Json(Ok(String::from("Login error")));
            }
        }
    } else {
        println!("Username not found");
        return Json(Ok(String::from("Login error")));
    }
}
