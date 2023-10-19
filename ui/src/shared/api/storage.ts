import AbstractAPI, { Query } from "./abstract";
import { PaginatedData } from "./types";

class API extends AbstractAPI {
    public async list(cursor?: string): Promise<PaginatedData<Storage>> {
        const query: Query = {};

        if (cursor) {
            query["cursor"] = cursor;
        }

        const resp = await this.request("GET", "/api/storages", query);
        const json = await resp.json();

        return json;
    }
}

export interface Storage {
  id: string,
  name: string,
  settings: Settings,
}

export type Settings = FSSettings;

export interface FSSettings {
  base_path: string,
}

export default API;
