Bookshelf
=========

A selfhosted books organizer and reader.

Use:
- PostgreSQL for storing all metadata;
- Minio (or other S3 compatible storage) for storing books;
- Keycloak as authentication and authorization provider.

Development
===========

Process
-------

Run `docker compose up` for setupping all required services.

From scratch you will be able to login as ordinal user with credentials:
Email: test@example.com
Password: 123456

Tools
-----

You should have next tools installed:
- cargo
- cargo-watch
- sqlx cli