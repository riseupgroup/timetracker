use std::{cell::UnsafeCell, mem::MaybeUninit};

static APP_DATA: InitOnce<AppData> = InitOnce::new();

struct InitOnce<T>(UnsafeCell<MaybeUninit<T>>);

impl<T> InitOnce<T> {
    pub const fn new() -> Self {
        Self(UnsafeCell::new(MaybeUninit::uninit()))
    }

    pub unsafe fn init(&self, data: T) {
        *self.0.get() = MaybeUninit::new(data);
    }

    pub fn get(&self) -> &T {
        unsafe { (*self.0.get()).assume_init_ref() }
    }
}

unsafe impl<T> Send for InitOnce<T> {}
unsafe impl<T> Sync for InitOnce<T> {}

pub struct AppData {
    pub conn: sea_orm::DbConn,
    pub authentication_service: authentication_service::Client,
    pub api_key_duration: chrono::Duration,
}

impl AppData {
    pub async fn new() -> Self {
        let authentication_service = {
            let server_key = std::env::var("AUTH_SERVER_KEY")
                .unwrap_or_else(|_| String::from("auth_server.pem"));

            let private_key =
                std::env::var("PRIVATE_KEY").unwrap_or_else(|_| String::from("private.pem"));

            let server_key = match std::fs::read(&*shellexpand::tilde(&server_key)) {
                Ok(x) => x,
                Err(err) => panic!("Unable to open {server_key:?}: {err:?}"),
            };

            let private_key = match std::fs::read(&*shellexpand::tilde(&private_key)) {
                Ok(x) => x,
                Err(err) => panic!("Unable to open {private_key:?}: {err:?}"),
            };

            let host = std::env::var("AUTH_SERVER_HOST")
                .expect("Missing environment variable AUTH_SERVER_HOST");

            let server_id = std::env::var("AUTH_SERVER_ID")
                .expect("Missing environment variable AUTH_SERVER_ID")
                .parse()
                .expect("Invalid environment variable AUTH_SERVER_ID");

            authentication_service::Client::new(server_id, &private_key, host, &server_key).unwrap()
        };

        let api_key_duration = match std::env::var("API_KEY_REFRESH_LAST_USED_AFTER") {
            Ok(duration_string) => {
                chrono::Duration::from_std(parse_duration::parse(&duration_string).expect("Error parsing environment variable API_KEY_REFRESH_LAST_USED_AFTER, format after ISO 8601")).expect("Error parsing environment variable API_KEY_REFRESH_LAST_USED_AFTER: out of range")
            },
            Err(_) => {
                log::warn!("Missing environment variable API_KEY_REFRESH_LAST_USED_AFTER; Using default of 5 seconds");
                chrono::Duration::seconds(5)
            }
        };

        let database_url =
            std::env::var("DATABASE_URL").expect("Missing environment variable DATABASE_URL");

        let conn = sea_orm::Database::connect(&database_url)
            .await
            .expect("Unable to connect to database");
        <migration::Migrator as migration::MigratorTrait>::up(&conn, None)
            .await
            .expect("Failed to run migrations");

        Self {
            conn,
            authentication_service,
            api_key_duration,
        }
    }

    pub async unsafe fn init() {
        APP_DATA.init(Self::new().await)
    }

    pub fn get() -> &'static Self {
        APP_DATA.get()
    }
}
