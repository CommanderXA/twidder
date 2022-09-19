use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Media::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Media::Id).primary_key().uuid().not_null())
                    .col(ColumnDef::new(Media::Type).string().not_null())
                    .col(ColumnDef::new(Media::File).blob(BlobSize::Long).not_null())
                    .col(ColumnDef::new(Media::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).primary_key().uuid().not_null())
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::FirstName).string().not_null())
                    .col(ColumnDef::new(User::LastName).string().not_null())
                    .col(ColumnDef::new(User::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Post::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Text).string().not_null())
                    .col(ColumnDef::new(Post::Media).uuid().null())
                    .col(ColumnDef::new(Post::User).uuid().null())
                    .col(ColumnDef::new(Post::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_post_media_to_media_id")
                            .from_tbl(Post::Table)
                            .from_col(Post::Media)
                            .to_tbl(Media::Table)
                            .to_col(Media::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_post_user_to_user_id")
                            .from_tbl(Post::Table)
                            .from_col(Post::User)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Media::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Post::User).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Media {
    Table,
    Id,
    Type,
    File,
    CreatedAt,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Username,
    Email,
    FirstName,
    LastName,
    CreatedAt,
}

#[derive(Iden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
    Media,
    User,
    CreatedAt,
}
