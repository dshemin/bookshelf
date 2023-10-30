import { Card, Pagination, PaginationProps } from "antd";
import { Document, Page, PageProps, pdfjs } from "react-pdf";
import { EffectCallback, useCallback, useEffect, useState } from "react";
import { NormalizedBook } from "entities/book/model";

export interface BookCardProps {
    book?: NormalizedBook,
    isLoading: boolean,
}

pdfjs.GlobalWorkerOptions.workerSrc = new URL(
    "pdfjs-dist/build/pdf.worker.min.js",
    import.meta.url,
).toString();

export const BookCard: React.FC<BookCardProps> = ({ book, isLoading }) => {
    const [page, setPage] = useState(1);
    const [numPages, setNumPages] = useState(0);

    const onDocumentLoaded = useCallback(
        (data: { numPages: number }) => {
            setNumPages(data.numPages);
        },
        [setNumPages],
    );

    const onPageChange = useCallback(
        (page: number) => {
            setPage(page);
        },
        [setPage],
    );

    useEffect(() => {
        const event = "click";
        const handler = (ev: MouseEvent): EffectCallbackReturn => {
            //@ts-expect-error 'cause we didn't have enough types.
            if (!ev || !ev.target || ev.target.localName !== "mark") {
                return;
            }

            //@ts-expect-error 'cause we didn't have enough types.
            const index = ev.target.dataset.index;

            // eslint-disable-next-line no-alert
            window.alert(book?.highlights[page][index].title);
        };
        document.addEventListener(event, handler);

        return () => {
            document.removeEventListener(event, handler);
        };
    }, [book?.highlights, page]);

    const textRenderer = useCallback(
        (textItem: TextItem) => {
            const index = textItem.itemIndex;
            let str = textItem.str;

            book?.highlights[page]
                .forEach(({ lineStart, lineEnd, symbolStart, symbolEnd }, hIndex) => {
                    if (lineStart < index && index > lineEnd) {
                        return;
                    }
                    const before = str.substring(0, symbolStart);
                    const marked = str.substring(symbolStart, symbolEnd);
                    const after = str.substring(symbolEnd);

                    str = `${before}<mark data-index="${hIndex}">${marked}</mark>${after}`;
                });

            return str;
        },
        [book?.highlights, page],
    );

    if (!book || isLoading) {
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
            title={book.title}
        >
            <Pagination {...paginationProps} />
            <Document
                file={book.uri}
                onLoadSuccess={onDocumentLoaded}
            >
                <Page
                    pageNumber={page}
                    customTextRenderer={textRenderer}
                />
            </Document>
            <Pagination {...paginationProps} />
        </Card>
    );
};

type TextItem = Parameters<NonNullable<PageProps["customTextRenderer"]>>[0];
type EffectCallbackReturn = ReturnType<EffectCallback>;
