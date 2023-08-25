CREATE TABLE "storages" (
    "id" UUID PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "settings" JSONB NOT NULL
);

CREATE TABLE "users" (
    "id" UUID PRIMARY KEY,
    "role" VARCHAR(50) NOT NULL
);
