use sea_orm::{DatabaseBackend, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE `tracker` (
                `id` INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
                `name` VARCHAR(255) NULL,
                `owner` INT UNSIGNED,
                `job` INT UNSIGNED,
                `created` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                `valid_from` DATETIME DEFAULT CURRENT_TIMESTAMP,
                `valid_until` DATETIME DEFAULT NULL,
                `time_pensum` SMALLINT UNSIGNED NULL,
                `time_pensum_unit` ENUM('none', 'week', 'month', 'year') NOT NULL DEFAULT 'none',
                `display_range_unit` ENUM('week', 'month', 'year') NOT NULL,
                CONSTRAINT `tracker-job` FOREIGN KEY (`job`) REFERENCES `job`(`id`) ON DELETE CASCADE,
                CONSTRAINT `tracker-owner+or+job` CHECK (`owner` IS NOT NULL XOR `job` IS NOT NULL),
                CONSTRAINT `tracker-valid_from+before+valid_until` CHECK (`valid_from` < `valid_until` OR `valid_from` IS NULL OR `valid_until` IS NULL)
            )
        "#;
        let stmt = Statement::from_string(DatabaseBackend::MySql, sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            DROP TABLE `tracker`
        "#;
        let stmt = Statement::from_string(DatabaseBackend::MySql, sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
