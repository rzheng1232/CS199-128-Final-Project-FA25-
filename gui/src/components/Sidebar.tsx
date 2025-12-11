import { invoke } from "@tauri-apps/api/core";
import { JSX, useState } from "react";

// KEEP & FIX THIS:
type SideBarProps = {
  chats: ChatInfo[];
  activeChat: string | null;
  handleChatClick: (chatName: string) => void;
  onNewChat: (username: string) => void;
  currentUser: string;
  onNewChatDone: () => void;
  onLogout: () => void;
};

type ChatInfo = {
  id: string; // "alice_bob"
  users: string[]; // ["alice", "bob"]
};

function SideBar({
  chats,
  activeChat,
  handleChatClick,
  //  onNewChat,
  currentUser,
  onNewChatDone,
  onLogout,
}: SideBarProps): JSX.Element {
  const [username, setUsername] = useState("");
  const [error, setError] = useState<string>("");

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setUsername(e.target.value);
    setError("");
  };

  const deleteChat = async (chatId: string, currentUser: string) => {
    try {
      const result = await invoke<number>("delete_chat", {
        user: currentUser,
        id: chatId  // ← Changed from chat_id to chatId
      });

      console.log("Delete result:", result);  // 0=fail, 1=success

      if (result === 1) {

      } else {
        alert("Could not delete chat. Try again.");
      }
    } catch (err) {
      console.error("Failed to delete chat:", err);
      alert("Could not delete chat. Try again.");
    }
  };


  const handleNewChat = async (e: React.FormEvent) => {
    e.preventDefault();
    const trimmedUsername = username.trim();

    if (!trimmedUsername) {
      setError("Please enter a username");
      return;
    }

    if (trimmedUsername === currentUser) {
      setError("You can't chat with yourself");
      return;
    }

    // Check if chat already exists (by users array)
    const alreadyExists = chats.some((chat) => {
      const users = chat.users || [];
      return (
        users.includes(currentUser) &&
        users.includes(trimmedUsername) &&
        users.length === 2
      );
    });

    if (alreadyExists) {
      // Switch to existing chat instead of creating new one
      const existingChat = chats.find((chat) => {
        const users = chat.users || [];
        return (
          users.includes(currentUser) &&
          users.includes(trimmedUsername) &&
          users.length === 2
        );
      });
      if (existingChat) {
        handleChatClick(existingChat.id);
      }
      setUsername("");
      return;
    }

    // Create new chat via backend
    try {
      const result = await invoke<number>("handleNewChat", {
        currentUser,
        user: trimmedUsername,
      });
      console.log("handleNewChat result:", result);

      if (result === 1) {
        await onNewChatDone();
        setUsername("");
      } else {
        setError("User does not exist");
      }
    } catch (err) {
      console.error("Failed to create chat:", err);
      setError("Failed to connect to server");
    }
  };

  return (
    <div className="flex flex-col bg-slate-900 text-slate-100 p-4 h-screen w-64 overflow-y-auto ">
      <div className="flex items-center gap-4 mb-3">
        <h1 className="text-lg font-semibold">Chats</h1>
        <button
          type="button"
          onClick={onLogout}
          className="px-3 py-1 text-xs font-semibold bg-red-600 rounded text-white hover:bg-red-500 active:bg-red-700 transition"
        >
          Logout
        </button>
      </div>
      <div className="flex-1 overflow-y-auto px-4">
        {chats.length === 0 && (
          <p className="text-sm text-slate-400 mb-3">Start your first chat!</p>
        )}

        <ul className="list-none p-0 m-0 space-y-1">
          {chats.map((chat) => {
            const others = (chat.users || []).filter((u) => u !== currentUser);
            const label = others.join(", ");

            return (
              <li key={chat.id} className="py-3">
                <div className="flex items-center gap-3">
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

                  <button
                    type="button"
                    onClick={(e) => {
                      e.stopPropagation();
                      deleteChat(currentUser, chat.id);
                      console.log(currentUser);
                      console.log(chat.id);
                    }}
                    className="inline-flex items-center justify-center h-7 w-7 shrink-0 p-0 bg-transparent-600 hover:bg-blue-500 text-white text-sm leading-none"
                  >
                    🗑️
                  </button>

                </div>
              </li>
            );
          })}
        </ul>
      </div>

      {error && (
        <div className="mt-3 mb-2 p-3 rounded-lg bg-red-900/60 border border-red-700 text-red-300 text-center text-sm font-medium">
          {error}
        </div>
      )}

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
