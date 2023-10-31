import "react-pdf/dist/esm/Page/AnnotationLayer.css";
import "react-pdf/dist/esm/Page/TextLayer.css";
import "./styles.css";

import { Card, Modal, Pagination, PaginationProps } from "antd";
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
    const [selectedHighlight, setSelectedHighlight] = useState(0);
    const [isModalOpen, setIsModalOpen] = useState(false);

    useEffect(() => {
        const event = "click";
        const handler = (ev: MouseEvent): EffectCallbackReturn => {
            //@ts-expect-error 'cause we didn't have enough types.
            if (!ev || !ev.target || ev.target.localName !== "mark") {
                return;
            }

            //@ts-expect-error 'cause we didn't have enough types.
            const index = ev.target.dataset.index;

            setSelectedHighlight(index);
            setIsModalOpen(true);
        };
        document.addEventListener(event, handler);

        return () => {
            document.removeEventListener(event, handler);
        };
    }, [
        book?.highlights,
        page,
        setSelectedHighlight,
        setIsModalOpen,
    ]);

    const textRenderer = useCallback(
        (textItem: TextItem) => {
            const lineCurr = textItem.itemIndex;
            let str = textItem.str;

            book?.highlights[page]
                .forEach(({ lineStart, lineEnd, symbolStart, symbolEnd }, index) => {
                    str = highlightText(
                        str,
                        index,
                        lineCurr,
                        lineStart,
                        lineEnd,
                        symbolStart,
                        symbolEnd,
                    );
                });

            return str;
        },
        [book?.highlights, page],
    );

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

    const handleModelClose = useCallback(
        () => {
            setIsModalOpen(false);
        },
        [setIsModalOpen],
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

    const highlight = book.highlights[page][selectedHighlight];

    return (
        <Card
            title={book.title}
        >
            <Pagination {...paginationProps} />
            <Document
                file={book.uri}
                onLoadSuccess={onDocumentLoaded}
                className="pdf-document"
            >
                <Page
                    pageNumber={page}
                    customTextRenderer={textRenderer}
                />
            </Document>
            <Pagination {...paginationProps} />

            <Modal
                title={highlight.title}
                open={isModalOpen}
                onCancel={handleModelClose}
                footer={null}
            >
                {highlight.note}
            </Modal>
        </Card>
    );
};

const highlightText = (
    str: string,
    highlightIndex: number,
    lineCurr: number,
    lineStart: number,
    lineEnd: number,
    symbolStart: number,
    symbolEnd: number,
): string => {
    // We should check that current line is inside highlight.
    if (lineStart < lineCurr && lineCurr > lineEnd) {
        return str;
    }

    let highlight: HighlightResult;

    // Single line highlight.
    if (lineStart === lineEnd) {
        highlight = highlightSingleLine(str, symbolStart, symbolEnd);
    } else {
        // Multiline highlight.
        highlight = highlightMultiLine(str, lineCurr, lineStart, lineEnd, symbolStart, symbolEnd);
    }

    const { prefix, marked, suffix } = highlight;

    // Check if current highlight is multiline.
    return `${prefix}<mark data-index="${highlightIndex}">${marked}</mark>${suffix}`;
};

interface HighlightResult {
    prefix?: string,
    marked?: string,
    suffix?: string,
}

/*
    highlightSingleLine
    Simply mark text between symbolStart and symbolEnd.
*/
const highlightSingleLine = (
    str: string,
    symbolStart: number,
    symbolEnd: number,
): HighlightResult => ({
    prefix: str.substring(0, symbolStart),
    marked: str.substring(symbolStart, symbolEnd),
    suffix: str.substring(symbolEnd),
});

/*
    highlightMultiLine
    Handles three different scenarios:

    1. Current line is the first line of highlight;
    2. Current line is inside of highlight;
    3. Current line is the last line of highlights.
*/
const highlightMultiLine = (
    str: string,
    lineCurr: number,
    lineStart: number,
    lineEnd: number,
    symbolStart: number,
    symbolEnd: number,
): HighlightResult => {
    const res: HighlightResult = {};

    switch (lineCurr) {
        // First scenario.
        // Mark whole text from the symbolStart to the end of the line.
        case lineStart:
            res.prefix = str.substring(0, symbolStart);
            res.marked = str.substring(symbolStart);
            break;

        // Third scenario.
        // Mark whole text from the beginning of the line to symbolEnd.
        case lineEnd:
            res.marked = str.substring(0, symbolEnd);
            res.suffix = str.substring(symbolEnd);
            break;

        // Second scenario.
        // Mark whole line.
        default:
            res.marked = str;
    }

    return res;
};

type TextItem = Parameters<NonNullable<PageProps["customTextRenderer"]>>[0];
type EffectCallbackReturn = ReturnType<EffectCallback>;
