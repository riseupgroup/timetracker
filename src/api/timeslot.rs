use {
    crate::{
        entities::timeslot,
        timeslot::{
            get_units, NewTimeslot, TimeslotFilterParams, TimeslotSearchParams, UpdateTimeslot,
        },
        tracker::ExtendedTracker,
        user::SessionUser,
    },
    actix_session::Session,
    actix_web::{delete, get, patch, post, put, web, Error, HttpResponse},
    serde_qs::actix::QsQuery,
};

#[post("/api/trackers/{id}/timeslots")]
async fn create_tracker(
    session: Session,
    new_timeslot: web::Json<NewTimeslot>,
    tracker: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let timeslot = timeslot::Model::create(
        &user,
        new_timeslot.into_inner(),
        Some(tracker.into_inner()),
        None,
    )
    .await?;
    Ok(HttpResponse::Created().json(timeslot))
}

#[post("/api/jobs/{job_id}/trackers/{trackers_id}/timeslots")]
async fn create_job_tracker(
    session: Session,
    new_timeslot: web::Json<NewTimeslot>,
    path: web::Path<(u32, u32)>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job, tracker) = path.into_inner();
    let timeslot =
        timeslot::Model::create(&user, new_timeslot.into_inner(), Some(tracker), Some(job)).await?;
    Ok(HttpResponse::Created().json(timeslot))
}

#[post("/api/jobs/{id}/timeslots")]
async fn create_job(
    session: Session,
    new_timeslot: web::Json<NewTimeslot>,
    job: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let timeslot = timeslot::Model::create(
        &user,
        new_timeslot.into_inner(),
        None,
        Some(job.into_inner()),
    )
    .await?;
    Ok(HttpResponse::Created().json(timeslot))
}

#[put("/api/trackers/{id}/timeslots")]
async fn create_many_tracker(
    session: Session,
    new_timeslots: web::Json<Vec<NewTimeslot>>,
    tracker: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let timeslots = timeslot::Model::create_many(
        &user,
        new_timeslots.into_inner(),
        Some(tracker.into_inner()),
        None,
    )
    .await?;
    Ok(HttpResponse::Created().json(timeslots))
}

#[put("/api/jobs/{job_id}/trackers/{trackers_id}/timeslots")]
async fn create_many_job_tracker(
    session: Session,
    new_timeslots: web::Json<Vec<NewTimeslot>>,
    path: web::Path<(u32, u32)>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job, tracker) = path.into_inner();
    let timeslots =
        timeslot::Model::create_many(&user, new_timeslots.into_inner(), Some(tracker), Some(job))
            .await?;
    Ok(HttpResponse::Created().json(timeslots))
}

#[put("/api/jobs/{id}/timeslots")]
async fn create_many_job(
    session: Session,
    new_timeslots: web::Json<Vec<NewTimeslot>>,
    job: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let timeslots = timeslot::Model::create_many(
        &user,
        new_timeslots.into_inner(),
        None,
        Some(job.into_inner()),
    )
    .await?;
    Ok(HttpResponse::Created().json(timeslots))
}

#[get("/api/timeslots/{id}")]
async fn get_standalone(session: Session, timeslot: web::Path<u64>) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let timeslot = timeslot::Model::get(timeslot.into_inner(), None, &user, None).await?;
    Ok(HttpResponse::Ok().json(timeslot))
}

#[get("/api/trackers/{tracker_id}/timeslots/current")]
async fn get_tracker_current(
    session: Session,
    tracker: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let tracker = tracker.into_inner();
    let tracker = ExtendedTracker::get(Some(tracker), None, &user)
        .await?
        .tracker;
    let timeslot = tracker.get_current_timeslot().await?;
    Ok(HttpResponse::Ok().json(timeslot))
}

#[get("/api/trackers/{tracker_id}/timeslots/{timeslot_id}")]
async fn get_tracker(session: Session, path: web::Path<(u32, u64)>) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (tracker, timeslot) = path.into_inner();
    let timeslot = timeslot::Model::get(timeslot, Some(tracker), &user, None).await?;
    Ok(HttpResponse::Ok().json(timeslot))
}

