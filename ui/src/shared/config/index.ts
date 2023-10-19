export default {
    auth: {
        authority: import.meta.env.BS_AUTH_OIDC_AUTHORITY ?? "",
        clientID: import.meta.env.BS_AUTH_OIDC_CLIENT_ID ?? "",
        redirectURI: import.meta.env.BS_AUTH_OIDC_REDIRECT_URI ?? "",
    },
    api: {
        baseURL: import.meta.env.BS_API_BASE_URL,
    },
};
