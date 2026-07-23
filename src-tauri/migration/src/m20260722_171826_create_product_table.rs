use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260722_171826_create_product_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Products::Uuid)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Products::Barcode).string().not_null())
                    .col(ColumnDef::new(Products::Title).string().not_null())
                    .col(ColumnDef::new(Products::Price).decimal().not_null())
                    .col(ColumnDef::new(Products::Amount).decimal().not_null())
                    .col(ColumnDef::new(Products::Dimension).string().not_null())
                    .col(
                        ColumnDef::new(Products::CreatedAt)
                            .string()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .col(
                        ColumnDef::new(Products::UpdatedAt)
                            .string()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Products::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Products {
    Table,
    Uuid,
    Barcode,
    Title,
    Price,
    Amount,
    Dimension,
    CreatedAt,
    UpdatedAt,
}
