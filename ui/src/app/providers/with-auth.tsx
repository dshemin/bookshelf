import {
    AuthProvider,
    AuthProviderProps,
} from "react-oidc-context";
import { Provider } from "./types";
import { User } from "oidc-client-ts";
import config from "shared/config";

/* eslint-disable camelcase */
const oidcConfig: AuthProviderProps = {
    authority: config.auth.authority,
    client_id: config.auth.clientID,
    redirect_uri: config.auth.redirectURI,
    // eslint-disable-next-line @typescript-eslint/no-invalid-void-type,@typescript-eslint/no-unused-vars
    onSigninCallback: (_: User | void): void => {
        window.history.replaceState(
            {},
            document.title,
            window.location.pathname,
        );
    },
};
/* eslint-enable camelcase */

export const withAuth: Provider = (component: () => React.ReactNode) => () => (
    <AuthProvider {...oidcConfig}>
        {component()}
    </AuthProvider>
);
