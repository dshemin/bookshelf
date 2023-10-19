import StorageAPI from "./storage";
import config from "shared/config";

class API {
    public storage: StorageAPI;

    public constructor(baseURL: string) {
        this.storage = new StorageAPI(baseURL);
    }
}

const api = new API(config.api.baseURL);

export default api;
