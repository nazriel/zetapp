import { bytesHumanReadable, timeHumanReadable } from "../utils.ts";
import ConnectionStatus from "../components/ConnectionStatus.tsx";
import ConnectionSignal from "../components/ConnectionSignal.tsx";
import NoConnection from "../components/NoConnection.tsx";
import ErrorHandler from "../components/ErrorHandler.tsx";


import store from "../store/store.ts";
import { Show } from "solid-js";
import { useNavigate } from "@solidjs/router";

function Home() {
  const monthlyUsage = () => {
    return Math.floor(store.limits.down / store.limits.limit * 100);
  };

  const navigate = useNavigate();
  if (store.settings.defaults) {
    navigate("/settings", {
      replace: true
    });
  }

  return (
    <>
      <div class="pb-4 mb-4 border-solid border-b border-base-content border-opacity-10">
        <h2 class="text-xl text-center">Statistics</h2>
      </div>

      <Show when={!store.error} fallback={<ErrorHandler error={store.error!} />}>
        <Show when={store.connected} fallback={<NoConnection />}>

          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text">Network provider</span>
              {store.connection.provider}
            </label>
          </div>
          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text">Signal</span>
              <ConnectionSignal signal={store.connection.signal} />
            </label>
          </div>
          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text">Status</span>
              <ConnectionStatus status={store.connection.status} />
            </label>
          </div>
          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text">Mode</span>
              {store.connection.mode}
            </label>
          </div>
          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text">Current speed</span>
              {"↓ " + bytesHumanReadable(store.session.currDown) + "/s ↑ " + bytesHumanReadable(store.session.currUp) + "/s"}
            </label>
          </div>
          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text">Session transfer</span>
              {"↓ " + bytesHumanReadable(store.session.totalRx) + " ↑ " + bytesHumanReadable(store.session.totalTx)}
            </label>
          </div>
          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text">Session time</span>
              {timeHumanReadable(store.session.time)}
            </label>
          </div>
          <div class="form-control">
            <label class="label cursor-pointer">
              <span class="label-text">Monthly limits</span>
              {bytesHumanReadable(store.limits.down) + " / " + bytesHumanReadable(store.limits.limit)} ({monthlyUsage()}%)
            </label>
          </div>
        </Show>
      </Show>
    </>
  );
}

export default Home;
