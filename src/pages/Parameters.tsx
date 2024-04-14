import { Show, createSignal } from "solid-js";
import store from "../store/store";
import { changeConnection, changeMode } from "../store/actions";
import NoConnection from "../components/NoConnection";
import { useNavigate } from "@solidjs/router";
import ErrorHandler from "../components/ErrorHandler";

function Parameters() {
  const navigate = useNavigate();
  if (store.settings.defaults) {
    navigate("/settings", {
      replace: true
    });
  }

  let targetMode = store.connection.mode;
  let targetConnection = store.connection.status === "ppp_connected" ? true : false;
  let [processing, setProcessing] = createSignal(false);

  async function updateParams() {
    setProcessing(true);
    if (targetMode !== store.connection.mode) {
      await changeMode(targetMode);
      console.log("changed mode", targetMode);
    }
    if (targetConnection !== (store.connection.status === "ppp_connected")) {
      await changeConnection(targetConnection);
      console.log("changed connection", targetConnection);

      if (targetConnection === false) {
        // ugly hack but ZTE behaves weirdly when disconnecting
        await new Promise(r => setTimeout(r, 5000));
      }
    }
    await new Promise(r => setTimeout(r, 1000));
    setProcessing(false);
    navigate("/", {
      scroll: true
    })
  }

  return (
    <>
      <div class="pb-4 mb-4 border-solid border-b border-base-content border-opacity-10">
        <h2 class="text-xl text-center">Connection parameters</h2>
      </div>
      <Show when={!store.error} fallback={<ErrorHandler error={store.error!} />}>

        <Show when={store.connected || processing()} fallback={<NoConnection />}>

          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text">Network connected</span>
              <input type="checkbox" class="toggle" disabled={processing()} checked={store.connection.status === "ppp_connected"} onChange={(e) => targetConnection = e.target.checked} />
            </label>
          </div>
          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text">Connection Mode</span>
              <select class="select select-bordered w-min max-w-xs" onChange={(e) => targetMode = e.target.value} disabled={processing()}>
                <option selected={store.connection.mode === "5G"}>5G</option>
                <option selected={store.connection.mode === "LTE"}>LTE</option>
                <option disabled>3G</option>
              </select>
            </label>
          </div >
          <button class="btn w-full mt-4" onClick={updateParams} disabled={processing()}>
            <Show when={processing()} fallback={("Save")}>
              <span class="loading loading-dots loading-md"></span>
              Saving
            </Show>
          </button>
        </Show>
      </Show>
    </>
  )
}

export default Parameters;
