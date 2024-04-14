import { Show, createSignal } from "solid-js";
import store from "../store/store";
import { updateSettings } from "../store/actions";
import { useNavigate } from "@solidjs/router";

function Settings() {
  const navigite = useNavigate();
  let password: string = "";
  let deviceIp: string = store.settings.deviceIp;
  let deviceModel: string = store.settings.deviceModel;

  let [processing, setProcessing] = createSignal(false);

  async function saveSettings() {
    setProcessing(true);
    await updateSettings({ password, deviceIp, deviceModel, defaults: false });
    setProcessing(false);
    navigite("/", {
      replace: true,
      scroll: true
    })
  }

  return (<>
    <div class="pb-4 mb-4 border-solid border-b border-base-content border-opacity-10">
      <h2 class="text-xl text-center">Application settings</h2>
    </div>
    <div class="form-control">
      <label class="label cursor-pointer">
        <span class="label-text">Device model</span>

        <select class="select select-bordered w-2/3" onChange={(e) => deviceModel = e.target.value} disabled={processing()}>
          <option selected={deviceModel === "mc889"} value="mc889">MC889</option>
          <option selected={deviceModel === "mf283"} value="mf283">MF283</option>
        </select>
      </label>
    </div>

    <div class="form-control">
      <label class="label cursor-pointer">
        <span class="label-text">Device IP</span>
        <input type="string" placeholder="IP" value={deviceIp} class="input input-bordered w-2/3" onChange={(e) => deviceIp = e.target.value} />
      </label>
    </div>

    <div class="form-control">
      <label class="label cursor-pointer">
        <span class="label-text">Password</span>
        <input type="password" value={password} class="input input-bordered w-2/3" onChange={(e) => password = e.target.value} />
      </label>
    </div>

    <button class="btn w-full mt-4" onClick={saveSettings} disabled={processing()}>
      <Show when={processing()} fallback={("Save")}>
        <span class="loading loading-dots loading-md"></span>
        Saving
      </Show>
    </button>
  </>)
}

export default Settings;
