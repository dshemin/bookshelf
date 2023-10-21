import App from "app";
import React from "react";
import { createRoot } from "react-dom/client";
import { mockServer as setupMockServer } from "shared/mock";
import { setupTranslation } from "shared/i18n";
import config from "shared/config";
import { setupEffector } from "shared/effector";

setupTranslation(config.env);
setupMockServer(config.env);
setupEffector(config.env)

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const root = createRoot(document.getElementById("root")!);

root.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
