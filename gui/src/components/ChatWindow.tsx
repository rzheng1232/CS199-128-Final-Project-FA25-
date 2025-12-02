import { JSX } from "react";
import { Chat } from "../types";

type ChatWindowProps = {
  chat: Chat | null;
};

function ChatWindow({ chat }: ChatWindowProps): JSX.Element {
  if (!chat) {
    return <div className="p-3">Please select a chat from the sidebar.</div>;
  }
  return (
    <div
      className="flex-grow-1 bg-light p-3 bg-light"
      style={{ height: "100vh", overflowY: "auto" }}
    >
      <h1 className="p3 border-bottom">Chat with {chat.name}</h1>
      {chat.messages.map((msg, idx) => (
        <div key={idx} style={{ marginBottom: "1em" }}>
          <div>
            <strong>{msg.user}:</strong>
            {msg.message}
          </div>
          <div style={{ fontSize: "0.8em", color: "#888" }}>
            {msg.timestamp}
          </div>
        </div>
      ))}
    </div>
  );
}

export default ChatWindow;