#[get("/api/jobs/{job_id}/trackers/{tracker_id}/timeslots/current")]
async fn get_job_tracker_current(
    session: Session,
    path: web::Path<(u32, u32)>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job, tracker) = path.into_inner();
    let tracker = ExtendedTracker::get(Some(tracker), Some(job), &user)
        .await?
        .tracker;
    let timeslot = tracker.get_current_timeslot().await?;
    Ok(HttpResponse::Ok().json(timeslot))
}

#[get("/api/jobs/{job_id}/trackers/{tracker_id}/timeslots/{timeslot_id}")]
async fn get_job_tracker(
    session: Session,
    path: web::Path<(u32, u32, u64)>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job, tracker, timeslot) = path.into_inner();

    let timeslot = timeslot::Model::get(timeslot, Some(tracker), &user, Some(job)).await?;
    Ok(HttpResponse::Ok().json(timeslot))
}

#[get("/api/jobs/{job_id}/timeslots/current")]
async fn get_job_current(session: Session, job: web::Path<u32>) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let tracker = ExtendedTracker::get(None, Some(job.into_inner()), &user)
        .await?
        .tracker;
    let timeslot = tracker.get_current_timeslot().await?;
    Ok(HttpResponse::Ok().json(timeslot))
}

#[get("/api/jobs/{job_id}/timeslots/{timeslot_id}")]
async fn get_job(session: Session, path: web::Path<(u32, u64)>) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job, timeslot) = path.into_inner();

    let timeslot = timeslot::Model::get(timeslot, None, &user, Some(job)).await?;
    Ok(HttpResponse::Ok().json(timeslot))
}

#[get("/api/trackers/{id}/timeslots")]
async fn get_many_tracker(
    session: Session,
    tracker: web::Path<u32>,
    params: web::Query<TimeslotSearchParams>,
) -> Result<HttpResponse, Error> {
    let params = params.into_inner();
    let user = SessionUser::try_from(&session)?;

    let timeslots =
        timeslot::Model::get_many(&user, params, Some(tracker.into_inner()), None).await?;
    Ok(HttpResponse::Ok().json(timeslots))
}

#[get("/api/jobs/{job_id}/trackers/{tracker_id}/timeslots")]
async fn get_many_job_tracker(
    session: Session,
    path: web::Path<(u32, u32)>,
    params: web::Query<TimeslotSearchParams>,
) -> Result<HttpResponse, Error> {
    let params = params.into_inner();
    let user = SessionUser::try_from(&session)?;
    let (job, tracker) = path.into_inner();

    let timeslots = timeslot::Model::get_many(&user, params, Some(tracker), Some(job)).await?;
    Ok(HttpResponse::Ok().json(timeslots))
}

#[get("/api/jobs/{id}/timeslots")]
async fn get_many_job(
    session: Session,
    job: web::Path<u32>,
    params: web::Query<TimeslotSearchParams>,
) -> Result<HttpResponse, Error> {
    let params = params.into_inner();
    let user = SessionUser::try_from(&session)?;
    let timeslots = timeslot::Model::get_many(&user, params, None, Some(job.into_inner())).await?;
    Ok(HttpResponse::Ok().json(timeslots))
}

#[get("/api/trackers/{id}/units")]
async fn get_units_tracker(
    session: Session,
    tracker: web::Path<u32>,
    params: QsQuery<TimeslotFilterParams>,
) -> Result<HttpResponse, Error> {
    let params = params.into_inner();
    let user = SessionUser::try_from(&session)?;

    let tracker = ExtendedTracker::get(Some(tracker.into_inner()), None, &user).await?;
    let units = get_units(tracker.tracker, params).await?;

    Ok(HttpResponse::Ok().json(units))
}

#[get("/api/jobs/{job_id}/trackers/{tracker_id}/units")]
async fn get_units_job_tracker(
    session: Session,
    path: web::Path<(u32, u32)>,
    params: QsQuery<TimeslotFilterParams>,
) -> Result<HttpResponse, Error> {
    let params = params.into_inner();
    let user = SessionUser::try_from(&session)?;
    let (job, tracker) = path.into_inner();

    let tracker = ExtendedTracker::get(Some(tracker), Some(job), &user).await?;
    let units = get_units(tracker.tracker, params).await?;

    Ok(HttpResponse::Ok().json(units))
}

