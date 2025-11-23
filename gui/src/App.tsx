
import { useState } from "react";
import LoginScreen from "./components/LoginScreen";   // your login component
import ChatApp from "./ChatApp";           // chat

function App() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);

  if (!isLoggedIn) {
    return <LoginScreen onLoginSuccess={() => setIsLoggedIn(true)} />;
  }

  return <ChatApp />;
}

export default App;