import { invoke } from "@tauri-apps/api/core";
import { destroy, init } from "./store";
import { Settings } from "./data";

export async function changeConnection(on: boolean) {
  await invoke("toggle_connection", {
    on: on
  });
}

export async function changeMode(mode: string) {
  await invoke("toggle_mode", {
    enable5g: mode === "5G",
  });
}

export async function updateSettings({ deviceIp, deviceModel, password }: Settings) {
  destroy();
  await new Promise(resolve => setTimeout(resolve, 100));
  await invoke("set_settings", {
    settings: {
      deviceIp,
      deviceModel,
      password
    }
  });
  await new Promise(resolve => setTimeout(resolve, 100));
  init();
  await new Promise(resolve => setTimeout(resolve, 1000));
}
