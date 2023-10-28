import { Registry, Server } from "miragejs";
import { StorageModel, storageEndpoints, storageSeed, storageSerializer } from "./storage";
import { AnyFactories } from "miragejs/-types";
import Schema from "miragejs/orm/schema";
import { BookModel, bookEndpoints, bookSeed, bookSerializer } from "./book";

export const models = {
    storage: StorageModel,
    book: BookModel,
};

export const serializers = {
    storage: storageSerializer,
    book: bookSerializer,
};

export const endpoints = [
    storageEndpoints,
    bookEndpoints,
];

export const seeds = [
    storageSeed,
    bookSeed,
];

export type AppRegistry = Registry<typeof models, AnyFactories>;
export type AppSchema = Schema<AppRegistry>;
export type AppServer = Server<AppRegistry>;
