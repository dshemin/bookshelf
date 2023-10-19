import { ConfigProvider } from "antd";
import { Provider } from "./types";

export const withConfigProvider: Provider = (component: () => React.ReactNode) => () => (
    <ConfigProvider>
        {component()}
    </ConfigProvider>
);
