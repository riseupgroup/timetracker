use {
    crate::{
        authentication_middleware::InnerAuthentication,
        entities::{prelude::*, *},
        error::{MapToErr, ToErr},
        tracker::{DisplayRange, ExtendedTracker},
        update_value::{UpdateOption, UpdateValue},
        AppData,
    },
    actix_web::{
        error::{ErrorBadRequest, ErrorInternalServerError},
        Error,
    },
    chrono::{NaiveDate, Utc},
    chrono_tz::Tz,
    migration::Expr,
    sea_orm::{
        prelude::DateTimeUtc, ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel,
        IntoSimpleExpr, ModelTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
    },
    serde::{Deserialize, Serialize},
    serde_inline_default::serde_inline_default,
    std::cmp::Ordering,
    tracker::TimePensumUnit,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTimeslot {
    #[serde(default)]
    tracker: UpdateValue<u32>,
    #[serde(default)]
    start: UpdateValue<DateTimeUtc>,
    #[serde(default)]
    end: UpdateOption<DateTimeUtc>,
    #[serde(default)]
    comment: UpdateOption<String>,
}

impl timeslot::Model {
    pub async fn create(
        user: &InnerAuthentication,
        new_timeslot: NewTimeslot,
        tracker: Option<u32>,
        job: Option<u32>,
    ) -> Result<timeslot::Model, Error> {
        let (job, tracker) = tracker::Model::get_using_job(user, job, tracker).await?;

        let tracker = Tracker::find_by_id(tracker)
            .one(&AppData::get().conn)
            .await
            .map_to_err()?;

        tracker.check_owner(user, job.as_ref()).await?;

        tracker.check_time_valid(new_timeslot.start)?;
        if let Some(end) = new_timeslot.end {
            tracker.check_time_valid(end)?;
        }

        new_timeslot
            .into_active_model(tracker.id)
            .insert(&AppData::get().conn)
            .await
            .to_err()
    }

    pub async fn create_many(
        user: &InnerAuthentication,
        new_timeslots: Vec<NewTimeslot>,
        tracker: Option<u32>,
        job: Option<u32>,
    ) -> Result<Vec<timeslot::Model>, Error> {
        let (job, tracker) = tracker::Model::get_using_job(user, job, tracker).await?;
        let tracker = Tracker::find_by_id(tracker)
            .one(&AppData::get().conn)
            .await
            .map_to_err()?;

        tracker.check_owner(user, job.as_ref()).await?;

        let txn = AppData::get().conn.begin().await.to_err()?;
        let mut timeslots = Vec::with_capacity(new_timeslots.len());

        for new_timeslot in new_timeslots {
            tracker.check_time_valid(new_timeslot.start)?;
            if let Some(end) = new_timeslot.end {
                tracker.check_time_valid(end)?;
            }

            let timeslot = new_timeslot
                .into_active_model(tracker.id)
                .insert(&txn)
                .await
                .to_err()?;

            timeslots.push(timeslot);
        }

        txn.commit().await.to_err()?;
        Ok(timeslots)
    }

    pub async fn get_with_tracker(
        timeslot: u64,
        tracker: Option<u32>,
        user: &InnerAuthentication,
        job: Option<u32>,
    ) -> Result<(timeslot::Model, tracker::Model), Error> {
        let timeslot = Timeslot::find_by_id(timeslot)
            .one(&AppData::get().conn)
            .await
            .map_to_err()?;

        let tracker = match tracker {
            Some(tracker) => {
                if tracker != timeslot.tracker {
                    return Err(ErrorBadRequest("Timeslot does not belong to this tracker"));
                }
                tracker
            }
            None => timeslot.tracker,
        };

        let tracker = ExtendedTracker::get(Some(tracker), job, user).await?;

        Ok((timeslot, tracker.tracker))
    }

    pub async fn get(
        timeslot: u64,
        tracker: Option<u32>,
        user: &InnerAuthentication,
        job: Option<u32>,
    ) -> Result<timeslot::Model, Error> {
        let (timeslot, _) = Self::get_with_tracker(timeslot, tracker, user, job).await?;
        Ok(timeslot)
    }

    pub async fn get_many(
        user: &InnerAuthentication,
        params: TimeslotSearchParams,
        tracker: Option<u32>,
        job: Option<u32>,
    ) -> Result<Vec<timeslot::Model>, Error> {
        let (job, tracker) = tracker::Model::get_using_job(user, job, tracker).await?;
        let tracker = Tracker::find_by_id(tracker)
            .one(&AppData::get().conn)
            .await
            .map_to_err()?;

        tracker.check_owner(user, job.as_ref()).await?;
        let mut query = Timeslot::find().filter(timeslot::Column::Tracker.eq(tracker.id));

        if let Some(search) = params.search {
            query = query.filter(timeslot::Column::Comment.like(format!("%{search}%")));
        }

        query = match params.sort {
            TimeslotSort::StartDesc => query.order_by_desc(timeslot::Column::Start),
            TimeslotSort::StartAsc => query.order_by_asc(timeslot::Column::Start),
            TimeslotSort::TimeWorkedAsc => query.order_by_asc(Expr::cust_with_exprs(
                "TIMEDIFF(?, ?)",
                [
                    timeslot::Column::End.into_simple_expr(),
                    timeslot::Column::Start.into_simple_expr(),
                ],
            )),
            TimeslotSort::TimeWorkedDesc => query.order_by_desc(Expr::cust_with_exprs(
                "TIMEDIFF(?, ?)",
                [
                    timeslot::Column::End.into_simple_expr(),
                    timeslot::Column::Start.into_simple_expr(),
                ],
            )),
        };

        let timeslots = query
            .limit(params.limit)
            .all(&AppData::get().conn)
            .await
            .to_err()?;

        Ok(timeslots)
    }

    pub async fn update(
        self,
        update: UpdateTimeslot,
        tracker: tracker::Model,
    ) -> Result<Self, Error> {
        let mut timeslot = self.into_active_model();
        if let UpdateValue::Set(start) = update.start {
            tracker.check_time_valid(start)?;
        }
        if let UpdateOption::Set(Some(end)) = update.end {
            tracker.check_time_valid(end)?;
        }

        timeslot.start = update.start.into();
        timeslot.end = update.end.into();
        timeslot.comment = update.comment.into();
        timeslot.tracker = update.tracker.into();

        timeslot.update(&AppData::get().conn).await.to_err()
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
pub struct NewTimeslot {
    #[serde(default = "Utc::now")]
    start: DateTimeUtc,
    end: Option<DateTimeUtc>,
    comment: Option<String>,
}

impl NewTimeslot {
    pub fn into_active_model(self, tracker_id: u32) -> timeslot::ActiveModel {
        timeslot::ActiveModel {
            start: Set(self.start),
            end: Set(self.end),
            comment: Set(self.comment),
            tracker: Set(tracker_id),
            ..Default::default()
        }
    }
}

#[serde_inline_default]
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TimeslotSearchParams {
    search: Option<String>,
    #[serde(default)]
    sort: TimeslotSort,
    #[serde_inline_default(255)]
    limit: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TimeslotFilterParams {
    range: Option<DisplayRange>,
    #[serde(alias = "tz")]
    timezone: Tz,
    search: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub enum TimeslotSort {
    #[default]
    StartDesc,
    StartAsc,
    TimeWorkedAsc,
    TimeWorkedDesc,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    start: NaiveDate,
    end: NaiveDate,
    timeslots: Vec<timeslot::Model>,
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Eq for Unit {}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Unit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

fn split_in_units(
    timeslots: Vec<timeslot::Model>,
    time_pensum_unit: TimePensumUnit,
    start: NaiveDate,
    end: NaiveDate,
    timezone: Tz,
) -> Option<Vec<Unit>> {
    let mut units = if time_pensum_unit == TimePensumUnit::None {
        vec![Unit {
            start,
            end,
            timeslots,
        }]
    } else {
        let mut units: Vec<Unit> = Vec::new();
        {
            let mut start = time_pensum_unit.get_start(start)?;
            let end = time_pensum_unit.get_start(end)?;
            while start <= end {
                let end = time_pensum_unit
                    .checked_add_units_date(start, 1)?
                    .checked_sub_days(chrono::Days::new(1))?;
                units.push(Unit {
                    start,
                    end,
                    timeslots: Vec::new(),
                });
                start = time_pensum_unit.checked_add_units_date(start, 1)?;
            }
        }
        for timeslot in timeslots {
            let start_date = timeslot.start.with_timezone(&timezone).date_naive();
            let start = time_pensum_unit.get_start(start_date)?;
            let unit = units.iter_mut().find(|x| x.start == start)?;
            unit.timeslots.push(timeslot);
        }
        units
    };

    units.sort();
    for unit in &mut units {
        unit.timeslots.sort();
    }
    Some(units)
}

pub async fn get_units(
    tracker: tracker::Model,
    params: TimeslotFilterParams,
) -> Result<Vec<Unit>, Error> {
    let range = match params.range {
        Some(x) => x,
        None => tracker.display_range_unit.now(params.timezone),
    };

    let (start_date, end_date) = range.get_range().ok_or(ErrorBadRequest("Invalid range"))?;
    let (start, end) = DisplayRange::apply_timezone(
        start_date,
        end_date,
        tracker.time_pensum_unit,
        params.timezone,
    )
    .ok_or(ErrorBadRequest("Invalid range"))?;

    let mut query = Timeslot::find()
        .filter(timeslot::Column::Tracker.eq(tracker.id))
        .filter(timeslot::Column::Start.gte(start))
        .filter(timeslot::Column::Start.lt(end));

    if let Some(search) = params.search {
        query = query.filter(timeslot::Column::Comment.like(format!("%{search}%")));
    }

    let timeslots = query.all(&AppData::get().conn).await.to_err()?;

    split_in_units(
        timeslots,
        tracker.time_pensum_unit,
        start_date,
        end_date,
        params.timezone,
    )
    .ok_or(ErrorInternalServerError("Invalid date"))
}
