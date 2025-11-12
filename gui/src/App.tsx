import "./App.css";
import Sidebar from "./components/Sidebar";
import ChatWindow from "./components/ChatWindow";
import Messagebar from "./components/Messagebar";
import { Message, Chat } from "../types";
import { useState } from "react";

function App() {
  const [activeChat, setActiveChat] = useState<Chat | null>(null);

  const chats: Chat[] = [ // Temporary hard coded messages
    {
      name: "Ryan",
      messages: [
        {
          user: "Len",
          message: "Hi!",
          timestamp: "2025-11-11T03:12:45Z",
        },
        {
          user: "Ryan",
          message: "Hi!",
          timestamp: "2025-11-11T03:13:10Z",
        },
      ],
    },
    {
      name: "Mia",
      messages: [
        { user: "Len", message: "Hi!", timestamp: "2025-11-11T04:00:00Z" },
        { user: "Mia", message: "Hi!", timestamp: "2025-11-11T04:01:00Z" },
      ],
    },
  ];

  const handleClick = (chatName: string) => {
    console.log(`Clicked on ${chatName}`);
    setActiveChat(chatName);
  };

  const handleSend = async (text: string) => {};

  return (
    <div className="d-flex vh-100">
      <Sidebar
        chats={chats}
        activeChat={activeChat}
        handleChatClick={handleClick}
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
