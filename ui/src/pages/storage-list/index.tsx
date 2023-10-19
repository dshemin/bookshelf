import { Col, Empty, Layout, Spin } from "antd";
import { StorageRow, storageModel } from "entities/storage";
import { list, variant } from "@effector/reflect";
import { Storage } from "shared/api/storage";
import { combine } from "effector";

const StorageListPage: React.FC = () => (
    <Layout>
        <Content />
    </Layout>
);

const Item: React.FC<{ storage: Storage }> = ({ storage }) => <Col>
    <StorageRow
        data={storage}
    />
</Col>;

const TasksList = list({
    view: Item,
    source: storageModel.$storagesList,
    mapItem: {
        storage: (storage) => storage,
    },
    getKey: (storage) => storage.id,
});

// eslint-disable-next-line effector/enforce-store-naming-convention
const Content = variant({
    source: combine(
        {
            isLoading: storageModel.$getStorageListFxLoading,
            isEmpty: storageModel.$storagesListEmpty,
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
        empty: () => <Empty description="No storages found" />,
        ready: TasksList,
    },
    hooks: {
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        mounted: storageModel.getStorageListFx.prepend(() => { }),
    },
});


export default StorageListPage;
