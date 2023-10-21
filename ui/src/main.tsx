import App from "app";
import React from "react";
import config from "shared/config";
import { createRoot } from "react-dom/client";
import { setupEffector } from "shared/effector";
import { setupMockServer } from "shared/mock";
import { setupTranslation } from "shared/i18n";

setupTranslation(config.env);
setupMockServer(config.env);
setupEffector(config.env);

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const root = createRoot(document.getElementById("root")!);

root.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
