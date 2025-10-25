pub use sea_orm_migration::prelude::*;
mod m20241001_173026_create_job;
mod m20241001_173034_create_tracker;
mod m20241001_173039_create_timeslot;
mod m20241001_173342_job_add_active_tracker;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241001_173026_create_job::Migration),
            Box::new(m20241001_173034_create_tracker::Migration),
            Box::new(m20241001_173039_create_timeslot::Migration),
            Box::new(m20241001_173342_job_add_active_tracker::Migration),
        ]
    }
}
