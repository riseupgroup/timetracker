use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::SameSite, App, HttpServer};

mod app_data;
mod error;
mod frontend;
mod user;

pub(crate) use app_data::AppData;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    #[cfg(debug_assertions)]
    {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    #[cfg(not(debug_assertions))]
    {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    #[cfg(debug_assertions)]
    let secret_key = {
        const KEY: [u8; 64] = [
            214, 235, 254, 208, 2, 104, 84, 123, 188, 216, 236, 30, 146, 156, 213, 15, 147, 35,
            130, 11, 141, 202, 130, 20, 211, 63, 205, 136, 81, 195, 0, 80, 80, 42, 206, 22, 171,
            158, 238, 37, 98, 227, 20, 175, 117, 41, 12, 238, 110, 162, 252, 129, 230, 118, 61,
            122, 20, 108, 234, 140, 246, 149, 111, 174,
        ];
        actix_web::cookie::Key::from(&KEY)
    };
    #[cfg(not(debug_assertions))]
    let secret_key = actix_web::cookie::Key::generate();

    unsafe {
        AppData::init().await;
    }

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap({
                #[cfg(not(debug_assertions))]
                let cookie_secure = true;
                #[cfg(debug_assertions)]
                let cookie_secure = false;

                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Strict)
                    .cookie_secure(cookie_secure)
                    .build()
            })
            .configure(user::init)
            .configure(frontend::init)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
