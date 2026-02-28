-- All ID's are UUID. SQLite doesn't have UUID type so BLOB will be used instead
-- for space efficiency.

CREATE TABLE "storages" (
    "id" BLOB PRIMARY KEY NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "settings" TEXT NOT NULL
);

CREATE TABLE "users" (
    "id" BLOB PRIMARY KEY NOT NULL,
    "login" VARCHAR(1024) NOT NULL,
    "password" VARCHAR(1024) NOT NULL,
    "role" VARCHAR(128) NOT NULL
);
CREATE UNIQUE INDEX idx_users_unique_login ON users(login);

CREATE TABLE "books" (
    "id" BLOB PRIMARY KEY NOT NULL,
    "storage_id" BLOB NOT NULL REFERENCES "storages" ("id") ON DELETE RESTRICT,
    "title" VARCHAR(1024) NOT NULL,
    "path" TEXT NOT NULL,
    "uploader_id" BLOB NOT NULL REFERENCES "users" ("id") ON DELETE RESTRICT
);

CREATE TABLE "tags" (
    "id" BLOB PRIMARY KEY NOT NULL,
    "name" VARCHAR(1024) NOT NULL
);

CREATE TABLE "authors" (
    "id" BLOB PRIMARY KEY NOT NULL,
    "first_name" VARCHAR(255) NOT NULL,
    "middle_name" VARCHAR(255),
    "second_name" VARCHAR(255)
);

CREATE TABLE "books_tags" (
    "book_id" BLOB NOT NULL REFERENCES "books" ("id") ON DELETE CASCADE,
    "tag_id" BLOB NOT NULL REFERENCES "tags" ("id") ON DELETE CASCADE,
    PRIMARY KEY ("book_id", "tag_id")
);

CREATE TABLE "books_authors" (
    "book_id" BLOB NOT NULL REFERENCES "books" ("id") ON DELETE CASCADE,
    "author_id" BLOB NOT NULL REFERENCES "author" ("id") ON DELETE CASCADE,
    PRIMARY KEY ("book_id", "author_id")
);

CREATE TABLE "highlights" (
    "id" BLOB PRIMARY KEY NOT NULL,
    "book_id" BLOB NOT NULL REFERENCES "books" ("id") ON DELETE CASCADE,
    "page" SMALLINT NOT NULL,
    "start" SMALLINT NOT NULL,
    "end" SMALLINT NOT NULL,
    "title" VARCHAR(512) NOT NULL,
    "note" TEXT,
    "owner_id" BLOB NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE
);

CREATE TABLE "bookmarks" (
    "id" BLOB PRIMARY KEY NOT NULL,
    "book_id" BLOB NOT NULL REFERENCES "books" ("id") ON DELETE CASCADE,
    "page" SMALLINT NOT NULL,
    "title" VARCHAR(512) NOT NULL,
    "note" TEXT,
    "owner_id" BLOB NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE
);

