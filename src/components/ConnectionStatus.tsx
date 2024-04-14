import { createEffect, createSignal } from "solid-js";

function ConnectionStatus(props: any) {
  const [statusText, setStatusText] = createSignal("Disconnected");
  const [statusColor, setStatusColor] = createSignal("bg-red-500");

  createEffect(() => {
    if (props.status === "ppp_connected") {
      setStatusText("Connected");
      setStatusColor("bg-green-500");
    } else if (props.status === "ppp_connecting") {
      setStatusText("Connecting");
      setStatusColor("bg-yellow-500");
    } else if (props.status === "ppp_disconnecting") {
      setStatusText("Disconnecting");
      setStatusColor("bg-yellow-500");
    } else if (props.status === "ppp_disconnected") {
      setStatusText("Disconnected");
      setStatusColor("bg-red-500");
    }
  })
  return (
    <div>
      {statusText()} &nbsp;
      <span class={`inline-block w-3 h-3 rounded-full ${statusColor()}`}></span>
    </div>
  );
}

export default ConnectionStatus;
