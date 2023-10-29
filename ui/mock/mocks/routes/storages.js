const STORAGES = {
    data: [
        {
            id: "id1",
            name: "fs test 1",
            settings: {
                base_path: "/foo/bar",
            },
        },
    ],
};

module.exports = [
    {
        id: "get-storages",
        url: "/api/storages",
        method: "GET",
        variants: [
            {
                id: "success",
                type: "json",
                options: {
                    status: 200,
                    body: STORAGES,
                },
            },
            {
                id: "error",
                type: "json",
                options: {
                    status: 400,
                    body: {
                        message: "Error",
                    },
                },
            },
        ],
    },
];
