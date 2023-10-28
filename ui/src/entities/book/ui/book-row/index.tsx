import { Book } from "shared/api/book";
import { Row } from "antd";

export interface BookRowProps {
  data: Book
}

export const BookRow: React.FC<BookRowProps> = ({ data }) => (
    <Row>
        {data.title}
    </Row>
);
