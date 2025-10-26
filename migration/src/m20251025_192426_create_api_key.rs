use sea_orm::{DatabaseBackend, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        CREATE TABLE `api_key` (
            `id` INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
            `name` VARCHAR(100) NULL,
            `owner` INT UNSIGNED NOT NULL,
            `disabled` TINYINT(1) NOT NULL DEFAULT 0,
            `valid_until` DATETIME NULL,
            `added` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            `last_changed` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            `last_used` DATETIME NULL DEFAULT NULL,
            `key` VARCHAR(255) NOT NULL
        )"#;
        let stmt = Statement::from_string(DatabaseBackend::MySql, sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            DROP TABLE `api_key`
        "#;
        let stmt = Statement::from_string(DatabaseBackend::MySql, sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
