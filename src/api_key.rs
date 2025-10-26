use {
    crate::{
        entities::api_key::{Entity, Model},
        error::ToErr,
        AppData,
    },
    actix_web::{error::ErrorForbidden, Error},
    argon2::{Algorithm, Argon2, Params, PasswordHash, Version},
    base64::{engine::general_purpose, Engine},
    chrono::Utc,
    password_hash::Salt,
    rand::{distr::Alphanumeric, rng, Rng, RngCore},
    sea_orm::{entity::prelude::*, IntoActiveModel, Set},
};

lazy_static::lazy_static! {
    static ref HASHER: Argon2<'static> = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(4096, Params::DEFAULT_T_COST, Params::DEFAULT_P_COST, None).unwrap(),
    );
}

impl Entity {
    /// # Returns
    /// `(hash: String, key: String)`
    pub fn generate() -> Result<(String, String), password_hash::Error> {
        let key = rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect::<Vec<char>>();

        let display = key
            .chunks(6)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("-");

        let hash = Self::hash(&String::from_iter(key))?;
        Ok((hash, display))
    }

    fn hash(password: &str) -> Result<String, password_hash::Error> {
        let mut salt_buffer = [0u8; Salt::RECOMMENDED_LENGTH];
        rng().fill_bytes(&mut salt_buffer);
        let salt_string = general_purpose::STANDARD_NO_PAD.encode(salt_buffer);
        let salt = Salt::from_b64(salt_string.as_str())?;
        let hash = argon2::PasswordHasher::hash_password(&*HASHER, password.as_ref(), salt)?;
        Ok(hash.serialize().to_string())
    }
}

impl Model {
    pub async fn verify(&self, key: &str) -> Result<(), Error> {
        if self.disabled
            || self
                .valid_until
                .map(|date| date < Utc::now())
                .unwrap_or(false)
        {
            return Err(ErrorForbidden("Forbidden"));
        }
        let key = key.replace("-", "");
        let hash = PasswordHash::new(&self.key).unwrap();
        hash.verify_password(&[&*HASHER], key)
            .map_err(|_| ErrorForbidden("Forbidden"))?;
        self.update_last_used().await
    }

    async fn update_last_used(&self) -> Result<(), Error> {
        let now = Utc::now();
        if let Some(last) = self.last_used {
            if last + AppData::get().api_key_duration > now {
                return Ok(());
            }
        }

        let mut model = self.clone().into_active_model();
        model.last_used = Set(Some(now));
        model
            .update(&AppData::get().conn)
            .await
            .to_err()
            .map(|_| ())
    }
}
