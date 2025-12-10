
import { useState } from "react";
import LoginScreen from "./components/LoginScreen"; 
import RegisterScreen from "./components/RegisterScreen";
import ChatApp from "./ChatApp";           // chat

function App() {
  const [currentUser, setCurrentUser] = useState<string | null>(null);
  const [showRegister, setShowRegister] = useState(true);

  if (!currentUser) {
  if (showRegister) {
    return (
      <RegisterScreen
        onRegisterSuccess={(username: string) => {
          setCurrentUser(username);
        }}
        onLoginPress={() => {
          setShowRegister(false);
          console.log("Switched to Login");
        }}
      />
    );
  }

  return (
    <LoginScreen
      onLoginSuccess={(username: string) => {
        setCurrentUser(username);
      }}
      onRegisterPress={() => {
        setShowRegister(true);
        console.log("Switched to register");
      }}
    />
  );
}

  return <ChatApp currentUser={currentUser} />;
}

export default App;