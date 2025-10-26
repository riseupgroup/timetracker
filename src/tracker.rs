use {
    crate::{
        authentication_middleware::InnerAuthentication,
        entities::{prelude::*, tracker::DisplayRangeUnit, *},
        error::{MapToErr, ToErr},
        update_value::{UpdateOption, UpdateValue},
        AppData,
    },
    actix_web::{
        error::{ErrorBadRequest, ErrorForbidden, ErrorInternalServerError},
        Error,
    },
    chrono::{DateTime, Datelike, Days, Months, NaiveDate, NaiveTime, Utc, Weekday},
    chrono_tz::Tz,
    sea_orm::{
        prelude::DateTimeUtc,
        sea_query::{Alias, Expr, Func, Query},
        ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel,
        IntoSimpleExpr, ModelTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
    },
    serde::{Deserialize, Serialize},
    tracker::TimePensumUnit,
};

macro_rules! calc_date {
    ($t:ty, $op:ident, $name:ident) => {
        paste::paste! {
            #[allow(dead_code)]
            pub fn [< checked_ $op _ $name >](self: &Self, date: $t, units: u32) -> Option<$t> {
                match self {
                    Self::None => date.[< checked_ $op _days >](Days::new(units as u64)),
                    Self::Week => date.[< checked_ $op _days >](Days::new(units as u64 * 7)),
                    Self::Month => date.[< checked_ $op _months >](Months::new(units)),
                    Self::Year => date.[< checked_ $op _months >](Months::new(units * 12)),
                }
            }
        }
    };
}

impl TimePensumUnit {
    pub fn get_start(&self, date: NaiveDate) -> Option<NaiveDate> {
        match self {
            Self::None => Some(date),
            Self::Week => Some(date.week(Weekday::Mon).first_day()),
            Self::Month => date.with_day(1),
            Self::Year => date.with_month(1)?.with_day(1),
        }
    }

    pub fn get_end(&self, date: NaiveDate) -> Option<NaiveDate> {
        match self {
            Self::None => date.checked_add_days(Days::new(1)),
            Self::Week => Some(
                date.checked_add_days(Days::new(7))?
                    .week(Weekday::Mon)
                    .first_day(),
            ),
            Self::Month => date.checked_add_months(Months::new(1))?.with_day(1),
            Self::Year => NaiveDate::from_ymd_opt(date.year() + 1, 1, 1),
        }
    }

    calc_date!(DateTime<Tz>, add, units);
    calc_date!(DateTime<Tz>, sub, units);
    calc_date!(NaiveDate, add, units_date);
    calc_date!(NaiveDate, sub, units_date);
}

impl DisplayRangeUnit {
    pub fn now(&self, timezone: Tz) -> DisplayRange {
        let now = Utc::now().with_timezone(&timezone);
        match self {
            Self::Week => DisplayRange::Week {
                year: now.year(),
                week: now.iso_week().week(),
            },
            Self::Month => DisplayRange::Month {
                year: now.year(),
                month: now.month(),
            },
            Self::Year => DisplayRange::Year { year: now.year() },
        }
    }
}

#[derive(Debug, Clone, Deserialize, Copy)]
#[serde(tag = "unit", content = "value")]
pub enum DisplayRange {
    Week { year: i32, week: u32 },
    Month { year: i32, month: u32 },
    Year { year: i32 },
}

impl DisplayRange {
    pub fn get_range(&self) -> Option<(NaiveDate, NaiveDate)> {
        let start: NaiveDate;
        let end: NaiveDate;
        match *self {
            Self::Week { year, week } => {
                start = NaiveDate::from_isoywd_opt(year, week, Weekday::Mon)?;
                end = NaiveDate::from_isoywd_opt(year, week, Weekday::Sun)?;
            }
            Self::Month { year, month } => {
                start = NaiveDate::from_ymd_opt(year, month, 1)?;
                end = NaiveDate::from_ymd_opt(year, month + 1, 1)
                    .or(NaiveDate::from_ymd_opt(year + 1, 1, 1))?
                    .pred_opt()?;
            }
            Self::Year { year } => {
                start = NaiveDate::from_ymd_opt(year, 1, 1)?;
                end = NaiveDate::from_ymd_opt(year + 1, 1, 1)?.pred_opt()?;
            }
        }
        Some((start, end))
    }

