import { Col, Empty, Layout, Spin } from "antd";
import { list, variant } from "@effector/reflect";
import { BookRow } from "entities/book/ui/book-row";
import { NormalizedBook } from "entities/book/model";
import { bookModel } from "entities/book";
import { combine } from "effector";

const MainPage: React.FC = () => (
    <Layout>
        <BooksList />
    </Layout>
);

const Item: React.FC<{ book: NormalizedBook }> = ({ book }) => (
    <Col>
        <BookRow
            data={book}
        />
    </Col>
);

const TasksList = list({
    view: Item,
    source: bookModel.$booksList,
    mapItem: {
        book: (book) => book,
    },
    getKey: (book) => book.id,
});

// eslint-disable-next-line effector/enforce-store-naming-convention
const BooksList = variant({
    source: combine(
        {
            isLoading: bookModel.$getBookListFxLoading,
            isEmpty: bookModel.$booksListEmpty,
        },
        ({ isLoading, isEmpty }) => {
            switch (true) {
                case isLoading:
                    return "loading";

                case isEmpty:
                    return "empty";
            }

            return "ready";
        },
    ),
    cases: {
        loading: () => <Spin size="large" />,
        empty: () => <Empty description="No books found" />,
        ready: TasksList,
    },
    hooks: {
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        mounted: bookModel.getBookListFx.prepend(() => { }),
    },
});

export default MainPage;
