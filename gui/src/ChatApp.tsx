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

interface Props {  // or ChatWindowProps - whatever you called it
  messages: Message[];
  currentUser: string;
}


type ChatInfo = {
  id: string;
  users: string[];
}

function ChatApp({ currentUser }: Props) {
  const [activeChat, setActiveChat] = useState<string | null>(null); // "alice_bob"
  const [chats, setChats] = useState<ChatInfo[]>([]); // List of chat IDs
  const [messages, setMessages] = useState<Message[]>([]); // Active chat messages

  // Load all chats on startup
  useEffect(() => {
    invoke<string[]>("list_chats", {})
      .then((chatIds) => {
        console.log("Chats loaded:", chatIds);
        setChats(chatIds.map(id => ({ id, users: id.split('_') })));
      })
      .catch((err) => {
        console.log("No chats yet - Tauri command not ready");  // ← Graceful fail
        setChats([]);  // ← Set empty array so UI doesn't break
      });
  }, []);

  // Load messages when activeChat changes
  useEffect(() => {
    if (!activeChat) return;

    invoke<Message[]>("get_chat_messages", {
      users: activeChat.split('_').sort()
    })
      .then(setMessages)
      .catch(console.error);
  }, [activeChat]);

  const handleSend = async (text: string) => {
    if (!activeChat || !text.trim()) return;

    const chatUsers = activeChat.split('_').sort();

    try {
      await invoke("log_message", {
        users: chatUsers,
        user: currentUser as string,
        message: text.trim(),
      });

      // Reload messages for this chat
      const updatedMessages = await invoke<Message[]>("get_chat_messages", {
        users: chatUsers
      });
      setMessages(updatedMessages);
    } catch (error) {
      console.error("Failed to send:", error);
      alert("Error sending message: " + error);
    }
  };


  const handleNewChat = async (username: string) => {
    if (!username || username === currentUser) return;

    const chatId = [currentUser, username].sort().join('_');

    try {
      // Create chat with both users
      await invoke("log_message", {
        users: [currentUser as string, username],
        user: currentUser as string,
        message: ``,
      });

      setActiveChat(chatId);
      // Refresh chats list
      const updatedChats = await invoke<string[]>("list_chats", {});
      setChats(updatedChats.map(id => ({ id, users: id.split('_') })));
    } catch (error) {
      console.error("Failed to create new chat:", error);
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
      />
      <div className="flex-grow-1 d-flex flex-column">
        <ChatWindow          // ← Fixed typo
          messages={messages}
          currentUser={currentUser}
        />
        <Messagebar onSend={handleSend} activeChat={activeChat} />
      </div>
    </div>
  );


}
export default ChatApp;
