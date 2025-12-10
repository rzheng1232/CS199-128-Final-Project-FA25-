import { invoke } from "@tauri-apps/api/core";
import { JSX, useState } from "react";

// KEEP & FIX THIS:
type SideBarProps = {
  chats: ChatInfo[];        // ← CHANGE Chat[] to ChatInfo[]
  activeChat: string | null;
  handleChatClick: (chatName: string) => void;
  onNewChat: (username: string) => void;
  currentUser: string;
  onNewChatDone: () => void;
};

type ChatInfo = {
  id: string;     // "alice_bob"
  users: string[]; // ["alice", "bob"]
};

function SideBar({
  chats,
  activeChat,
  handleChatClick,
  onNewChat,
  currentUser,
  onNewChatDone
}: SideBarProps): JSX.Element {
  const [username, setUsername] = useState("");

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setUsername(e.target.value);
  };

  const handleNewChat = async (e: React.FormEvent) => {
    e.preventDefault();
    const trimmedUsername = username.trim();
    const result = await invoke<number>("handleNewChat", { currentUser: currentUser, user: trimmedUsername });
    console.log(result);

    if (result === 1) {
      // onNewChat(trimmedUsername);
      await onNewChatDone();
      setUsername("");
    } else {
      alert("User does not exist");
    }



    /*
    if (!trimmedUsername) return;

    try {
      const response = await fetch(`http://44.192.82.24/checkuser/username/${encodeURIComponent(trimmedUsername)}`);
      const result = await response.json();

      if (result.Ok === true) {
        onNewChat(trimmedUsername);
        setUsername("");
      } else {
        alert(`User "${trimmedUsername}" does not exist.`);
      }
    } catch (error) {
      console.warn("Server check failed, creating chat anyway for testing", error);
      onNewChat(trimmedUsername);
      setUsername("");
    } */
  };



  return (
    <div className="flex flex-col bg-slate-900 text-slate-100 p-4 h-screen w-64 overflow-y-auto ">
      <h1 className="text-lg font-semibold mb-3 ">Chats</h1>

      {chats.length === 0 && (
        <p className="text-sm text-slate-400 mb-3">Start your first chat!</p>
      )}

      <ul className="list-none p-0 m-0 space-y-1">
        {chats.map((chat) => {
          const others = (chat.users || []).filter(u => u !== currentUser);
          const label = others.join(", ");

          return (
            <li key={chat.id}>
              <button
                type="button"
                onClick={() => handleChatClick(chat.id)}
                style={{ borderRadius: "6px" }}
                className={
                  activeChat === chat.id
                    ? "w-full text-left bg-indigo-600 text-white px-2 py-2 text-sm hover:bg-indigo-500"
                    : "w-full text-left text-slate-200 px-2 py-2 text-sm hover:bg-slate-600 hover:text-white"
                }
              >
                {label || chat.id}
              </button>
            </li>
          );
        })}
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
