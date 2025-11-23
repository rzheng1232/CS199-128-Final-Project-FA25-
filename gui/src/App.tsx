import "./App.css";
import Sidebar from "./components/Sidebar";
import ChatWindow from "./components/ChatWindow";
import Messagebar from "./components/Messagebar";
import { Message, Chat } from "./types";
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "bootstrap/dist/css/bootstrap.min.css";

function App() {
  const [activeChat, setActiveChat] = useState<string | null>(null);

  const [chats, setChats] = useState<Chat[]>([]);

  useEffect(() => {
    invoke<Chat[]>("print_messages", { path: null })
      .then((data) => {
        console.log("Chats loaded:", data);
        setChats(data);
      })
      .catch((err) => {
        console.error("Failed to load chats:", err);
      });
  }, []);

  const handleClick = (chatName: string) => {
    console.log(`Clicked on ${chatName}`);
    setActiveChat(chatName);
  };

  const handleSend = async (text: string) => {
    if (!activeChat) return;

    try {
      await invoke("log_message", {
        chatName: activeChat,
        user: "Len",
        message: text,
      });

      const updatedChats = await invoke<Chat[]>("print_messages", {
        path: null,
      });
      setChats(updatedChats);
    } catch (error) {
      console.error("Failed to send:", error);
      alert("Error sending message: " + error);
    }
  };

  const handleNewChat = async (username: string) => {
    setActiveChat(username);
    invoke("log_message", {
      chatName: username,
      user: "Len",
      message: "",
    });
    invoke<Chat[]>("print_messages", { path: null });
  };

  return (
    <div className="d-flex vh-100">
      <Sidebar
        chats={chats}
        activeChat={activeChat}
        handleChatClick={handleClick}
        onNewChat={handleNewChat}
      />
      <div className="flex-grow-1 d-flex flex-column">
        <ChatWindow
          chat={chats.find((chat) => chat.name === activeChat) || null}
        />
        <Messagebar onSend={handleSend} activeChat={activeChat} />
      </div>
    </div>
  );
}
export default App;
