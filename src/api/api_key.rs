use {
    crate::{
        entities::{prelude::*, *},
        error::{MapToErr, ToErr},
        update_value::{UpdateOption, UpdateValue},
        AppData, Authentication,
    },
    actix_web::{
        delete,
        error::ErrorForbidden,
        get, patch, post,
        web::{self},
        Error, HttpResponse,
    },
    chrono::Utc,
    sea_orm::{
        prelude::DateTimeUtc, ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait,
        IntoActiveModel, ModelTrait, QueryFilter, Set,
    },
    serde::{Deserialize, Serialize},
};

#[get("/api/keys")]
async fn get_many(auth: Authentication) -> Result<HttpResponse, Error> {
    let user = auth.take()?;

    let keys = ApiKey::find()
        .filter(api_key::Column::Owner.eq(user.id))
        .all(&AppData::get().conn)
        .await
        .to_err()?;

    Ok(HttpResponse::Ok().json(keys))
}

#[get("/api/keys/{id}")]
async fn get(auth: Authentication, id: web::Path<u32>) -> Result<HttpResponse, Error> {
    let user = auth.take()?;

    let key = ApiKey::find_by_id(id.into_inner())
        .filter(api_key::Column::Owner.eq(user.id))
        .one(&AppData::get().conn)
        .await
        .to_err()?;

    Ok(HttpResponse::Ok().json(key))
}

#[derive(Serialize)]
struct CreateResponse {
    id: u32,
    key: String,
    name: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewApiKey {
    name: Option<String>,
    disabled: bool,
    valid_until: Option<DateTimeUtc>,
}

#[post("/api/keys")]
async fn create(
    auth: Authentication,
    new_key: web::Json<NewApiKey>,
) -> Result<HttpResponse, Error> {
    let user = auth.take_session()?;
    let new_key = new_key.into_inner();
    let (hash, key) = ApiKey::generate().to_err()?;

    let now = Utc::now();
    let api_key = api_key::ActiveModel {
        name: Set(new_key.name),
        owner: Set(user.id),
        disabled: Set(new_key.disabled),
        valid_until: Set(new_key.valid_until),
        added: Set(now),
        last_changed: Set(now),
        key: Set(hash),
        ..Default::default()
    }
    .insert(&AppData::get().conn)
    .await
    .to_err()?;

    Ok(HttpResponse::Created().json(CreateResponse {
        id: api_key.id,
        key,
        name: api_key.name,
    }))
}

#[delete("/api/keys/{id}")]
async fn delete(auth: Authentication, id: web::Path<u32>) -> Result<HttpResponse, Error> {
    let user = auth.take_session()?;
    let key = ApiKey::find_by_id(id.into_inner())
        .one(&AppData::get().conn)
        .await
        .map_to_err()?;

    if key.owner != user.id {
        return Err(ErrorForbidden("Forbidden"));
    }

    key.delete(&AppData::get().conn).await.to_err()?;
    Ok(HttpResponse::NoContent().finish())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateApiKey {
    #[serde(default)]
    name: UpdateOption<String>,
    #[serde(default)]
    disabled: UpdateValue<bool>,
    #[serde(default)]
    valid_until: UpdateOption<DateTimeUtc>,
    #[serde(default)]
    regenerate: bool,
}

#[patch("/api/keys/{id}")]
async fn update(
    auth: Authentication,
    id: web::Path<u32>,
    update_key: web::Json<UpdateApiKey>,
) -> Result<HttpResponse, Error> {
    let user = auth.take_session()?;
    let update_key = update_key.into_inner();

    let api_key = ApiKey::find_by_id(id.into_inner())
        .one(&AppData::get().conn)
        .await
        .map_to_err()?;

    if api_key.owner != user.id {
        return Err(ErrorForbidden("Forbidden"));
    }

    let mut update = api_key.clone().into_active_model();
    update.name = update_key.name.into();
    update.disabled = update_key.disabled.into();
    update.valid_until = update_key.valid_until.into();

    let mut response = None;

    if update_key.regenerate {
        let (hash, key) = ApiKey::generate().to_err()?;
        update.key = Set(hash);

        let name = match update.name.clone() {
            ActiveValue::Set(name) => name,
            _ => api_key.name,
        };

        update.last_changed = Set(Utc::now());

        response = Some(CreateResponse {
            id: api_key.id,
            key,
            name,
        });
    }

    update.update(&AppData::get().conn).await.to_err()?;

    match response {
        Some(response) => Ok(HttpResponse::Ok().json(response)),
        None => Ok(HttpResponse::Ok().finish()),
    }
}

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get);
    cfg.service(get_many);
    cfg.service(create);
    cfg.service(delete);
    cfg.service(update);
}
