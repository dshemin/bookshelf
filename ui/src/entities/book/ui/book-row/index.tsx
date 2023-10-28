import { Row } from "antd";
import { Book } from "shared/api/book";

export interface BookRowProps {
  data: Book
}

export const BookRow: React.FC<BookRowProps> = ({ data }) => (
    <Row>
        {data.title}
    </Row>
);
