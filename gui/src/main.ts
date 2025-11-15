import { invoke } from "@tauri-apps/api";

async function testInvoke() {
  const result = await invoke("greet", { name: "Leonard" });
  console.log(result);
}

testInvoke();