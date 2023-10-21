import { AppServer, endpoints, models, seeds, serializers } from "./types";
import config from "shared/config";
import { createServer } from "miragejs";

export const setupMockServer = (environment = "test"): AppServer => {
    return createServer({
        environment,

        models,

        serializers,

        seeds(server) {
            seeds.forEach(f => f(server));
        },

        routes() {
            this.logging = true;
            this.urlPrefix = config.api.baseURL;
            this.namespace = "api";

            endpoints.forEach(f => f(this));
        },
    });
};
