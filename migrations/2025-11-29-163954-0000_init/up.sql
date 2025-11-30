CREATE TABLE "storages" (
    "id" TEXT PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "settings" TEXT NOT NULL
);

CREATE TABLE "users" (
    "id" TEXT PRIMARY KEY,
    "login" VARCHAR(1024) NOT NULL,
    "password" VARCHAR(1024),
    "role" VARCHAR(128) NOT NULL
);

CREATE TABLE "books" (
    "id" TEXT PRIMARY KEY,
    "storage_id" TEXT NOT NULL REFERENCES "storages" ("id") ON DELETE RESTRICT,
    "title" VARCHAR(1024) NOT NULL,
    "path" TEXT NOT NULL,
    "uploader_id" TEXT NOT NULL REFERENCES "users" ("id") ON DELETE RESTRICT
);

CREATE TABLE "tags" (
    "id" TEXT PRIMARY KEY,
    "name" VARCHAR(1024) NOT NULL
);

CREATE TABLE "authors" (
    "id" TEXT PRIMARY KEY,
    "first_name" VARCHAR(255) NOT NULL,
    "middle_name" VARCHAR(255),
    "second_name" VARCHAR(255)
);

CREATE TABLE "books_tags" (
    "book_id" TEXT NOT NULL REFERENCES "books" ("id") ON DELETE CASCADE,
    "tag_id" TEXT NOT NULL REFERENCES "tags" ("id") ON DELETE CASCADE,
    PRIMARY KEY ("book_id", "tag_id")
);

CREATE TABLE "books_authors" (
    "book_id" TEXT NOT NULL REFERENCES "books" ("id") ON DELETE CASCADE,
    "author_id" TEXT NOT NULL REFERENCES "author" ("id") ON DELETE CASCADE,
    PRIMARY KEY ("book_id", "author_id")
);

CREATE TABLE "highlights" (
    "id" TEXT NOT NULL,
    "book_id" TEXT NOT NULL REFERENCES "books" ("id") ON DELETE CASCADE,
    "page" SMALLINT NOT NULL,
    "start" SMALLINT NOT NULL,
    "end" SMALLINT NOT NULL,
    "title" VARCHAR(512) NOT NULL,
    "note" TEXT,
    "owner_id" TEXT NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE
);

CREATE TABLE "bookmarks" (
    "id" TEXT NOT NULL,
    "book_id" TEXT NOT NULL REFERENCES "books" ("id") ON DELETE CASCADE,
    "page" SMALLINT NOT NULL,
    "title" VARCHAR(512) NOT NULL,
    "note" TEXT,
    "owner_id" TEXT NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE
);

