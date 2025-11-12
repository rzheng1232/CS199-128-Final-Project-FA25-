import "./App.css";
import Sidebar from "./components/Sidebar"
import ChatWindow from "./components/ChatWindow"
import Messagebar from "./components/Messagebar"

function App() {
  return (
  <div className="d-flex">
      <Sidebar />
      <div className="flex-grow-1 p-3">
        <ChatWindow />
      </div>
    </div>
    );
}

export default App;