import { Link } from "react-router-dom";
import { NormalizedBook } from "entities/book/model";
import { Row } from "antd";

export interface BookRowProps {
    data: NormalizedBook,
}

export const BookRow: React.FC<BookRowProps> = ({ data }) => (
    <Row>
        <Link to={`/books/${data.id}`}>{data.title}</Link>
    </Row>
);
