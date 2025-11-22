import React, { JSX, useState } from "react";
import { Message, Chat } from "../types";

type MessageBarProps = {
  onSend: (text: string) => void;
  activeChat: string | null;
};

function Messagebar({ onSend, activeChat }: MessageBarProps): JSX.Element | null {
  const [text, setText] = useState("");

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setText(e.target.value);
  };

  const handleSend = (e: React.FormEvent) => {
    e.preventDefault();
    if (text.trim()) {
      onSend(text.trim());
      setText("");
    }
  };

  if(!activeChat) {
    return null;
  }

  return (
    <form className="d-flex p-3" style={{ gap: "0.5em" }} onSubmit={handleSend}>
      <input
        type="text"
        className="form-control"
        placeholder="Type a message..."
        value={text}
        onChange={handleChange}
      />
      <button type="submit" className="btn btn-primary">
        Send
      </button>
    </form>
  );
}

export default Messagebar;
