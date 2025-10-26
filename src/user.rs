use {
    crate::{error::ToErr, AppData, Authentication},
    actix_web::{
        get,
        http::header,
        post,
        web::{self},
        Error, HttpRequest, HttpResponse, Responder,
    },
    serde::Deserialize,
};

#[get("/auth")]
async fn auth_redirect() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .append_header((
            header::LOCATION,
            AppData::get().authentication_service.get_redirect_url(),
        ))
        .finish()
}

#[derive(Deserialize)]
struct AuthServerId {
    id: Option<u64>,
}

#[get("/auth/auth_server")]
async fn auth_server_login(
    req: HttpRequest,
    auth: Authentication,
    id: web::Query<AuthServerId>,
) -> Result<impl Responder, Error> {
    match id.into_inner().id {
        Some(id) => {
            let user = AppData::get()
                .authentication_service
                .query_authentication_request(id)
                .await
                .to_err()?;

            auth.set(user.id, user.display_name);

            let mut response = HttpResponse::Found();
            match req.cookie("path") {
                Some(cookie) => response.append_header(("location", cookie.value())),
                None => response.append_header(("location", "/")),
            };
            Ok(response.finish())
        }
        None => Ok(HttpResponse::Found()
            .append_header((
                "location",
                AppData::get().authentication_service.get_redirect_url(),
            ))
            .finish()),
    }
}

#[get("/auth/user")]
async fn get_user(auth: Authentication) -> Result<impl Responder, Error> {
    auth.take().map(|user| HttpResponse::Ok().json(user))
}

#[post("/auth/logout")]
async fn logout(auth: Authentication) -> Result<impl Responder, Error> {
    auth.unset();
    Ok(HttpResponse::Ok().finish())
}

#[get("/auth/users/{id}/picture")]
async fn get_profile_picture(id: web::Path<u32>) -> Result<impl Responder, Error> {
    let id = id.into_inner();
    Ok(HttpResponse::PermanentRedirect()
        .append_header((
            header::LOCATION,
            format!(
                "https://{}/api/profiles/{id}/picture",
                AppData::get().authentication_service.host()
            ),
        ))
        .finish())
}

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(auth_redirect);
    cfg.service(auth_server_login);
    cfg.service(get_user);
    cfg.service(logout);
    cfg.service(get_profile_picture);
}
