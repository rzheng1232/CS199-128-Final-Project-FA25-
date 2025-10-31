# CS199-128-Final-Project-FA25-

## Group Name

Ryan Mia Len

## Names and Net IDs

Ryan - ryanmz2

Mia - mmarg

Len - llivs

## Project Introduction

We are working on a live chat server that encrypts and stores messages. We want to create a server that will allow users to send messages back and forth and view message history. The server will have also have message security by encrypting and decrypting messages on user devices, so that the central server will not be able to see the plaintext of the messages, and will only be able to see encrypted versions of the messages. The server will also be responsible for message routing, encryption management, and database storage, while each client application will handle message encryption/decryption and local caching. We plan to use tokio for asynchronous networking, ring for encryption, and sqlx for database interaction. We chose to work on this project because we would learn about networking, servers, and encryption while using our Rust skills to make the final product memory safe and efficient.

## Technical Overview:
 - Centralized Server hosted on Raspberry Pi 5 with Tauri
 - Distributed client side application with UI
 - Authentication: Password based user authentication 
 - Local caching using JSON
 - SQL database for chat history storage

## Roadmap:
1. Create a centralized server 
2. Learn how to differentiate between different users - tracking their unique address and using passwords/usernames when signing on to the server
3. Make sure the server handles people simultaneously using threads and asynchronous networking
4. Nake sure the messsages are encrypted and password/username protected
5. Cache local history using .json files and store them on the server with a database 
6. Set up a GUI using Tauri
### Checkpoint Schedule:
Checkpoint 1: Set up the basics -- Have a encrypted server that users can chat with others off of, using local caches to store messages with. 

Checkpoint 2: Set up advanced things -- Centralize message storage on the server, add a GUI using Tauri, and add username/password functionality so that you can access your messages off of multiple devices 

## Challenges:
- We will need to connect clients with the server and ensure that all messages going between clients follow a set protocol so that they can be encrypted/decrypted and delivered smoothly
- We will have to set up authentication and encryption/decryption protocols so that the messages are private
- We will have to store encrypted messages on the server and deliver them to clients when they get online for them to see message history
- We have to do asynchronous networking in order to manage multiple users, including some being online and some being offline
- We have to ensure thread safety for our multiple different processes that will all be running asynchronously
- We will have to manage passwords to ensure that users are who they say they are so they can recieve message history
- We will also have to set up a basic GUI using Tauri, which involes HTML/CSS/JS for the frontend while we will stay completely in Rust for the backend

## References:
This project is inspired by a lot of different chat apps and websites, but with a focus on encryption and message history that can be transferred between devices.