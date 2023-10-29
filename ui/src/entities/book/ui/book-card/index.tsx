import { Card, Pagination, PaginationProps } from "antd";
import { Document, Page, pdfjs } from "react-pdf";
import { useCallback, useState } from "react";
import { Book } from "shared/api/book";

export interface BookCardProps {
  data?: Book,
  isLoading: boolean,
}

pdfjs.GlobalWorkerOptions.workerSrc = new URL(
    "pdfjs-dist/build/pdf.worker.min.js",
    import.meta.url,
).toString();

export const BookCard: React.FC<BookCardProps> = ({ data, isLoading }) => {
    const [page, setPage] = useState(1);
    const [numPages, setNumPages] = useState(0);

    const onDocumentLoaded = useCallback((data: {numPages: number}) => {
        setNumPages(data.numPages);
    }, [ setNumPages ]);

    const onPageChange = useCallback((page: number) => {
        setPage(page);
    }, [ setPage ]);

    if (!data || isLoading) {
        return null;
    }

    const paginationProps: PaginationProps = {
        current: page,
        total: numPages,
        pageSize: 1,
        onChange: onPageChange,
    };

    return (
        <Card
            title={data.title}
        >
            <Pagination {...paginationProps}/>
            <Document
                file={data.uri}
                onLoadSuccess={onDocumentLoaded}
            >
                <Page pageNumber={page} />
            </Document>
            <Pagination {...paginationProps}/>
        </Card>
    );
};
