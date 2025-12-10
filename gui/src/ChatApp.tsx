// ChatApp.tsx ← Use ONLY this version

import "./App.css";
import Sidebar from "./components/Sidebar";
import ChatWindow from "./components/ChatWindow";
import Messagebar from "./components/Messagebar";
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "bootstrap/dist/css/bootstrap.min.css";

type ChatInfo = {
  id: string;
  users: string[];
};

function ChatApp({ currentUser }: { currentUser: string }) {
  const [activeChat, setActiveChat] = useState<string | null>(null);
  const [chats, setChats] = useState<ChatInfo[]>([]);
  const [messages, setMessages] = useState<any[]>([]);

  const refreshChats = async () => {
    try {
      const list = await invoke<ChatInfo[]>("list_chats", { user: currentUser });
      setChats(list);
    } catch (e) {
      console.error("Failed to load chats", e);
    }
  };

  useEffect(() => {
    refreshChats();
    const id = setInterval(refreshChats, 1000);
    return () => clearInterval(id);
  }, [currentUser]);

  // Load messages + auto-refresh
  useEffect(() => {
    if (!activeChat) {
      setMessages([]);
      return;
    }

    const load = async () => {
      try {
        const msgs = await invoke<any[]>("print_messages", { id: activeChat });
        setMessages(msgs); // :3 can u run again and lmk what happens 
      } catch (e) {
        console.error("Failed to load messages", e); // wait lm try something YAYAYAYAYAYAYAYAYAAYYAYAYA check discord
      }
    };

    load();
    const id = setInterval(load, 3000);
    return () => clearInterval(id);
  }, [activeChat]);

  const handleNewChat = async (username: string) => { // is it working, I change a little in the rust function
    if (!username || username === currentUser) return;

    const result = await invoke<number>("handleNewChat", {
      currentUser,
      user: username,
    });

    if (result === 1) {
      await refreshChats();
      const users = [currentUser, username].sort();
      const chat_name = users.join("");
      setActiveChat(chat_name);
    } else {
      alert("User does not exist");
    }
  };

  const handleSend = async (text: string) => {
    if (!activeChat || !text.trim()) return;
    console.log("Handling new message")
    await invoke("log_message", {
      id: activeChat,
      username: currentUser,
      message: text,
    });
    /// bruh thats why I pyut the console.log :| check discord i dont see
    // did not work i dont think  in inspect? it didnt log?oh i didnt even look LOL i though it was gonna be in treminal i didt read ops :3
    const updated = await invoke<any[]>("print_messages", { id: activeChat }); // lemme check tho i just closed it for a sec

    setMessages(updated);// 
  };

  return (
    <div className="d-flex vh-100">
      <Sidebar
        chats={chats}
        activeChat={activeChat}
        handleChatClick={setActiveChat}
        onNewChat={handleNewChat}
        currentUser={currentUser}
        onNewChatDone={refreshChats}
      />
      <div className="flex-grow-1 d-flex flex-column">
        <ChatWindow messages={messages} currentUser={currentUser} />
        <Messagebar onSend={handleSend} activeChat={activeChat} />
      </div>
    </div>
  );
}

export default ChatApp;