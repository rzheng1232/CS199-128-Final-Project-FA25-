import "./App.css";
import Sidebar from "./components/Sidebar";
import ChatWindow from "./components/ChatWindow";
import Messagebar from "./components/Messagebar";
import { Chat } from "./types";
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "bootstrap/dist/css/bootstrap.min.css";

type Props = {
  currentUser: String;
}

function ChatApp({ currentUser }: Props) {
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
        user: currentUser,
        message: text,
      });

      const updatedChats = await invoke<Chat[]>("print_messages", { path: null });
      setChats(updatedChats);

    } catch (error) {
      console.error("Failed to send:", error);
      alert("Error sending message: " + error);
    }
  };

  const handleNewChat = async (username: string) => {
  if (!username) return;

  try {
    setChats((prev) => {
      if (prev.some((c) => c.name === username)) return prev;
      return [...prev, { name: username, messages: [] }];
    });

    setActiveChat(username);

    await invoke("new_chat", {
      chatName: username,
    });

    const updatedChats = await invoke<Chat[]>("print_messages", { path: null });
    console.log("Updated chats after new chat:", updatedChats);
    setChats(updatedChats);
  } catch (error) {
    console.error("Failed to create new chat:", error);
    alert("Error creating new chat: " + error);
  }
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
export default ChatApp;
