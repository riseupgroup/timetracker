use sea_orm::{DatabaseBackend, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE `timeslot` (
                `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
                `tracker` INT UNSIGNED NOT NULL,
                `start` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                `end` DATETIME NULL,
                `comment` VARCHAR(1023) NULL,
                CONSTRAINT `timeslot-tracker` FOREIGN KEY (`tracker`) REFERENCES `tracker`(`id`) ON DELETE CASCADE,
                CONSTRAINT `timeslot-start+before+end` CHECK (`start` < `end` OR `end` IS NULL)
            )
        "#;

        let stmt = Statement::from_string(DatabaseBackend::MySql, sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"DROP TABLE `timeslot`"#;
        let stmt = Statement::from_string(DatabaseBackend::MySql, sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
