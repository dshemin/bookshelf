import { Navigate, Route, Routes } from "react-router-dom";
import { lazy } from "react";

const MainPage = lazy(() => import("./main"));
const StorageListPage = lazy(() => import("./storage-list"));
const BookPage = lazy(() => import("./book"));

export const Routing: React.FC = () => (
    <Routes>
        <Route id="main" path="/" element={<MainPage />} />
        <Route id="storage-list" path="/storages" element={<StorageListPage />} />
        <Route id="book" path="/books/:id" element={<BookPage />} />
        <Route path="*" element={<Navigate to="/" />} />
    </Routes>
);
