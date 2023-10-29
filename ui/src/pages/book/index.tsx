import "react-pdf/dist/esm/Page/AnnotationLayer.css";
import "react-pdf/dist/esm/Page/TextLayer.css";


import { BookCard, bookModel } from "entities/book";
import { Button, Layout, Result } from "antd";
import { Link, useParams } from "react-router-dom";
import { reflect } from "@effector/reflect";
import { useEffect } from "react";


interface ItemProps {
    isLoading: boolean,
}

const Item: React.FC<ItemProps> = ({ isLoading }) => {
    const { id } = useParams();
    const book = bookModel.useBook(id ?? "");

    useEffect(() => {
        bookModel.getBookFx(id ?? "");
    }, [id]);

    if (!book && !isLoading) {
        return (
            <Result
                status="404"
                title="404"
                subTitle="Book was not found"
                extra={<Link to="/"><Button type="primary">Back to main</Button></Link>}
            />
        );
    }

    return (
        <Layout>
            <Layout.Content>
                <BookCard
                    book={book}
                    isLoading={isLoading}
                />
            </Layout.Content>
        </Layout>
    );
};

const BookPage = reflect({
    view: Item,
    bind: {
        isLoading: bookModel.$getBookFxLoading,
    },
});


export default BookPage;
