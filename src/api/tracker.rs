use {
    crate::{
        tracker::{ExtendedTracker, NewTracker, Timezone, UpdateTracker},
        user::SessionUser,
    },
    actix_session::Session,
    actix_web::{delete, get, patch, post, put, web, Error, HttpResponse},
    chrono::NaiveDate,
    chrono_tz::Tz,
    serde::Deserialize,
};

#[post("/api/trackers")]
async fn create_standalone(
    session: Session,
    new_tracker: web::Json<NewTracker>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let tracker = ExtendedTracker::create(&user, new_tracker.into_inner(), None).await?;
    Ok(HttpResponse::Created().json(tracker))
}

#[post("/api/jobs/{id}/trackers")]
async fn create_job(
    session: Session,
    new_tracker: web::Json<NewTracker>,
    job: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let tracker =
        ExtendedTracker::create(&user, new_tracker.into_inner(), Some(job.into_inner())).await?;
    Ok(HttpResponse::Created().json(tracker))
}

#[put("/api/trackers")]
async fn create_many_standalone(
    session: Session,
    new_trackers: web::Json<Vec<NewTracker>>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let trackers = ExtendedTracker::create_many(&user, new_trackers.into_inner(), None).await?;
    Ok(HttpResponse::Created().json(trackers))
}

#[put("/api/jobs/{id}/trackers")]
async fn create_many_job(
    session: Session,
    new_trackers: web::Json<Vec<NewTracker>>,
    job: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let trackers =
        ExtendedTracker::create_many(&user, new_trackers.into_inner(), Some(job.into_inner()))
            .await?;
    Ok(HttpResponse::Created().json(trackers))
}

#[get("/api/trackers/{id}")]
async fn get_standalone(
    session: Session,
    tracker: web::Path<u32>,
    params: web::Query<Timezone>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let tracker = ExtendedTracker::get_with_time_worked(
        Some(tracker.into_inner()),
        None,
        &user,
        params.timezone,
    )
    .await?;
    Ok(HttpResponse::Ok().json(tracker))
}

#[get("/api/jobs/{job_id}/trackers/{tracker_id}")]
async fn get_job(
    session: Session,
    path: web::Path<(u32, u32)>,
    params: web::Query<Timezone>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job_id, tracker_id) = path.into_inner();

    let tracker = ExtendedTracker::get_with_time_worked(
        Some(tracker_id),
        Some(job_id),
        &user,
        params.timezone,
    )
    .await?;

    Ok(HttpResponse::Ok().json(tracker))
}

#[get("/api/trackers")]
async fn get_many_standalone(session: Session) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let trackers = ExtendedTracker::get_many(&user, None).await?;
    Ok(HttpResponse::Ok().json(trackers))
}

#[get("/api/jobs/{id}/trackers")]
async fn get_many_job(session: Session, job: web::Path<u32>) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let trackers = ExtendedTracker::get_many(&user, Some(job.into_inner())).await?;
    Ok(HttpResponse::Ok().json(trackers))
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct TimeWorkedParams {
    #[serde(alias = "tz")]
    timezone: Tz,
    end: Option<NaiveDate>,
}

#[get("/api/trackers/{id}/time-worked")]
async fn get_time_worked_standalone(
    session: Session,
    tracker: web::Path<u32>,
    params: web::Query<TimeWorkedParams>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;

    let tracker = ExtendedTracker::get(Some(tracker.into_inner()), None, &user)
        .await?
        .tracker;

    let time_worked = tracker
        .calculate_time_worked(params.timezone, params.end)
        .await?;

    Ok(HttpResponse::Ok().json(time_worked))
}

#[get("/api/jobs/{job_id}/trackers/{tracker_id}/time-worked")]
async fn get_time_worked_job(
    session: Session,
    path: web::Path<(u32, u32)>,
    params: web::Query<TimeWorkedParams>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job_id, tracker_id) = path.into_inner();

    let tracker = ExtendedTracker::get(Some(tracker_id), Some(job_id), &user)
        .await?
        .tracker;

    let time_worked = tracker
        .calculate_time_worked(params.timezone, params.end)
        .await?;

    Ok(HttpResponse::Ok().json(time_worked))
}

#[patch("/api/trackers/{id}")]
async fn update_standalone(
    session: Session,
    tracker: web::Path<u32>,
    update_tracker: web::Json<UpdateTracker>,
    params: web::Query<Timezone>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;

    let tracker = ExtendedTracker::get(Some(tracker.into_inner()), None, &user)
        .await?
        .update(update_tracker.into_inner(), params.timezone)
        .await?;

    Ok(HttpResponse::Ok().json(tracker))
}

#[patch("/api/jobs/{job_id}/trackers/{tracker_id}")]
async fn update_job(
    session: Session,
    path: web::Path<(u32, u32)>,
    update_tracker: web::Json<UpdateTracker>,
    params: web::Query<Timezone>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job_id, tracker_id) = path.into_inner();

    let tracker = ExtendedTracker::get(Some(tracker_id), Some(job_id), &user)
        .await?
        .update(update_tracker.into_inner(), params.timezone)
        .await?;

    Ok(HttpResponse::Ok().json(tracker))
}

#[delete("/api/trackers/{id}")]
async fn delete_standalone(
    session: Session,
    tracker: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;

    ExtendedTracker::get(Some(tracker.into_inner()), None, &user)
        .await?
        .delete()
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/api/jobs/{job_id}/trackers/{tracker_id}")]
async fn delete_job(session: Session, path: web::Path<(u32, u32)>) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job_id, tracker_id) = path.into_inner();

    ExtendedTracker::get(Some(tracker_id), Some(job_id), &user)
        .await?
        .delete()
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(create_standalone);
    cfg.service(create_job);

    cfg.service(create_many_standalone);
    cfg.service(create_many_job);

    cfg.service(get_standalone);
    cfg.service(get_job);

    cfg.service(get_many_standalone);
    cfg.service(get_many_job);

    cfg.service(get_time_worked_standalone);
    cfg.service(get_time_worked_job);

    cfg.service(update_standalone);
    cfg.service(update_job);

    cfg.service(delete_standalone);
    cfg.service(delete_job);
}
