import { JSX, MouseEvent, useState } from "react";
import { Chat } from "../types";

type SideBarProps = {
  chats: Chat[];
  activeChat: string | null;
  handleChatClick: (chatName: string) => void;
  handleNewChat: () => void;
};

function SideBar({ chats, activeChat, handleChatClick, handleNewChat }: SideBarProps): JSX.Element {
  return (
    <div
      className="d-flex flex-column bg-dark text-light p-3"
      style={{ height: "100vh", width: "250px", overflowY: "auto" }}
    >
      <h1>Chats</h1>
      {chats.length === 0 && <p>Start your first chat!</p>}
      {/* Inspires users to start a chat if there are none instead of just displaying nothing */}
      <ul className="nav flex-column">
        {chats.map((chat) => (
          <li key={chat.name}>
            <a
              href="#"
              onClick={() => handleChatClick(chat.name)}
              className={`nav-link ${
                activeChat === chat.name ? "bg-secondary text-white" : "text-light"
              }`}
            >
              {chat.name}
            </a>
          </li>
        ))}
      </ul>
      <button type="button" className="btn btn-primary" onClick = {handleNewChat}>
        New Chat
      </button>
    </div>
  );
}

export default SideBar;
