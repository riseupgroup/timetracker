use sea_orm::{DatabaseBackend, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            ALTER TABLE `job`
                ADD COLUMN `active_tracker` INT UNSIGNED NULL,
                ADD CONSTRAINT `job-active_tracker` FOREIGN KEY (`active_tracker`) REFERENCES `tracker`(`id`) ON DELETE SET NULL
        "#;
        let stmt = Statement::from_string(DatabaseBackend::MySql, sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            ALTER TABLE `job`
                DROP CONSTRAINT `job-active_tracker`,
                DROP COLUMN `active_tracker`
        "#;
        let stmt = Statement::from_string(DatabaseBackend::MySql, sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
