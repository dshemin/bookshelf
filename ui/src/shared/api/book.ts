import AbstractAPI, { Query } from "./abstract";
import { PaginatedData } from "./types";

class API extends AbstractAPI {
    public async list(cursor?: string): Promise<PaginatedData<Book>> {
        const query: Query = {};

        if (cursor) {
            query["cursor"] = cursor;
        }

        const resp = await this.request("GET", "/api/books", query);
        const json = await resp.json();

        return json;
    }

    public async get(id: string): Promise<Book> {
        const resp = await this.request("GET", `/api/books/${id}`);
        const json = await resp.json();

        return json;
    }
}

export interface Book {
  id: string,
  title: string,
  uri: string,
  tags: string[],
  highlights: Highlight[],
  bookmarks: Bookmark[],
}

export interface Highlight {
    page: number,
    lineStart: number,
    lineEnd: number,
    symbolStart: number,
    symbolEnd: number,
    title: string,
    note: string,
}

export interface Bookmark {
    page: number,
    title: string,
    note: string,
}

export default API;
