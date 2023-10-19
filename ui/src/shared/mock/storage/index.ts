import { AppServer } from "shared/mock/types";
import { Model } from "miragejs";
import { ModelDefinition } from "miragejs/-types";
import { Serializer } from "shared/mock/utils";
import { Storage } from "shared/api/storage";

export const StorageModel: ModelDefinition<Storage> = Model.extend({});

export const storageEndpoints = (server: AppServer): void => {
    server.get("/storages", (schema) => schema.all("storage"));
    server.post("/storages", (schema, request) => {
        const attrs = JSON.parse(request.requestBody);

        return schema.create("storage", attrs);
    });
};

export const storageSeed = (server: AppServer): void => {
    server.create("storage", {
        id: crypto.randomUUID(),
        name: "fs test 1",
        settings: {
            // eslint-disable-next-line camelcase
            base_path: "/foo/bar",
        },
    });
};

export const storageSerializer = Serializer;
