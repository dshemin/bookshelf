import { BrowserRouter } from "react-router-dom";
import type { Provider } from "./types";
import { Suspense } from "react";

export const withRouter: Provider = (component: () => React.ReactNode) => () => (
    <BrowserRouter>
        <Suspense fallback="Loading...">
            {component()}
        </Suspense>
    </BrowserRouter>
);