    pub fn apply_timezone(
        start: NaiveDate,
        end: NaiveDate,
        unit: TimePensumUnit,
        timezone: Tz,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        let start = unit
            .get_start(start)?
            .and_time(NaiveTime::MIN)
            .and_local_timezone(timezone)
            .earliest()?
            .to_utc();

        let end = unit
            .get_end(end)?
            .and_time(NaiveTime::MIN)
            .and_local_timezone(timezone)
            .earliest()?
            .to_utc();

        Some((start, end))
    }
}

impl tracker::Model {
    pub async fn get_using_job(
        user: &InnerAuthentication,
        job: Option<u32>,
        tracker: Option<u32>,
    ) -> Result<(Option<job::Model>, u32), Error> {
        let job = match job {
            Some(job) => {
                let job = Job::find_by_id(job)
                    .one(&AppData::get().conn)
                    .await
                    .map_to_err()?;

                if job.owner != user.id {
                    return Err(ErrorForbidden("Forbidden"));
                }

                Some(job)
            }
            None => None,
        };
        let tracker = match tracker {
            Some(tracker) => tracker,
            None => match &job {
                Some(job) => match job.active_tracker {
                    Some(tracker) => tracker,
                    None => return Err(ErrorBadRequest("Job does not have an active tracker")),
                },
                None => return Err(ErrorInternalServerError("tracker and job missing")),
            },
        };
        Ok((job, tracker))
    }

    /// # Return:
    /// boolean indicating wether tracker is the active tracker of the job or not
    pub async fn check_owner(
        &self,
        user: &InnerAuthentication,
        job: Option<&job::Model>,
    ) -> Result<bool, Error> {
        if let Some(job) = &job {
            if Some(job.id) != self.job {
                return Err(ErrorBadRequest("Tracker does not belong to this job"));
            }
        }

        if let Some(job_id) = self.job {
            let job = if let Some(job) = job {
                job
            } else {
                &Job::find_by_id(job_id)
                    .one(&AppData::get().conn)
                    .await
                    .map_to_err()?
            };

            if job.owner != user.id {
                return Err(ErrorForbidden("Forbidden"));
            }

            Ok(job.active_tracker == Some(self.id))
        } else if let Some(owner) = self.owner {
            if owner != user.id {
                return Err(ErrorForbidden("Forbidden"));
            }

            Ok(false)
        } else {
            Err(ErrorInternalServerError("Internal Server Error"))
        }
    }

    pub async fn calculate_time_worked(
        &self,
        timezone: Tz,
        end: Option<NaiveDate>,
    ) -> Result<u32, actix_web::Error> {
        let mut query = Query::select();
        query
            .from(Timeslot)
            .expr(
                Expr::expr(
                    Func::cust(Alias::new("TIME_TO_SEC")).arg(
                        Func::cust(Alias::new("TIMEDIFF"))
                            .arg(Func::coalesce(vec![
                                Expr::col(timeslot::Column::End).into_simple_expr(),
                                Expr::current_timestamp().into_simple_expr(),
                            ]))
                            .arg(Expr::col(timeslot::Column::Start)),
                    ),
                )
                .sum()
                .into_simple_expr()
                .cast_as(Alias::new("UNSIGNED INTEGER")),
            )
            .and_where(timeslot::Column::Tracker.eq(self.id));

        if let Some(end) = end {
            let end = self
                .time_pensum_unit
                .get_end(end)
                .ok_or(ErrorInternalServerError("Invalid date"))?
                .and_time(NaiveTime::MIN)
                .and_local_timezone(timezone)
                .earliest()
                .ok_or(ErrorInternalServerError("Invalid date"))?;

            let start = self
                .time_pensum_unit
                .checked_sub_units(end, 1)
                .ok_or(ErrorInternalServerError("Invalid date"))?;

            query
                .and_where(timeslot::Column::Start.gte(start.to_utc()))
                .and_where(timeslot::Column::Start.lt(end.to_utc()));
        }

        let conn = &AppData::get().conn;

        if let Some(res) = conn
            .query_one(conn.get_database_backend().build(&query))
            .await
            .to_err()?
        {
            Ok(res.try_get_by_index(0).unwrap_or(0))
        } else {
            Ok(0)
        }
    }