#[get("/api/jobs/{job_id}/units")]
async fn get_units_job(
    session: Session,
    job: web::Path<u32>,
    params: QsQuery<TimeslotFilterParams>,
) -> Result<HttpResponse, Error> {
    let params = params.into_inner();
    let user = SessionUser::try_from(&session)?;

    let tracker = ExtendedTracker::get(None, Some(job.into_inner()), &user).await?;
    let units = get_units(tracker.tracker, params).await?;

    Ok(HttpResponse::Ok().json(units))
}

#[patch("/api/timeslots/{id}")]
async fn update_standalone(
    session: Session,
    timeslot: web::Path<u64>,
    update: web::Json<UpdateTimeslot>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;

    let (timeslot, tracker) =
        timeslot::Model::get_with_tracker(timeslot.into_inner(), None, &user, None).await?;
    let timeslot = timeslot.update(update.into_inner(), tracker).await?;

    Ok(HttpResponse::Ok().json(timeslot))
}

#[patch("/api/trackers/{tracker_id}/timeslots/current")]
async fn update_tracker_current(
    session: Session,
    tracker: web::Path<u32>,
    update: web::Json<UpdateTimeslot>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;

    let tracker = ExtendedTracker::get(Some(tracker.into_inner()), None, &user)
        .await?
        .tracker;
    let timeslot = tracker.get_current_timeslot().await?;
    let timeslot = timeslot.update(update.into_inner(), tracker).await?;

    Ok(HttpResponse::Ok().json(timeslot))
}

#[patch("/api/trackers/{tracker_id}/timeslots/{timeslot_id}")]
async fn update_tracker(
    session: Session,
    path: web::Path<(u32, u64)>,
    update: web::Json<UpdateTimeslot>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (tracker, timeslot) = path.into_inner();

    let (timeslot, tracker) =
        timeslot::Model::get_with_tracker(timeslot, Some(tracker), &user, None).await?;
    let timeslot = timeslot.update(update.into_inner(), tracker).await?;

    Ok(HttpResponse::Ok().json(timeslot))
}

#[patch("/api/jobs/{job_id}/trackers/{tracker_id}/timeslots/current")]
async fn update_job_tracker_current(
    session: Session,
    path: web::Path<(u32, u32)>,
    update: web::Json<UpdateTimeslot>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job, tracker) = path.into_inner();

    let tracker = ExtendedTracker::get(Some(tracker), Some(job), &user)
        .await?
        .tracker;
    let timeslot = tracker.get_current_timeslot().await?;
    let timeslot = timeslot.update(update.into_inner(), tracker).await?;

    Ok(HttpResponse::Ok().json(timeslot))
}

#[patch("/api/jobs/{job_id}/trackers/{tracker_id}/timeslots/{timeslot_id}")]
async fn update_job_tracker(
    session: Session,
    path: web::Path<(u32, u32, u64)>,
    update: web::Json<UpdateTimeslot>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job, tracker, timeslot) = path.into_inner();

    let (timeslot, tracker) =
        timeslot::Model::get_with_tracker(timeslot, Some(tracker), &user, Some(job)).await?;
    let timeslot = timeslot.update(update.into_inner(), tracker).await?;

    Ok(HttpResponse::Ok().json(timeslot))
}

#[patch("/api/jobs/{job_id}/timeslots/current")]
async fn update_job_current(
    session: Session,
    job: web::Path<u32>,
    update: web::Json<UpdateTimeslot>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;

    let tracker = ExtendedTracker::get(None, Some(job.into_inner()), &user)
        .await?
        .tracker;
    let timeslot = tracker.get_current_timeslot().await?;
    let timeslot = timeslot.update(update.into_inner(), tracker).await?;

    Ok(HttpResponse::Ok().json(timeslot))
}

