CREATE TABLE "storages" (
    "id" UUID PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "settings" JSONB NOT NULL
);

CREATE TABLE "users" (
    "id" UUID PRIMARY KEY,
    "login" VARCHAR(1024) NOT NULL,
    "password" VARCHAR(1024),
    "external_id" VARCHAR(1024),
    "role" VARCHAR(128) NOT NULL
);

CREATE TABLE "books" (
    "id" UUID PRIMARY KEY,
    "storage_id" UUID NOT NULL REFERENCES "storages" ("id") ON DELETE RESTRICT,
    "title" VARCHAR(1024) NOT NULL,
    "path" JSONB NOT NULL,
    "uploader_id" UUID NOT NULL REFERENCES "users" ("id") ON DELETE RESTRICT
);

CREATE TABLE "tags" (
    "id" UUID PRIMARY KEY,
    "name" VARCHAR(1024) NOT NULL
);

CREATE TABLE "books_tags" (
    "book_id" UUID NOT NULL REFERENCES "books" ("id") ON DELETE CASCADE,
    "tag_id" UUID NOT NULL REFERENCES "tags" ("id") ON DELETE CASCADE,
    PRIMARY KEY ("book_id", "tag_id")
);

CREATE TABLE "highlights" (
    "id" UUID NOT NULL,
    "book_id" UUID NOT NULL REFERENCES "books" ("id") ON DELETE CASCADE,
    "page" SMALLINT NOT NULL,
    "start" SMALLINT NOT NULL,
    "end" SMALLINT NOT NULL,
    "title" VARCHAR(512) NOT NULL,
    "note" TEXT,
    "owner_id" UUID NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE
);

CREATE TABLE "bookmarks" (
    "id" UUID NOT NULL,
    "book_id" UUID NOT NULL REFERENCES "books" ("id") ON DELETE CASCADE,
    "page" SMALLINT NOT NULL,
    "title" VARCHAR(512) NOT NULL,
    "note" TEXT,
    "owner_id" UUID NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE
);
