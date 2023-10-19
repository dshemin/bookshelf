import App from "app";
import React from "react";
import ReactDOM from "react-dom/client";
import { attachLogger } from "effector-logger";
import { mockServer } from "shared/mock";

if (process.env.NODE_ENV === "development") {
    mockServer({ environment: "development" });
    attachLogger({
        name: "bookshelf",
    });
}

const root = document.getElementById("root");

ReactDOM
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    .createRoot(root!)
    .render(
        <React.StrictMode>
            <App />
        </React.StrictMode>,
    );