    pub fn check_time_valid(&self, time: DateTime<Utc>) -> Result<(), Error> {
        if let Some(valid_from) = self.valid_from {
            if time < valid_from {
                return Err(ErrorBadRequest("Time is before tracker's valid range"));
            }
        }

        if let Some(valid_until) = self.valid_until {
            if time > valid_until {
                return Err(ErrorBadRequest("Time is after tracker's valid range"));
            }
        }

        Ok(())
    }

    pub async fn get_current_timeslot(&self) -> Result<timeslot::Model, Error> {
        timeslot::Entity::find()
            .filter(timeslot::Column::Tracker.eq(self.id))
            .filter(timeslot::Column::End.is_null())
            .order_by_desc(timeslot::Column::Start)
            .one(&AppData::get().conn)
            .await
            .map_to_err()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedTracker {
    #[serde(flatten)]
    pub tracker: tracker::Model,
    pub time_worked: Option<u32>,
    pub is_active: bool,
}

impl ExtendedTracker {
    pub async fn create(
        user: &InnerAuthentication,
        new_tracker: NewTracker,
        job: Option<u32>,
    ) -> Result<Self, Error> {
        let job = match job {
            Some(job) => Some(
                Job::find_by_id(job)
                    .one(&AppData::get().conn)
                    .await
                    .map_to_err()?,
            ),
            None => None,
        };
        let (owner, job) = NewTracker::get_parent(user, job.as_ref())?;

        let tracker = new_tracker
            .into_active_model(owner, job)
            .insert(&AppData::get().conn)
            .await
            .to_err()?;

        Ok(Self {
            tracker,
            time_worked: None,
            is_active: false,
        })
    }

    pub async fn create_many(
        user: &InnerAuthentication,
        new_trackers: Vec<NewTracker>,
        job: Option<u32>,
    ) -> Result<Vec<Self>, Error> {
        let job = match job {
            Some(job) => Some(
                Job::find_by_id(job)
                    .one(&AppData::get().conn)
                    .await
                    .map_to_err()?,
            ),
            None => None,
        };
        let (owner, job) = NewTracker::get_parent(user, job.as_ref())?;

        let txn = AppData::get().conn.begin().await.to_err()?;
        let mut trackers = Vec::with_capacity(new_trackers.len());

        for new_tracker in new_trackers {
            let tracker = new_tracker
                .into_active_model(owner, job)
                .insert(&txn)
                .await
                .to_err()?;

            trackers.push(Self {
                tracker,
                time_worked: None,
                is_active: false,
            });
        }

        txn.commit().await.to_err()?;
        Ok(trackers)
    }

    pub async fn get(
        id: Option<u32>,
        job: Option<u32>,
        user: &InnerAuthentication,
    ) -> Result<Self, Error> {
        let (job, id) = tracker::Model::get_using_job(user, job, id).await?;

        let tracker = Tracker::find_by_id(id)
            .one(&AppData::get().conn)
            .await
            .map_to_err()?;

        let is_active = tracker.check_owner(user, job.as_ref()).await?;
        Ok(Self {
            tracker,
            time_worked: None,
            is_active,
        })
    }

    pub async fn get_with_time_worked(
        id: Option<u32>,
        job: Option<u32>,
        user: &InnerAuthentication,
        timezone: Tz,
    ) -> Result<Self, Error> {
        let mut tracker = Self::get(id, job, user).await?;

        let end = Utc::now().with_timezone(&timezone).date_naive();

        tracker.time_worked = Some(
            tracker
                .tracker
                .calculate_time_worked(timezone, Some(end))
                .await?,
        );

        Ok(tracker)
    }

    pub async fn get_many(
        user: &InnerAuthentication,
        job: Option<u32>,
    ) -> Result<Vec<Self>, Error> {
        let (query, active_tracker) = match job {
            Some(job) => {
                let job = Job::find_by_id(job)
                    .one(&AppData::get().conn)
                    .await
                    .map_to_err()?;

                if job.owner != user.id {
                    return Err(ErrorForbidden("Forbidden"));
                }

                (
                    Tracker::find().filter(tracker::Column::Job.eq(job.id)),
                    job.active_tracker,
                )
            }
            None => (
                Tracker::find().filter(tracker::Column::Owner.eq(user.id)),
                None,
            ),
        };

        let trackers = query.all(&AppData::get().conn).await.to_err()?;

        let trackers: Vec<Self> = trackers
            .into_iter()
            .map(|tracker| Self {
                is_active: active_tracker == Some(tracker.id),
                tracker,
                time_worked: None,
            })
            .collect();

        Ok(trackers)
    }

    pub async fn update(
        mut self,
        update_tracker: UpdateTracker,
        timezone: Tz,
    ) -> Result<Self, Error> {
        let mut tracker = self.tracker.into_active_model();
        // TODO: macro
        tracker.name = update_tracker.name.into();
        tracker.time_pensum = update_tracker.time_pensum.into();
        tracker.time_pensum_unit = update_tracker.time_pensum_unit.into();
        tracker.display_range_unit = update_tracker.display_range_unit.into();
        tracker.valid_from = update_tracker.valid_from.into();
        tracker.valid_until = update_tracker.valid_until.into();

        self.tracker = tracker.update(&AppData::get().conn).await.to_err()?;

        let end = Utc::now().with_timezone(&timezone).date_naive();

        self.time_worked = Some(
            self.tracker
                .calculate_time_worked(timezone, Some(end))
                .await?,
        );
        Ok(self)
    }

    pub async fn delete(self) -> Result<(), Error> {
        ModelTrait::delete(self.tracker, &AppData::get().conn)
            .await
            .to_err()?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewTracker {
    name: Option<String>,
    time_pensum: Option<u16>,
    #[serde(default, deserialize_with = "TimePensumUnit::deserialize_nullable")]
    time_pensum_unit: TimePensumUnit,
    display_range_unit: DisplayRangeUnit,
    valid_from: Option<DateTimeUtc>,
    valid_until: Option<DateTimeUtc>,
}

impl NewTracker {
    pub fn into_active_model(self, owner: Option<u32>, job: Option<u32>) -> tracker::ActiveModel {
        tracker::ActiveModel {
            owner: Set(owner),
            job: Set(job),
            name: Set(self.name),
            time_pensum: Set(self.time_pensum),
            time_pensum_unit: Set(self.time_pensum_unit),
            display_range_unit: Set(self.display_range_unit),
            valid_from: Set(self.valid_from),
            valid_until: Set(self.valid_until),
            created: Set(Utc::now()),
            ..Default::default()
        }
    }

    pub fn get_parent(
        user: &InnerAuthentication,
        job: Option<&job::Model>,
    ) -> Result<(Option<u32>, Option<u32>), Error> {
        let (owner, job) = match job {
            Some(job) => {
                if job.owner != user.id {
                    return Err(ErrorForbidden("Forbidden"));
                }
                (None, Some(job.id))
            }
            None => (Some(user.id), None),
        };
        Ok((owner, job))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Timezone {
    #[serde(alias = "tz")]
    pub timezone: Tz,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTracker {
    #[serde(default)]
    name: UpdateOption<String>,
    #[serde(default)]
    time_pensum: UpdateOption<u16>,
    #[serde(default)]
    time_pensum_unit: UpdateValue<TimePensumUnit>,
    #[serde(default)]
    display_range_unit: UpdateValue<DisplayRangeUnit>,
    #[serde(default)]
    valid_from: UpdateOption<DateTimeUtc>,
    #[serde(default)]
    valid_until: UpdateOption<DateTimeUtc>,
}
