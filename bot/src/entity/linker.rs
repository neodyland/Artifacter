use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "linker")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub discord_id: f64,
    pub genshin_id: i32,
    pub allow_quote: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub fn cast_u64_to_f64(id: u64) -> f64 {
    id as f64
}

use sea_orm_migration::prelude::{
    sea_query, ColumnDef, DbErr, DeriveMigrationName, MigrationTrait, SchemaManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if !(manager.has_table(Entity.table_name()).await?) {
            manager
                .create_table(
                    sea_query::Table::create()
                        .if_not_exists()
                        .table(Entity)
                        .col(
                            ColumnDef::new(Column::AllowQuote)
                                .not_null()
                                .boolean()
                                .default(true),
                        )
                        .col(
                            ColumnDef::new(Column::GenshinId)
                                .integer()
                                .not_null()
                                .default(100000000),
                        )
                        .col(
                            ColumnDef::new(Column::DiscordId)
                                .double()
                                .not_null()
                                .primary_key()
                                .default(1.0),
                        )
                        .to_owned(),
                )
                .await?;
        };
        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Entity)
                    .add_column_if_not_exists(
                        ColumnDef::new(Column::AllowQuote)
                            .not_null()
                            .boolean()
                            .default(true),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Column::GenshinId)
                            .integer()
                            .not_null()
                            .default(100000000),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Column::DiscordId)
                            .double()
                            .not_null()
                            .primary_key()
                            .default(1.0),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        todo!();
    }
}
