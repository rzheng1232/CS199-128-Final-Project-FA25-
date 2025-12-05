import React from "react";
import { invoke } from "@tauri-apps/api/core"; //
import LoginScreen from "./LoginScreen";

type Props = {
  onRegisterSuccess: (username: string) => void;
  onLoginPress: () => void;
};

function RegisterScreen({ onRegisterSuccess, onLoginPress }: Props) {
  async function tryRegister() {
    const username = (
      document.getElementById("user") as HTMLInputElement
    ).value.trim();
    const password = (document.getElementById("pass") as HTMLInputElement)
      .value;
    try {
      const result = await invoke<string>("register", { username, password });
      console.log("Register success:", result);
      onRegisterSuccess(username);
    } catch (error: any) {
      console.error("Register failed:", error);
      alert("Wrong username or password");
    }
  }

  return (
    <div className="min-h-screen bg-slate-950 flex items-center justify-center px-4">
      <div className="w-full max-w-sm rounded-2xl bg-slate-900/80 border border-slate-800 shadow-xl px-6 py-8">
        <h1 className="text-2xl font-semibold text-white text-center">
          Chat Server
        </h1>
        <p className="mt-2 text-sm text-slate-400 text-center">
          Register with username and password
        </p>

        <form onSubmit={tryRegister} className="mt-6 space-y-4">
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

          <button
            type="submit"
            className="mt-2 flex w-full justify-center rounded-md bg-indigo-500 px-3 py-2 text-sm font-semibold text-white hover:bg-indigo-400 active:bg-indigo-600 transition focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-900"
          >
            Register
          </button>
        </form>

        <p className="mt-6 text-center text-sm text-slate-400">
          Already have an account?{" "}
          <button
            type="button"
            onClick={() => {
              console.log("button has been pressed");
              onLoginPress();
            }}
            className="font-semibold text-indigo-400 hover:text-indigo-300"
          >
            Login
          </button>
        </p>
      </div>
    </div>
  );
}

export default RegisterScreen;
