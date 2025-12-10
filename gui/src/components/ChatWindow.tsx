import { JSX } from "react";

type Message = {
  user: string;
  message: string;
  timestamp: string;
};

type ChatWindowProps = { 
  messages: Message[];
  currentUser: string;
};

const ChatWindow: React.FC<ChatWindowProps> = ({ messages, currentUser }) => { 
  return (
    <div
      className="flex-grow-1 bg-light p-3"
      style={{ height: "100vh", overflowY: "auto" }}
    >
      <h1 className="p3 border-bottom">Selected Chat:</h1>  {/* ← Fixed: no chat.name */}
      {messages.length === 0 ? (
        <div className="p-3 text-muted">No messages yet. Say hello!</div>
      ) : (
        messages.map((msg, idx) => (
          <div
            key={idx}
            className={`p-2 mb-2 rounded-lg max-w-md w-fit ${msg.user === currentUser
                ? "bg-sky-300 text-white ml-auto"      // YOU = light blue, aligned right
                : "bg-orange-400 text-black mr-auto"   // THEM = orange, aligned left
              }`}
            style={{ marginBottom: "1em" }}
          >
            <div>
              <strong>{msg.user}:</strong> {msg.message}
            </div>
            <div style={{ fontSize: "0.8em", color: "#888" }}>
              {new Date(msg.timestamp).toLocaleTimeString()}
            </div>
          </div>
        ))
      )}
    </div>
  );
};

export default ChatWindow;
