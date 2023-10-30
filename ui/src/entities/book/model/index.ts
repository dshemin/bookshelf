import type { Book, Bookmark, Highlight } from "shared/api/book";
import { combine, createEffect, createStore } from "effector";
import api from "shared/api";
import { useUnit } from "effector-react";

export const getBookListFx = createEffect((cursor?: string) => api.book.list(cursor));
export const $getBookListFxLoading = getBookListFx.pending;

export const getBookFx = createEffect((id: string) => api.book.get(id));
export const $getBookFxLoading = getBookFx.pending;

export type NormalizedHighlights = Record<number, Highlight[]>;
export type NormalizedBookmarks = Record<number, Bookmark>;

export type NormalizedBook = Omit<Book, "bookmarks" | "highlights"> & {
    highlights: NormalizedHighlights,
    bookmarks: NormalizedBookmarks,
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
        highlights: normalizeHighlights(book.highlights),
        bookmarks: normalizeBookmarks(book.bookmarks),
    };
};

const normalizeHighlights = (highlights: Highlight[]): NormalizedHighlights => highlights.reduce(
    (prev: NormalizedHighlights, curr: Highlight) => {
        const key = curr.page;

        if (!prev[key]) {
            prev[key] = [];
        }

        prev[key].push(curr);

        return prev;
    },
    {},
);

const normalizeBookmarks = (bookmarks: Bookmark[]): NormalizedBookmarks => bookmarks.reduce(
    (prev: NormalizedBookmarks, curr: Bookmark) => ({
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
