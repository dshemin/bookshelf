import "app/index.css";
import "normalize.css";

import { App as AntdApp, Flex, Layout, Menu, MenuProps } from "antd";
import { CSSProperties, Key, useCallback, useMemo, useState } from "react";
import { DatabaseOutlined, HomeOutlined } from "@ant-design/icons";
import { ArrElement } from "shared/types";
import { Link } from "react-router-dom";
import { Routing } from "pages";
import withProviders from "app/providers";

const { Header, Content } = Layout;

const flexStyle: CSSProperties = {
    height: "100%",
};

const headerStyle: CSSProperties = {
    textAlign: "left",
    color: "#fff",
    height: 64,
    paddingInline: 50,
    lineHeight: "64px",
    backgroundColor: "#7dbcea",
};

const contentStyle: CSSProperties = {
    padding: "10px",
};

type MenuItems = MenuProps["items"];
type MenuItem = ArrElement<MenuItems>;
type MenuOnClick = MenuProps["onClick"];

const App: React.FC = () => {
    const [current, setCurrent] = useState<Key>("home");

    const menuItems: MenuItems = useMemo(() => [
        {
            key: "home",
            label:
        <Link to="/">
            <HomeOutlined />
        </Link>
            ,
        },
        {
            key: "storages",
            label:
        <Link to="/storages">
            <DatabaseOutlined />
        </Link>
            ,
        },
    ], []);

    const onClick: MenuOnClick = useCallback((i: MenuItem) => {
        if (!i || !i.key) {
            throw new Error("menu item or key not set");
        }
        setCurrent(i.key);
    }, []);

    return (
        <AntdApp>
            <Flex style={flexStyle} vertical>
                <Header style={headerStyle}>Bookshelf</Header>
                <Menu
                    onClick={onClick}
                    selectedKeys={[current as string]}
                    items={menuItems}
                    mode="horizontal"
                />
                <Content style={contentStyle}>
                    <Routing />
                </Content>
            </Flex>
        </AntdApp>
    );
};

const WrappedApp = withProviders(App);
WrappedApp.displayName = "Root";

export default WrappedApp;
