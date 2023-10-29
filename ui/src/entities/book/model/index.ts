import { combine, createEffect, createStore } from "effector";
import type { Book, Bookmark, Highlight } from "shared/api/book";
import api from "shared/api";
import { useUnit } from "effector-react";

export const getBookListFx = createEffect((cursor?: string) => api.book.list(cursor));
export const $getBookListFxLoading = getBookListFx.pending;

export const getBookFx = createEffect((id: string) => api.book.get(id));
export const $getBookFxLoading = getBookFx.pending;

export type NormalizedBookHighlights = Record<number, Highlight[]>;
export type NormalizedBookBookmarks = Record<number, Bookmark>;

export type NormalizedBook = Omit<Book, "highlights" | "bookmarks"> & {
    highlights: NormalizedBookHighlights,
    bookmarks: NormalizedBookBookmarks,
}

type BooksState = Record<string, NormalizedBook>;

export const $books = createStore<BooksState>({})
    .on(getBookListFx.doneData, (_, books) => normalizeBooks(books.data))
    .on(getBookFx.doneData, (state, book) => ({
        ...state,
        [book.id]: normalizeBook(book),
    }));

const normalizeBooks = (books: Book[]): BooksState => books.reduce(
    (prev: BooksState, curr: Book): BooksState => ({
        ...prev,
        [curr.id]: normalizeBook(curr),
    }),
    {},
);

const normalizeBook = (book: Book): NormalizedBook => {
    return {
        id: book.id,
        title: book.title,
        uri: book.uri,
        tags: book.tags,
        highlights: normalizeBookHighlights(book.highlights),
        bookmarks: normalizeBookBookmarks(book.bookmarks),
    }
}

const normalizeBookHighlights = (highlights: Highlight[]): NormalizedBookHighlights =>
    highlights.reduce(
        (prev: NormalizedBookHighlights, curr: Highlight) => {
            const key = curr.page;

            if (!prev[key]) {
                prev[key] = [];
            }

            prev[key].push(curr);
            return prev;
        },
        {},
    );

const normalizeBookBookmarks = (bookmarks: Bookmark[]): NormalizedBookBookmarks =>
    bookmarks.reduce(
        (prev: NormalizedBookBookmarks, curr: Bookmark) => ({
            ...prev,
            [curr.page]: curr,
        }),
        {},
    );

export const $booksList = combine($books, (books) => Object.values(books));
export const $booksListEmpty = $booksList.map((list) => list.length === 0);

export const useBook = (id: string): NormalizedBook | undefined => {
    return useUnit($books)[id];
};
