import { invoke } from "@tauri-apps/api/core";

async function testInvoke() {
  
  const result = await invoke("greet", { name: "Leonard" });
  console.log(result);
}

testInvoke();