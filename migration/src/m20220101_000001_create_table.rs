use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Content::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Content::Id)
                            .not_null()
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Content::Title).string().not_null())
                    .col(ColumnDef::new(Content::Text).string().not_null())
                    .col(ColumnDef::new(Content::Source).string().not_null())
                    .col(ColumnDef::new(Content::Url).string().null())
                    .col(ColumnDef::new(Content::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Content::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Content {
    Table,
    Id,
    Title,
    Text,

    Source,
    Url,
    CreatedAt,
}
