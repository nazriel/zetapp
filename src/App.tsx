import { A } from "@solidjs/router";

import { init as initStore, destroy as destroyStore } from "./store/store";
import { onCleanup, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

function App(props: any) {

  async function rebuildStore() {
    destroyStore();
    await invoke("force_refresh", {});
    initStore();
  }

  onMount(() => {
    console.log("onMount")
    initStore()
  })

  onCleanup(() => {
    console.log("onUnmount")
    destroyStore()
  })

  return (
    <div class="flex flex-col max-h-screen landscape:md:max-h-80 landscape:lg:max-h-screen">
      <div class="navbar bg-base-100 p-4 pt-10" data-tauri-drag-region>
        <div class="navbar-start">
          <span class="w-6">
            <svg class="w-6 h-6 fill-base-content" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 470 470"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <g> <g> <g> <circle cx="85" cy="337" r="10"></circle> <circle cx="135" cy="337" r="10"></circle> <circle cx="185" cy="337" r="10"></circle> <circle cx="235" cy="337" r="10"></circle> <circle cx="285" cy="337" r="10"></circle> <circle cx="335" cy="337" r="10"></circle> <circle cx="385" cy="337" r="10"></circle> <path d="M462.5,279.5h-73.655l40.734-231.016c2.633-14.934-7.374-29.225-22.307-31.857c-14.93-2.64-29.224,7.374-31.858,22.307 l-38.221,216.765c-0.719,4.079,2.005,7.97,6.084,8.688c4.075,0.723,7.969-2.005,8.688-6.083l38.221-216.765 c1.197-6.789,7.692-11.336,14.481-10.141c6.788,1.197,11.336,7.693,10.139,14.48L373.613,279.5H222.5V42 c0-6.893,5.607-12.5,12.5-12.5s12.5,5.607,12.5,12.5v215c0,4.143,3.358,7.5,7.5,7.5c4.142,0,7.5-3.357,7.5-7.5V42 c0-15.163-12.336-27.5-27.5-27.5S207.5,26.837,207.5,42v237.5H96.387L55.193,45.879c-1.197-6.787,3.352-13.283,10.139-14.48 c6.792-1.194,13.285,3.353,14.481,10.141l38.221,216.765c0.719,4.079,4.611,6.807,8.688,6.083 c4.079-0.719,6.803-4.609,6.084-8.688L94.586,38.934c-2.633-14.933-16.923-24.945-31.858-22.307 c-14.933,2.633-24.94,16.924-22.307,31.857L81.155,279.5H7.5c-4.142,0-7.5,3.357-7.5,7.5v91c0,23.639,17.36,43.295,40,46.902V448 c0,4.143,3.358,7.5,7.5,7.5h70c4.142,0,7.5-3.357,7.5-7.5v-22.5h220V448c0,4.143,3.358,7.5,7.5,7.5h70c4.142,0,7.5-3.357,7.5-7.5 v-23.098c22.64-3.607,40-23.263,40-46.902v-91C470,282.857,466.642,279.5,462.5,279.5z M110,440.5H55v-15h55V440.5z M415,440.5 h-55v-15h55V440.5z M455,378c0,17.921-14.58,32.5-32.5,32.5h-375C29.58,410.5,15,395.921,15,378v-83.5h440V378z"></path> </g> </g> </g> </g></svg>
          </span>
          <h1 class="text-xl font-bold ml-2">ZTE Manager</h1>
        </div>
        <div class="navbar-end">
          <button class="btn btn-xs btn-ghost" onClick={rebuildStore}>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
              <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99" />
            </svg>
          </button>
        </div>
      </div>

      <div class="flex-grow overflow-y-auto p-6">
        {props.children}
      </div>

      <div class="btm-nav">
        <A activeClass="active" href="/" end={true}>

          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-7 h-7">
            <path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
          </svg>

        </A>
        <A activeClass="active" href="/params">

          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-7 h-7">
            <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 6h9.75M10.5 6a1.5 1.5 0 1 1-3 0m3 0a1.5 1.5 0 1 0-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-9.75 0h9.75" />
          </svg>

        </A>
        <A activeClass="active" href="/settings">

          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-7 h-7">
            <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12a7.5 7.5 0 0 0 15 0m-15 0a7.5 7.5 0 1 1 15 0m-15 0H3m16.5 0H21m-1.5 0H12m-8.457 3.077 1.41-.513m14.095-5.13 1.41-.513M5.106 17.785l1.15-.964m11.49-9.642 1.149-.964M7.501 19.795l.75-1.3m7.5-12.99.75-1.3m-6.063 16.658.26-1.477m2.605-14.772.26-1.477m0 17.726-.26-1.477M10.698 4.614l-.26-1.477M16.5 19.794l-.75-1.299M7.5 4.205 12 12m6.894 5.785-1.149-.964M6.256 7.178l-1.15-.964m15.352 8.864-1.41-.513M4.954 9.435l-1.41-.514M12.002 12l-3.75 6.495" />
          </svg>

        </A>
      </div>
    </div>
  )
}
export default App;
