use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Songs::Table)
                    .col(pk_auto(Songs::Id))
                    .col(string_null(Songs::Title))
                    .col(string_null(Songs::Link))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Songs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Songs {
    Table,
    Id,
    Title,
    Link,
    
}


