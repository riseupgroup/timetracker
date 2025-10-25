use sea_orm::{DatabaseBackend, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE `job` (
                `id` INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
                `owner` INT UNSIGNED NOT NULL,
                `name` VARCHAR(255) NULL,
                `company_name` VARCHAR(255) NULL,
                `company_logo` VARCHAR(255) NULL,
                `description` VARCHAR(255) NULL,
                `created` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                `disabled` TINYINT(1) NOT NULL DEFAULT 0
            )
        "#;
        let stmt = Statement::from_string(DatabaseBackend::MySql, sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"DROP TABLE `job`"#;
        let stmt = Statement::from_string(DatabaseBackend::MySql, sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
