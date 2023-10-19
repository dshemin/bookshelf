import { Navigate, Route, Routes } from "react-router-dom";
import { lazy } from "react";

const MainPage = lazy(() => import("./main"));
const StorageListPage = lazy(() => import("./storage-list"));

export const Routing: React.FC = () => (
    <Routes>
        <Route id="main" path="/" element={<MainPage />} />
        <Route id="storage-list" path="/storages" element={<StorageListPage />} />
        <Route path="*" element={<Navigate to="/" />} />
    </Routes>
);
