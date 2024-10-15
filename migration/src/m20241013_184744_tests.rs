use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Tests::Table)
                    .col(pk_auto(Tests::Id))
                    .col(string_null(Tests::Title))
                    .col(blob_null(Tests::Pdf))
                    .col(money_null(Tests::Cost))
                    .col(integer_null(Tests::Downloads))
                    .col(text_null(Tests::Description))
                    .col(date_null(Tests::Date))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tests::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tests {
    Table,
    Id,
    Title,
    Pdf,
    Cost,
    Downloads,
    Description,
    Date,
    
}


