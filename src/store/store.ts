import { createStore } from "solid-js/store";
import { invoke } from "@tauri-apps/api/core";

import Data, { Connection } from "./data";

const [store, updateStore] = createStore(new Data());

let lastFetch = Date.now();
async function fetchStats() {
  let stats: any = {};
  try {
    stats = await invoke("stats");
  }
  catch (e) {
    updateStore("connected", false);
    updateStore("error", e as string);
    console.log("failed to fetch data", e);
    return;
  }

  lastFetch = Date.now();
  updateStore("error", null);
  updateStore("connection", (conn: Connection) => ({
    ...conn,
    status: stats.status,
    mode: stats.mode,
    provider: stats.provider,
    signal: stats.signal,
  }));

  updateStore("session", (sess: any) => ({
    ...sess,
    time: stats.time,
    totalRx: stats.total_rx,
    totalTx: stats.total_tx,
    currDown: stats.current_rx,
    currUp: stats.current_tx
  }));

  updateStore("limits", (limit: any) => ({
    ...limit,
    down: stats.month_rx,
    up: stats.month_tx,
    limit: (1024 * 1024 * 1024 * 1024) + 1, // TODO: fetch from API somehow
  }));
  updateStore("connected", stats.online);
}

async function fetchConfig() {
  const settingsInfo: any = await invoke("get_settings", {});
  console.log("settings", settingsInfo);
  updateStore("settings", (settings: any) => ({
    ...settings,
    passwordType: settingsInfo.password_type,
    deviceIp: settingsInfo.device_ip,
    deviceModel: settingsInfo.device_model,
    password: settingsInfo.password,
    defaults: settingsInfo.defaults
  }))
}

// function loop() {
// interval = setTimeout(async () => {
//     try {
//         await fetchStats();
//         clearTimeout(interval);
//         loop();
//     }
//     catch (e) {
//         clearTimeout(interval);
//         console.log("Failed to fetch data - will wait extra 5 seconds", e);
//         await new Promise(r => setTimeout(r, 5000));
//         loop();
//     }
// }, 1000);
// }

let interval: any = null;
export function init() {
  fetchConfig();
  fetchStats();
  interval = setInterval(async () => await fetchStats(), 1000);
}

export function destroy() {
  clearTimeout(interval);
  updateStore("connected", false);
  // todo invoke logout
}

export default store;
