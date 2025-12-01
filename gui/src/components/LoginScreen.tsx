import { invoke } from "@tauri-apps/api/core"; // 

type Props = {
    onLoginSuccess: (username: string) => void;
};

function LoginScreen({ onLoginSuccess }: Props) {

    async function tryLogin() {
        const username = (document.getElementById("user") as HTMLInputElement).value.trim();
        const password = (document.getElementById("pass") as HTMLInputElement).value;
        try {
            const result = await invoke<string>("login", { username, password });
            console.log("Login success:", result);
            onLoginSuccess(username);

        } catch (error: any) {
            console.error("Login failed:", error);
            alert("Wrong username or password");
        }
    }

     async function tryRegister() {
    const username = (document.getElementById("user") as HTMLInputElement).value.trim();
    const password = (document.getElementById("pass") as HTMLInputElement).value;
    try {
      await invoke<string>("register", { username, password }); // call your register command here
      // optionally auto‑login after registration:
      onLoginSuccess(username);
    } catch (error: any) {
      alert("Could not register user");
    }
  }

    return (
        <div className="login-container">
            <h1>Chat Server</h1>
            <p>Enter username and password</p>

            <input id="user" className="login-input" placeholder="Username" />
            <input id="pass" className="login-input" type="password" placeholder="Password" />

            <button onClick={tryLogin} className="login-button">
                Log In
            </button>

            <button onClick={tryRegister} className="register-button">
                Register
            </button>
        </div>
    );

}

export default LoginScreen;  
