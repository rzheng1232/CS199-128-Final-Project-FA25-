import "./App.css";
import Sidebar from "./components/Sidebar";
import ChatWindow from "./components/ChatWindow";
import Messagebar from "./components/Messagebar";
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "bootstrap/dist/css/bootstrap.min.css";

type Message = {
  user: string;
  message: string;
  timestamp: string;
};

type ChatAppProps = {
  // messages: Message[];
  currentUser: string;
}


type ChatInfo = {
  id: string;
  users: string[];
}

function ChatApp({ currentUser }: ChatAppProps) {
  const [activeChat, setActiveChat] = useState<string | null>(null);
  const [chats, setChats] = useState<ChatInfo[]>([]);
  const [messages, setMessages] = useState<Message[]>([]);

  // Load all chats on startup
  async function refreshChats() {
    try {
      const updatedChats = await invoke<ChatInfo[]>("list_chats", { user: currentUser });
      setChats(updatedChats);
    } catch (err) {
      console.error("Failed to refresh chats:", err);
    }
  }

  useEffect(() => {
    refreshChats();
    const intervalId = setInterval(refreshChats, 5000); // r4efresh every 5s
    return () => clearInterval(intervalId);
  }, [currentUser]);



  // Load messages when activeChat changes
  useEffect(() => {
    if (!activeChat) return;

    // Find the chat object from the chats array
    const current = chats.find(c => c.id === activeChat);
    if (!current) return;

    invoke<Message[]>("get_chat_messages", {
      users: current.users
    })
      .then(setMessages)
      .catch(console.error);
  }, [activeChat, chats]);

  const onNewChatDone = async () => {
    await refreshChats();
  }


  const handleNewChat = async (username: string) => {
    if (!username || username === currentUser) return;

    const chatId = currentUser + username;

    try {
      await invoke("new_chat", {
        name: chatId,
        user: [currentUser, username],
      });

      await refreshChats();
      setActiveChat(chatId);
    } catch (error) {
      console.error("Failed to create new chat:", error);
    }
  };



  const handleSend = async (text: string) => {
    if (!activeChat) return;

    try {
      await invoke("log_message", {
        users: activeChat.split("_"),
        user: currentUser,
        message: text,
      });

      const msgs = await invoke<Message[]>("get_chat_messages", {
        users: activeChat.split("_").sort(),
      });
      setMessages(msgs);

    } catch (err) {
      console.error("Failed to send message:", err);
    }
  };



  const handleClick = (chatId: string) => {
    console.log(`Clicked on ${chatId}`);
    setActiveChat(chatId);
  };


  return (
    <div className="d-flex vh-100">
      <Sidebar
        chats={chats}
        activeChat={activeChat}
        handleChatClick={handleClick}
        onNewChat={handleNewChat}
        currentUser={currentUser}
        onNewChatDone={onNewChatDone}
      />
      <div className="flex-grow-1 d-flex flex-column">
        <ChatWindow
          messages={messages}
          currentUser={currentUser}
        />
        <Messagebar onSend={handleSend} activeChat={activeChat} />
      </div>
    </div>
  );


}
export default ChatApp;
