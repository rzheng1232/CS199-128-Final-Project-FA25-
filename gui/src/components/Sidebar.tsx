import { React, MouseEvent, useState } from "react";

function SideBar() {
  const [activeChat, setActiveChat] = useState<string | null>(null);
  let chats = ["Ryan", "Mia"];

  const handleClick = (chatName: string) => {
    console.log(`Clicked on ${chatName}`);
    setActiveChat(chatName);
  };

  return (
    <div
      className="d-flex flex-column bg-dark text-light p-3"
      style={{ height: "100vh", width: "250px" }}
    >
      <h1>Chats</h1>
      {chats.length === 0 && <p>Start your first chat!</p>}{" "}
      {/* Inspires users to start a chat if there are none instead of just displaying nothing */}
      <ul className="nav flex-column">
        {chats.map((chat) => (
          <li key={chat} onClick={() => handleClick(chat)} className="nav-item">
            <a
              href="#"
              className={
                `nav-link ${activeChat === chat ? "bg-secondary text-white" : "text-light"}`
              }
            >
              {chat}
            </a>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default SideBar;
