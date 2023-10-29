const BOOKS = {
    data: [
        {
            id: "id1",
            title: "Book1",
            uri: "http://localhost:5173/sample.pdf",
            tags: [
                "tag1",
                "tag2",
                "tag3",
            ],
            highlights: [
                {
                    page: 1,
                    lineStart: 1,
                    lineEnd: 1,
                    symbolStart: 10,
                    symbolEnd: 20,
                    title: "highlight1",
                    note: "highlight1 note",
                }
            ],
            bookmarks: [],
        },
    ],
};

module.exports = [
    {
        id: "get-books",
        url: "/api/books",
        method: "GET",
        variants: [
            {
                id: "success",
                type: "json",
                options: {
                    status: 200,
                    body: BOOKS,
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
    {
        id: "get-book",
        url: "/api/books/:id",
        method: "GET",
        variants: [
            {
                id: "success",
                type: "json",
                options: {
                    status: 200,
                    body: BOOKS.data[0],
                },
            },
        ],
    },
];
