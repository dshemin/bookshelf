import { AppServer } from "shared/mock/types";
import { Book } from "shared/api/book";
import { Model } from "miragejs";
import { ModelDefinition } from "miragejs/-types";
import { Serializer } from "shared/mock/utils";

export const BookModel: ModelDefinition<Book> = Model.extend({});

export const bookEndpoints = (server: AppServer): void => {
    server.get("/books", (schema) => schema.all("book"));
    server.get("/books/:id", (schema, request) => {
        const attrs = JSON.parse(request.requestBody);

        return schema.create("book", attrs);
    });
};

export const bookSeed = (server: AppServer): void => {
    server.create("book", {
        id: crypto.randomUUID(),
        title: "Book1",
        uri: "https://localhost:5173/sample.pdf",
        tags: [
            "tag1",
            "tag2",
            "tag3",
        ],
        highlights: [],
        bookmarks: [],
    });
};

export const bookSerializer = Serializer;
