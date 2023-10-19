abstract class AbstractAPI {
    private baseURL: string;

    public constructor(baseURL: string) {
        this.baseURL = baseURL;
    }

    protected async request<Body>(
        method: "DELETE" | "GET" | "POST" | "PUT",
        path: string,
        query?: Query,
        body?: Body,
        token?: string,
    ): Promise<Response> {
        const headers: Record<string, string> = {
            "Content-Type": "application/json",
        };

        if (token !== null) {
            headers["Authorization"] = `Bearer ${token}`;
        }

        const init: RequestInit = {
            method,
            headers,
            body: JSON.stringify(body),
            mode: "cors",
        };

        const url = this.buildURL(path, query);

        const data = await fetch(url, init);

        return data;
    }

    private buildURL(path: string, query: Query): URL {
        const u = new URL(path, this.baseURL);
        u.search = new URLSearchParams(query).toString();

        return u;
    }
}

export type Query = ConstructorParameters<typeof URLSearchParams>[0];

export default AbstractAPI;
