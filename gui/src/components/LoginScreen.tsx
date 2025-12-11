
import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type Props = {
  onLoginSuccess: (username: string) => void;
  onRegisterPress: () => void;
};

function LoginScreen({ onLoginSuccess, onRegisterPress }: Props) {
  const [error, setError] = useState<string>("");
  async function tryLogin(e: React.FormEvent) {
    e.preventDefault();
    setError(""); // clear old error
    const username = (
      document.getElementById("user") as HTMLInputElement
    ).value.trim();
    const password = (document.getElementById("pass") as HTMLInputElement)
      .value;
    try {
      // login returns 0 or 1
      const result = await invoke<number>("login", { user: username, pass: password });
      console.log(result);

      if (result === 1) {
        onLoginSuccess(username);   // this is what flips you to ChatApp
      } else {
        setError("Wrong username or password");
      }
    } catch (error) {
      console.error("Login call failed:", error);
      alert("Login error (backend unreachable)");
    }



  }

  return (
    <div className="min-h-screen bg-slate-950 flex items-center justify-center px-4">
      <div className="w-full max-w-sm rounded-2xl bg-slate-900/80 border border-slate-800 shadow-xl px-6 py-8">
        <h1 className="text-2xl font-semibold text-white text-center">
          Illini.Chat
        </h1>
        <p className="mt-2 text-sm text-slate-400 text-center">
          Login with username and password
        </p>

        <form onSubmit={tryLogin} className="mt-6 space-y-4">
          <div>
            <label className="block text-sm font-medium text-slate-200">
              Username
            </label>
            <input
              id="user"
              className="mt-1 block w-full rounded-md bg-slate-800 border border-slate-700 px-3 py-2 text-sm text-slate-100 placeholder:text-slate-500 focus:outline-none focus:ring-2 focus:ring-indigo-500"
              placeholder="Username"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-slate-200">
              Password
            </label>
            <input
              id="pass"
              type="password"
              className="mt-1 block w-full rounded-md bg-slate-800 border border-slate-700 px-3 py-2 text-sm text-slate-100 placeholder:text-slate-500 focus:outline-none focus:ring-2 focus:ring-indigo-500"
              placeholder="Password"
            />
          </div>

          {error && (
            <div className="p-4 rounded-lg bg-red-500/50 border border-red-800 text-red-300 text-center text-sm font-medium">
              {error}
            </div>
          )}

          <button
            type="submit"
            className="mt-2 flex w-full justify-center rounded-md bg-indigo-500 px-3 py-2 text-sm font-semibold text-white hover:bg-indigo-400 active:bg-indigo-600 transition focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-900"
          >
            Log in
          </button>
        </form>

        <p className="mt-6 text-center text-sm text-slate-400">
          Don't have an account?{" "}
          <button
            type="button"
            onClick={() => {
              console.log("button has been pressed");
              onRegisterPress();
            }}
            className="font-semibold text-indigo-400 hover:text-indigo-300"
          >
            Register
          </button>
        </p>
      </div>
    </div>
  );
}

export default LoginScreen;
