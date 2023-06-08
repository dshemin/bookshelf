use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Self::create_storage_table(manager).await?;
        Self::create_user_table(manager).await?;
        Self::create_shelf_table(manager).await?;
        Self::create_book_table(manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Shelf::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Storage::Table).to_owned())
            .await?;

        Ok(())
    }
}

impl Migration {
    async fn create_storage_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Storage::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Storage::Id).uuid().primary_key())
                    .col(ColumnDef::new(Storage::Name).string().not_null())
                    .col(ColumnDef::new(Storage::Type).string_len(40).not_null())
                    .col(ColumnDef::new(Storage::Settings).json_binary().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn create_user_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().primary_key())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn create_shelf_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Shelf::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Shelf::Id).uuid().primary_key())
                    .col(ColumnDef::new(Shelf::Name).string().not_null())
                    .col(ColumnDef::new(Shelf::OwnerID).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_shelf_user")
                    .from(Shelf::Table, Shelf::OwnerID)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
    async fn create_book_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Book::Id).uuid().primary_key())
                    .col(ColumnDef::new(Book::Name).string().not_null())
                    .col(ColumnDef::new(Book::Path).json_binary().not_null())
                    .col(ColumnDef::new(Book::StorageID).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_book_storage")
                    .from(Book::Table, Book::StorageID)
                    .to(Storage::Table, Storage::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Storage {
    Table,
    Id,
    Name,
    Type,
    Settings,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}

#[derive(Iden)]
enum Shelf {
    Table,
    Id,
    Name,
    OwnerID,
}

#[derive(Iden)]
enum Book {
    Table,
    Id,
    Name,
    Path,
    StorageID,
}
