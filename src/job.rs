use {
    crate::{
        entities::{
            job,
            prelude::{Job, Tracker},
        },
        error::{MapToErr, ToErr},
        tracker::ExtendedTracker,
        update_value::{UpdateOption, UpdateValue},
        user::SessionUser,
        AppData,
    },
    actix_web::{
        error::{ErrorBadRequest, ErrorForbidden},
        Error,
    },
    chrono::Utc,
    sea_orm::{
        ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
        QueryFilter, Set, TransactionTrait,
    },
    serde::{Deserialize, Serialize},
};

impl job::Model {
    pub async fn create(new_job: NewJob, user: &SessionUser) -> Result<Self, Error> {
        let active_model = new_job.into_active_model(user.id);
        active_model.insert(&AppData::get().conn).await.to_err()
    }

    pub async fn create_many(
        new_jobs: Vec<NewJob>,
        user: &SessionUser,
    ) -> Result<Vec<Self>, Error> {
        let txn = AppData::get().conn.begin().await.to_err()?;
        let mut jobs = Vec::with_capacity(new_jobs.len());

        for new_job in new_jobs {
            let job = new_job
                .into_active_model(user.id)
                .insert(&txn)
                .await
                .to_err()?;
            jobs.push(job);
        }

        txn.commit().await.to_err()?;
        Ok(jobs)
    }

    pub async fn get(job: u32, user: &SessionUser) -> Result<Self, Error> {
        let job = Job::find_by_id(job)
            .one(&AppData::get().conn)
            .await
            .map_to_err()?;

        if job.owner != user.id {
            return Err(ErrorForbidden("Forbidden"));
        }

        Ok(job)
    }

    pub async fn get_extended(job: u32, user: &SessionUser) -> Result<ExtendedJob, Error> {
        let (job, active_tracker) = Job::find_by_id(job)
            .find_also_related(Tracker)
            .one(&AppData::get().conn)
            .await
            .map_to_err()?;

        if job.owner != user.id {
            return Err(ErrorForbidden("Forbidden"));
        }

        let active_tracker = active_tracker.map(|tracker| ExtendedTracker {
            tracker,
            time_worked: None,
            is_active: true,
        });

        Ok(ExtendedJob {
            job,
            active_tracker,
        })
    }

    pub async fn get_extended_many(user: &SessionUser) -> Result<Vec<ExtendedJob>, Error> {
        let extended_jobs = Job::find()
            .filter(job::Column::Owner.eq(user.id))
            .find_also_related(Tracker)
            .all(&AppData::get().conn)
            .await
            .to_err()?
            .into_iter()
            .map(|(job, active_tracker)| ExtendedJob {
                job,
                active_tracker: active_tracker.map(|tracker| ExtendedTracker {
                    tracker,
                    time_worked: None,
                    is_active: true,
                }),
            })
            .collect::<Vec<ExtendedJob>>();

        Ok(extended_jobs)
    }

    pub async fn update(update_job: UpdateJob, job: job::Model) -> Result<ExtendedJob, Error> {
        let id = job.id;
        let mut update = job.into_active_model();
        update.name = update_job.name.into();
        update.company_name = update_job.company_name.into();
        update.company_logo = update_job.company_logo.into();
        update.description = update_job.description.into();
        update.disabled = update_job.disabled.into();

        let mut active_tracker = None;

        update.active_tracker = match update_job.active_tracker {
            UpdateOption::Set(Some(tracker_id)) => {
                let tracker = Tracker::find_by_id(tracker_id)
                    .one(&AppData::get().conn)
                    .await
                    .map_to_err()?;

                if tracker.job != Some(id) {
                    return Err(ErrorBadRequest("Tracker does not belong to this job"));
                }

                active_tracker = Some(tracker);
                ActiveValue::Set(Some(tracker_id))
            }
            _ => update_job.active_tracker.into(),
        };

        let job = update.update(&AppData::get().conn).await.to_err()?;
        let active_tracker = match job.active_tracker {
            Some(id) => match active_tracker {
                Some(tracker) => Some(tracker),
                None => {
                    let tracker = Tracker::find_by_id(id)
                        .one(&AppData::get().conn)
                        .await
                        .map_to_err()?;

                    if tracker.job != Some(id) {
                        return Err(ErrorBadRequest("Tracker does not belong to this job"));
                    }

                    Some(tracker)
                }
            },
            None => None,
        };

        Ok(ExtendedJob {
            job,
            active_tracker: active_tracker.map(|tracker| ExtendedTracker {
                tracker,
                time_worked: None,
                is_active: true,
            }),
        })
    }

    pub async fn delete(self) -> Result<(), Error> {
        ModelTrait::delete(self, &AppData::get().conn)
            .await
            .to_err()?;
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewJob {
    name: Option<String>,
    company_name: Option<String>,
    company_logo: Option<String>,
    description: Option<String>,
    #[serde(default = "bool::default")]
    disabled: bool,
}

impl NewJob {
    pub fn into_active_model(self, user_id: u32) -> job::ActiveModel {
        job::ActiveModel {
            owner: Set(user_id),
            name: Set(self.name),
            company_name: Set(self.company_name),
            company_logo: Set(self.company_logo),
            description: Set(self.description),
            created: Set(Utc::now()),
            disabled: Set(self.disabled),
            ..Default::default()
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedJob {
    #[serde(flatten)]
    pub job: job::Model,
    pub active_tracker: Option<ExtendedTracker>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateJob {
    #[serde(default)]
    name: UpdateOption<String>,
    #[serde(default)]
    company_name: UpdateOption<String>,
    #[serde(default)]
    company_logo: UpdateOption<String>,
    #[serde(default)]
    description: UpdateOption<String>,
    #[serde(default)]
    disabled: UpdateValue<bool>,
    #[serde(default)]
    active_tracker: UpdateOption<u32>,
}
