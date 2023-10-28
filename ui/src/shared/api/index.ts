import BookAPI from "./book";
import StorageAPI from "./storage";
import config from "shared/config";

class API {
    public storage: StorageAPI;
    public book: BookAPI;

    public constructor(baseURL: string) {
        this.storage = new StorageAPI(baseURL);
        this.book = new BookAPI(baseURL);
    }
}

const api = new API(config.api.baseURL);

export default api;
