use {
    crate::user::SessionUser,
    actix_session::Session,
    actix_web::{http::StatusCode, web, Error, HttpRequest, HttpResponse, Route},
};

const EXISTING: svelte_path_matcher::SveltePathMatcher<'static> =
    svelte_path_matcher::build_from_filesystem!("./frontend/src/");
const NO_LOGIN: &[&str] = &["", "login", "about"];

#[derive(Debug, serde::Deserialize)]
struct Query {
    path: Option<String>,
}

#[cfg(debug_assertions)]
mod debug {
    use {super::*, actix_files::NamedFile};

    fn get_file(path: &str, req: &HttpRequest) -> HttpResponse {
        match NamedFile::open(format!("./frontend/build/{path}")) {
            Ok(file) => file.into_response(req),
            Err(_) => match NamedFile::open("./frontend/build/index.html") {
                Ok(file) => {
                    let mut res = file.into_response(req);
                    *res.status_mut() = StatusCode::NOT_FOUND;
                    res
                }
                Err(_) => HttpResponse::NotFound().finish(),
            },
        }
    }

    pub(super) async fn serve_file(
        req: HttpRequest,
        session: Session,
        query: web::Query<Query>,
    ) -> Result<HttpResponse, Error> {
        let path = req.path().trim_matches('/');

        match EXISTING.matches(path) {
            true if SessionUser::try_from(&session).is_ok() => {
                if path == "login" {
                    Ok(HttpResponse::TemporaryRedirect()
                        .append_header(("location", query.path.as_deref().unwrap_or("/")))
                        .finish())
                } else {
                    Ok(get_file("index.html", &req))
                }
            }
            true if NO_LOGIN.contains(&path) => Ok(get_file("index.html", &req)),
            true => Ok(HttpResponse::TemporaryRedirect()
                .append_header(("location", format!("/login?path={}", req.uri())))
                .finish()),
            false => Ok(get_file(path, &req)),
        }
    }
}

#[cfg(debug_assertions)]
use debug::serve_file;

#[cfg(not(debug_assertions))]
mod release {
    use {
        super::*,
        actix_web::http::{header::ContentType, StatusCode},
    };

    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
    lazy_static::lazy_static! {
        static ref DATA: std::collections::HashMap<&'static str, static_files::Resource> = generate();
    }

    pub fn get_file(path: &str) -> HttpResponse {
        match DATA.get(path) {
            Some(file) => {
                let mut resp = HttpResponse::build(StatusCode::OK);
                resp.content_type(file.mime_type);
                resp.body(file.data)
            }
            None => match DATA.get("index.html") {
                Some(file) => {
                    let mut resp = HttpResponse::build(StatusCode::NOT_FOUND);
                    resp.content_type(ContentType::html());
                    resp.body(file.data)
                }
                None => HttpResponse::NotFound().finish(),
            },
        }
    }

    pub(super) async fn serve_file(
        req: HttpRequest,
        session: Session,
        query: web::Query<Query>,
    ) -> Result<HttpResponse, Error> {
        let path = req.path().trim_matches('/');

        match EXISTING.matches(path) {
            true if SessionUser::try_from(&session).is_ok() => {
                if path == "login" {
                    Ok(HttpResponse::TemporaryRedirect()
                        .append_header((
                            "location",
                            query.path.as_ref().map(String::as_str).unwrap_or("/"),
                        ))
                        .finish())
                } else {
                    Ok(get_file("index.html"))
                }
            }
            true if NO_LOGIN.contains(&path) => Ok(get_file("index.html")),
            true => Ok(HttpResponse::TemporaryRedirect()
                .append_header(("location", format!("/login?path={}", req.uri())))
                .finish()),
            false => Ok(get_file(path)),
        }
    }
}

#[cfg(not(debug_assertions))]
use release::serve_file;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.default_service(Route::new().to(serve_file));
}
