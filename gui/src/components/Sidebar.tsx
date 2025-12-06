import { JSX, useState } from "react";
import { Chat } from "../types";

type SideBarProps = {
  chats: Chat[];
  activeChat: string | null;
  handleChatClick: (chatName: string) => void;
  onNewChat: (username: string) => void;
};

function SideBar({
  chats,
  activeChat,
  handleChatClick,
  onNewChat,
}: SideBarProps): JSX.Element {
  const [username, setUsername] = useState("");

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setUsername(e.target.value);
  };

  const handleNewChat = (e: React.FormEvent) => {
    e.preventDefault();
    if (username.trim()) {
      // check if it exists
      
      onNewChat(username.trim());
      setUsername("");
    }
  };

  return (
    <div className="flex flex-col bg-slate-900 text-slate-100 p-4 h-screen w-64 overflow-y-auto ">
      <h1 className="text-lg font-semibold mb-3 ">Chats</h1>

      {chats.length === 0 && (
        <p className="text-sm text-slate-400 mb-3">Start your first chat!</p>
      )}

      <ul className="list-none p-0 m-0 space-y-1 ">
        {chats.map((chat) => (
          <li key={chat.name}>
            <button
              type="button"
              onClick={() => handleChatClick(chat.name)}
              style={{ borderRadius: '6px' }}
              className={
                activeChat === chat.name
                  ? "w-full text-left bg-indigo-600 text-white px-2 py-2 text-sm hover:bg-indigo-500"
                  : "w-full text-left text-slate-200 px-2 py-2 text-sm hover:bg-slate-600 hover:text-white"
              }
            >
              {chat.name}
            </button>
          </li>
        ))}
      </ul>

      <form onSubmit={handleNewChat} className="mt-auto space-y-2">
        <div className="flex flex-col gap-2">
          <input
            type="text"
            className="form-control"
            placeholder="Username"
            value={username}
            onChange={handleChange}
          />
          <button type="submit" className="btn btn-wide btn-primary">
            New chat
          </button>
        </div>
      </form>
    </div>
  );
}

export default SideBar;
