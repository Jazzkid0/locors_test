use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Music::Table)
                    .col(pk_auto(Music::Id))
                    .col(string_null(Music::Title))
                    .col(blob_null(Music::Pdf))
                    .col(money_null(Music::Cost))
                    .col(integer_null(Music::Downloads))
                    .col(text_null(Music::Description))
                    .col(date_null(Music::Date))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Music::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Music {
    Table,
    Id,
    Title,
    Pdf,
    Cost,
    Downloads,
    Description,
    Date,
    
}


