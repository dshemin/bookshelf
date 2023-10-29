import { Book } from "shared/api/book";
import { Link } from "react-router-dom";
import { Row } from "antd";

export interface BookRowProps {
    data: Book,
}

export const BookRow: React.FC<BookRowProps> = ({ data }) => (
    <Row>
        <Link to={`/books/${data.id}`}>{data.title}</Link>
    </Row>
);
