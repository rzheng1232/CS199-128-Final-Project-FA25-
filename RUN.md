# Run Instructions

## Downloading the App

Download the appropriate app our OS for yand run it:

- **macOS**: Since this build is not signed or notarized, there are two options to run the app. Option 2 will probably be easier for most users but requires installing the source code.
  - **Option 1** - Allow the downloaded app to run
    - [Download `.dmg`](https://github.com/rzheng1232/CS199-128-Final-Project-FA25-/releases/latest/download/Illini.Chat_0.1.0_aarch64.dmg) and drag it into the Applications folder
    - When you run the app, macOS will show you a warning.
    - Navigate to Settings -> Privacy & Security
    - Click "Open Anyway" near the "Illini.Chat was blocked from use" warning
    - If this does not work, you can skip to Option 2 or use the terminal.
    - In the terminal, run the following commands:
      ```bash
      sudo spctl --global-disable
      sudo xattr -r -d com.apple.quarantine /Applications/Illini.Chat.app
      ```
    - Open Illini.Chat
    - In the terminal, run the following command to re-enable Gatekeeper:
      ```bash
      sudo spctl --global-enable
      ```
    - If this does not work, use Option 2
  - **Option 2** - Clone and build the app manually
    - Run the following commands in the terminal
      ```bash
      git clone https://github.com/rzheng1232/CS199-128-Final-Project-FA25-.git
      cd CS199-128-Final-Project-FA25-/gui
      npm ci
      cd CS199-128-Final-Project-FA25-/gui/src-tauri
      cargo add
      npm run tauri build
      ```
    - Drag the app into the Applications folder and run it
- **Windows**: [Download `.exe`](https://github.com/rzheng1232/CS199-128-Final-Project-FA25-/releases/latest/download/Illini.Chat_0.1.0_x64-setup.exe)  
- **Linux**:  
  - [Download `.AppImage`](https://github.com/rzheng1232/CS199-128-Final-Project-FA25-/releases/latest/download/Illini.Chat_0.1.0_amd64.AppImage)  
  - [Download `.deb`](https://github.com/rzheng1232/CS199-128-Final-Project-FA25-/releases/latest/download/Illini.Chat_0.1.0_amd64.deb)  

---

## Creating an Account
 
1. Create an account with a username and password.
2. Press the **Register** button. 
3. If you already have an account, you can switch to the **Login** screen, enter your username and password, and press **Login**.

---

## Sending a Message

1. Create a chat by typing someone else’s username.  
2. Send messages to that user and read incoming messages.  
3. Feel free to send as many chats as you like!
