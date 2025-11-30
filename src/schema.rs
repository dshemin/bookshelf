// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Nullable<Text>,
        first_name -> Text,
        middle_name -> Nullable<Text>,
        second_name -> Nullable<Text>,
    }
}

diesel::table! {
    bookmarks (rowid) {
        rowid -> Integer,
        id -> Text,
        book_id -> Text,
        page -> SmallInt,
        title -> Text,
        note -> Nullable<Text>,
        owner_id -> Text,
    }
}

diesel::table! {
    books (id) {
        id -> Nullable<Text>,
        storage_id -> Text,
        title -> Text,
        path -> Text,
        uploader_id -> Text,
    }
}

diesel::table! {
    books_authors (book_id, author_id) {
        book_id -> Text,
        author_id -> Text,
    }
}

diesel::table! {
    books_tags (book_id, tag_id) {
        book_id -> Text,
        tag_id -> Text,
    }
}

diesel::table! {
    highlights (rowid) {
        rowid -> Integer,
        id -> Text,
        book_id -> Text,
        page -> SmallInt,
        start -> SmallInt,
        end -> SmallInt,
        title -> Text,
        note -> Nullable<Text>,
        owner_id -> Text,
    }
}

diesel::table! {
    storages (id) {
        id -> Nullable<Text>,
        name -> Text,
        settings -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Nullable<Text>,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Text>,
        login -> Text,
        password -> Nullable<Text>,
        role -> Text,
    }
}

diesel::joinable!(bookmarks -> books (book_id));
diesel::joinable!(bookmarks -> users (owner_id));
diesel::joinable!(books -> storages (storage_id));
diesel::joinable!(books -> users (uploader_id));
diesel::joinable!(books_authors -> books (book_id));
diesel::joinable!(books_tags -> books (book_id));
diesel::joinable!(books_tags -> tags (tag_id));
diesel::joinable!(highlights -> books (book_id));
diesel::joinable!(highlights -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    bookmarks,
    books,
    books_authors,
    books_tags,
    highlights,
    storages,
    tags,
    users,
);
