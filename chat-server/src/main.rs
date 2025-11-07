use axum::{
    extract::Path, response::Json, routing::get, Extension, Router
};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};use sqlx::SqlitePool;
use std::sync::Arc;

// Things the central sever processor needs to handle:
//    User prescence: Whether a user is currently online or not
//    User authentication & account login
//    "Ground truth" chat history: keeps a central state of the chat history that users pull from when they login
//    Message queue: Incoming message requests are added to a queue processed and updated to everyone's chats one at a time to prevent conflict
//    Chatroom management: Json 

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    let pool = SqlitePool::connect("sqlite:server.db").await?;
    let pool = Arc::new(pool);
    let app = Router::new()
        .route("/", get(root))
        .route("/Authenticate/username/{name}/password/{pass}", get(login))
        .route("/createaccount/username/{name}/password/{pass}", get(new_user))
        .layer(Extension(pool.clone()));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
async fn root() -> Json<String>{
    println!("200");
    Json(String::from("Root!"))
}
async fn new_user(Path((username, password)):Path<(String,String)>, Extension(pool):Extension<Arc<SqlitePool>>) -> Json<Result<(), ()>>{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
    sqlx::query!(
    r#"INSERT INTO users (username, password_hash)
    VALUES (?, ?)"#, username, password_hash
    ).execute(&*pool).await.unwrap();
    Json(Ok(()))
}

async fn login(Path((username, password)): Path<(String,String)>,  Extension(pool):Extension<Arc<SqlitePool>>) -> Json<Result<String, String>>{
    
    let row = sqlx::query!(
        "SELECT password_hash FROM users WHERE username = ?",
        username
    ).fetch_optional(&*pool) // returns Option
    .await.unwrap(); 
    if let Some(row) = row.as_ref() {
        match PasswordHash::new(&row.password_hash) {
            Ok(parsed_hash) => {
                if Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok()
                {
                    println!("Password correct!");
                    return Json(Ok(String::from("Login Success")));
                } else {
                    println!("Password incorrect!");
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
    // let password_hash = PasswordHash::new(&row.as_ref().unwrap().password_hash);
    // if Argon2::default().verify_password(password.as_bytes(), &password_hash.unwrap()).is_ok(){
    //     return Json(Ok(String::from("Login Successful")));
    // } else{
    //     return Json(Ok(String::from("Login error")));
    // }

    
}
