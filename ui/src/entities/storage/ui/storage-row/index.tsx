import { Row } from "antd";
import { Storage } from "shared/api/storage";

export interface StorageRowProps {
  data: Storage
}

export const StorageRow: React.FC<StorageRowProps> = ({ data }) => <Row>
    {data.name}
</Row>;

