use {
    crate::{
        entities::job,
        job::{NewJob, UpdateJob},
        tracker::Timezone,
        Authentication,
    },
    actix_web::{delete, get, patch, post, put, web, Error, HttpResponse, Responder},
};

#[post("/api/jobs")]
async fn create(auth: Authentication, new_job: web::Json<NewJob>) -> Result<impl Responder, Error> {
    let user = auth.take()?;
    let job = job::Model::create(new_job.into_inner(), &user).await?;
    Ok(HttpResponse::Created().json(job))
}

#[put("/api/jobs")]
async fn create_many(
    auth: Authentication,
    new_jobs: web::Json<Vec<NewJob>>,
) -> Result<impl Responder, Error> {
    let user = auth.take()?;
    let jobs = job::Model::create_many(new_jobs.into_inner(), &user).await?;
    Ok(HttpResponse::Created().json(jobs))
}

#[get("/api/jobs/{id}")]
async fn get(
    auth: Authentication,
    job: web::Path<u32>,
    params: web::Query<Timezone>,
) -> Result<impl Responder, Error> {
    let user = auth.take()?;
    let job = job::Model::get_extended(job.into_inner(), &user, params.timezone).await?;
    Ok(HttpResponse::Ok().json(job))
}

#[get("/api/jobs")]
async fn get_many(auth: Authentication) -> Result<impl Responder, Error> {
    let user = auth.take()?;
    let jobs = job::Model::get_extended_many(&user).await?;
    Ok(HttpResponse::Ok().json(jobs))
}

#[patch("/api/jobs/{id}")]
async fn update(
    auth: Authentication,
    job: web::Path<u32>,
    update_job: web::Json<UpdateJob>,
) -> Result<impl Responder, Error> {
    let user = auth.take()?;
    let job = job::Model::update(
        update_job.into_inner(),
        job::Model::get(job.into_inner(), &user).await?,
    )
    .await?;
    Ok(HttpResponse::Ok().json(job))
}

#[delete("/api/jobs/{id}")]
async fn delete(auth: Authentication, job: web::Path<u32>) -> Result<impl Responder, Error> {
    let user = auth.take()?;
    job::Model::get(job.into_inner(), &user)
        .await?
        .delete()
        .await?;
    Ok(HttpResponse::NoContent())
}

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(create);
    cfg.service(create_many);
    cfg.service(get);
    cfg.service(get_many);
    cfg.service(update);
    cfg.service(delete);
}
