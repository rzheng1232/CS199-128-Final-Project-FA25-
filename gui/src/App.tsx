
import { useState } from "react";
import LoginScreen from "./components/LoginScreen";   // your login component
import ChatApp from "./ChatApp";           // chat

function App() {
  const [currentUser, setCurrentUser] = useState<string | null>(null);

  if (!currentUser) {
    return <LoginScreen onLoginSuccess={setCurrentUser} />;
  }

  return <ChatApp currentUser={currentUser} />;
}

export default App;