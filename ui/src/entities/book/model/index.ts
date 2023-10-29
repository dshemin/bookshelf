import { combine, createEffect, createStore } from "effector";
import type { Book } from "shared/api/book";
import api from "shared/api";
import { useUnit } from "effector-react";

export const getBookListFx = createEffect((cursor?: string) => api.book.list(cursor));
export const $getBookListFxLoading = getBookListFx.pending;

export const getBookFx = createEffect((id: string) => api.book.get(id));
export const $getBookFxLoading = getBookFx.pending;

type BooksState = Record<string, Book>;

export const $books = createStore<BooksState>({})
    .on(getBookListFx.doneData, (_, books) => indexBooks(books.data))
    .on(getBookFx.doneData, (state, book) => ({
        ...state,
        [book.id]: book,
    }));

const indexBooks = (books: Book[]): BooksState => books.reduce(
    (prev: BooksState, curr: Book): BooksState => ({
        ...prev,
        [curr.id]: curr,
    }),
    {},
);

export const $booksList = combine($books, (books) => Object.values(books));
export const $booksListEmpty = $booksList.map((list) => list.length === 0);

export const useBook = (id: string): Book | undefined => {
    return useUnit($books)[id];
};
