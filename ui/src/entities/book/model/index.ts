import { combine, createEffect, createStore } from "effector";
import type { Book } from "shared/api/book";
import api from "shared/api";

export const getBookListFx = createEffect((cursor?: string) => api.book.list(cursor));
export const $getBookListFxLoading = getBookListFx.pending;

export const $books = createStore<Book[]>([])
    .on(getBookListFx.doneData, (_, book) => book.data);

export const $booksList = combine($books, (books) => Object.values(books));
export const $booksListEmpty = $booksList.map((list) => list.length === 0);
