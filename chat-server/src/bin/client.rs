use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize)]
struct Message {
    content: String,
}

#[derive(Deserialize)]
struct ChatHistoryMessage {
    username: String,
    content: String,
    created_at: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let base = "http://127.0.0.1:3000";

    loop {
        let options = vec![
            "Login",
            "Create Account",
            "Send Message",
            "Get Chat History",
            "Create Chat",
            "Quit",
        ];

        let selection = Select::new()
            .with_prompt("Choose an action")
            .items(&options)
            .interact()
            .unwrap();

        match selection {
            0 => {
                let username: String = Input::new().with_prompt("Username").interact().unwrap();
                let password: String = Input::new().with_prompt("Password").interact().unwrap();

                let url = format!("{}/Authenticate/username/{}/password/{}", base, username, password);
                let res = client.get(url).send().await?;

                println!("Response: {:?}", res.text().await?);
            }

            1 => {
                let username: String = Input::new().with_prompt("New Username").interact().unwrap();
                let password: String = Input::new().with_prompt("New Password").interact().unwrap();

                let url = format!("{}/createaccount/username/{}/password/{}", base, username, password);
                let res = client.get(url).send().await?;

                println!("Response: {:?}", res.text().await?);
            }

            2 => {
                let chat: String = Input::new().with_prompt("Chat Name").interact().unwrap();
                let username: String = Input::new().with_prompt("Your Username").interact().unwrap();
                let content: String = Input::new().with_prompt("Message").interact().unwrap();

                let msg = Message { content };
                let url = format!("{}/newmessage/chatname/{}/username/{}", base, chat, username);

                let res = client.post(url).json(&msg).send().await?;
                println!("Response: {:?}", res.text().await?);
            }

            3 => {
                let chat: String = Input::new().with_prompt("Chat Name").interact().unwrap();

                let url = format!("{}/getchat/chatname/{}", base, chat);
                let res = client.get(url).send().await?;

                match res.json::<Vec<ChatHistoryMessage>>().await {
                    Ok(history) => {
                        println!("\nChat History:");
                        for m in history {
                            println!("{} [{}]: {}", m.username, m.created_at, m.content);
                        }
                    }
                    Err(_) => println!("No chat history or error occurred"),
                }
            }

            4 => {
                let chat: String = Input::new().with_prompt("Chat Name").interact().unwrap();
                let users_str: String = Input::new()
                    .with_prompt("Users (comma-separated)")
                    .interact()
                    .unwrap();

                let mut url = format!("{}/createchat?name={}", base, chat);

                for user in users_str.split(',') {
                    url.push_str(&format!("&user={}", user.trim()));
                }

                let res = client.get(url).send().await?;
                println!("Response: {:?}", res.text().await?);
            }

            5 => {
                println!("Goodbye!");
                break;
            }

            _ => unreachable!(),
        }
    }

    Ok(())
}