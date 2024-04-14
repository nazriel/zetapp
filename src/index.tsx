/* @refresh reload */
import { render } from "solid-js/web";
import { lazy } from "solid-js";
import { Router, Route } from "@solidjs/router";
import { info, attachConsole } from "@tauri-apps/plugin-log";

import "./output.css";
import App from "./App";

const Settings = lazy(() => import("./pages/Settings"));
const Parameters = lazy(() => import("./pages/Parameters"));
const Stats = lazy(() => import("./pages/Stats"));

(async () => {
    await attachConsole();
})()

render(
    () => (
        <Router root={App}>
            <Route path="/" component={Stats} />
            <Route path="/params" component={Parameters} />
            <Route path="/settings" component={Settings} />
        </Router>
    ),
    document.getElementById("root") as HTMLElement
);
// render(() => <App />, document.getElementById("root") as HTMLElement);
