// src/components/ChatWindow.tsx
// Your original styling — restored pixel-perfect

import React, { useEffect, useRef } from "react";

type Message = {
  username: string;
  content: string;
  created_at: string;
};

type ChatWindowProps = {
  messages: Message[];
  currentUser: string;
};

const ChatWindow: React.FC<ChatWindowProps> = ({ messages, currentUser }) => {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const prevLengthRef = useRef<number>(messages.length);

  useEffect(() => {
    const el = containerRef.current;
    if (!el) return;

    if (messages.length > prevLengthRef.current) {
      el.scrollTo({ top: el.scrollHeight, behavior: "smooth" });
    }

    prevLengthRef.current = messages.length;
  }, [messages]);

  return (
    <div
      ref={containerRef}
      className="flex-grow-1 bg-light p-3"
      style={{ height: "100vh", overflowY: "auto" }}
    >
      {messages.length === 0 ? (
        <div className="p-3 text-muted">No messages yet. Say hello!</div>
      ) : (
        messages.map((msg, idx) => (
          <div
            key={idx}
            className={`p-2 mb-2 rounded-lg max-w-md w-fit ${
              msg.username === currentUser
                ? "bg-sky-300 text-white ml-auto"
                : "bg-orange-400 text-black mr-auto"
            }`}
            style={{ marginBottom: "1em" }}
          >
            <div>
              <strong>{msg.username}:</strong> {msg.content}
            </div>
            <div style={{ fontSize: "0.8em", color: "#888" }}>
              {new Date(msg.created_at).toLocaleTimeString()}
            </div>
          </div>
        ))
      )}
    </div>
  );
};

export default ChatWindow;
