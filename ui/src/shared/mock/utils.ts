import { PaginatedData } from "shared/api/types";
import { RestSerializer } from "miragejs";

export const Serializer = RestSerializer.extend({
    // eslint-disable-next-line @typescript-eslint/no-explicit-any,@typescript-eslint/no-unused-vars
    serialize: (res: any, _: any): PaginatedData<any> => {
        return {
            data: res.models,
        };
    },
});
