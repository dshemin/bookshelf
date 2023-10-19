import { Registry, Server } from "miragejs";
import { StorageModel, storageEndpoints, storageSeed, storageSerializer } from "./storage";
import { AnyFactories } from "miragejs/-types";
import Schema from "miragejs/orm/schema";

export const models = {
    storage: StorageModel,
};

export const serializers = {
    storage: storageSerializer,
};

export const endpoints = [storageEndpoints];

export const seeds = [storageSeed];

export type AppRegistry = Registry<typeof models, AnyFactories>;
export type AppSchema = Schema<AppRegistry>;
export type AppServer = Server<AppRegistry>;