#[patch("/api/jobs/{job_id}/timeslots/{timeslot_id}")]
async fn update_job(
    session: Session,
    path: web::Path<(u32, u64)>,
    update: web::Json<UpdateTimeslot>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job, timeslot) = path.into_inner();

    let (timeslot, tracker) =
        timeslot::Model::get_with_tracker(timeslot, None, &user, Some(job)).await?;
    let timeslot = timeslot.update(update.into_inner(), tracker).await?;

    Ok(HttpResponse::Ok().json(timeslot))
}

#[delete("/api/timeslots/{id}")]
async fn delete_standalone(
    session: Session,
    timeslot: web::Path<u64>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;

    let timeslot = timeslot::Model::get(timeslot.into_inner(), None, &user, None).await?;
    timeslot.delete().await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/api/trackers/{tracker_id}/timeslots/current")]
async fn delete_tracker_current(
    session: Session,
    tracker: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;

    let tracker = ExtendedTracker::get(Some(tracker.into_inner()), None, &user)
        .await?
        .tracker;
    tracker.get_current_timeslot().await?.delete().await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/api/trackers/{tracker_id}/timeslots/{timeslot_id}")]
async fn delete_tracker(
    session: Session,
    path: web::Path<(u32, u64)>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (tracker, timeslot) = path.into_inner();

    let timeslot = timeslot::Model::get(timeslot, Some(tracker), &user, None).await?;
    timeslot.delete().await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/api/jobs/{job_id}/trackers/{tracker_id}/timeslots/current")]
async fn delete_job_tracker_current(
    session: Session,
    path: web::Path<(u32, u32)>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job, tracker) = path.into_inner();

    let tracker = ExtendedTracker::get(Some(tracker), Some(job), &user)
        .await?
        .tracker;
    tracker.get_current_timeslot().await?.delete().await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/api/jobs/{job_id}/trackers/{tracker_id}/timeslots/{timeslot_id}")]
async fn delete_job_tracker(
    session: Session,
    path: web::Path<(u32, u32, u64)>,
) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job, tracker, timeslot) = path.into_inner();

    let timeslot = timeslot::Model::get(timeslot, Some(tracker), &user, Some(job)).await?;
    timeslot.delete().await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/api/jobs/{job_id}/timeslots/current")]
async fn delete_job_current(session: Session, job: web::Path<u32>) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;

    let tracker = ExtendedTracker::get(None, Some(job.into_inner()), &user)
        .await?
        .tracker;
    tracker.get_current_timeslot().await?.delete().await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/api/jobs/{job_id}/timeslots/{timeslot_id}")]
async fn delete_job(session: Session, path: web::Path<(u32, u64)>) -> Result<HttpResponse, Error> {
    let user = SessionUser::try_from(&session)?;
    let (job, timeslot) = path.into_inner();

    let timeslot = timeslot::Model::get(timeslot, None, &user, Some(job)).await?;
    timeslot.delete().await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_tracker);
    cfg.service(create_job_tracker);
    cfg.service(create_job);

    cfg.service(create_many_tracker);
    cfg.service(create_many_job_tracker);
    cfg.service(create_many_job);

    cfg.service(get_standalone);
    cfg.service(get_tracker_current);
    cfg.service(get_tracker);
    cfg.service(get_job_tracker_current);
    cfg.service(get_job_tracker);
    cfg.service(get_job_current);
    cfg.service(get_job);

    cfg.service(get_many_tracker);
    cfg.service(get_many_job_tracker);
    cfg.service(get_many_job);

    cfg.service(get_units_tracker);
    cfg.service(get_units_job_tracker);
    cfg.service(get_units_job);

    cfg.service(update_standalone);
    cfg.service(update_tracker_current);
    cfg.service(update_tracker);
    cfg.service(update_job_tracker_current);
    cfg.service(update_job_tracker);
    cfg.service(update_job_current);
    cfg.service(update_job);

    cfg.service(delete_standalone);
    cfg.service(delete_tracker_current);
    cfg.service(delete_tracker);
    cfg.service(delete_job_tracker_current);
    cfg.service(delete_job_tracker);
    cfg.service(delete_job_current);
    cfg.service(delete_job);
}
