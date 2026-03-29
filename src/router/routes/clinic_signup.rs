use axum::{extract::State, http::StatusCode, Json};
use regex::Regex;
use std::sync::Arc;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};

use crate::db::clinics::{create_clinic, CreateClinicData};
use crate::ApiContext;

#[derive(serde::Deserialize)]
pub struct ClinicSignupRequest {
    pub clinic_name: String,
    pub slug: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct ClinicSignupResponse {
    pub id: String,
}

pub async fn clinic_signup(
    State(ctx): State<Arc<ApiContext>>,
    Json(payload): Json<ClinicSignupRequest>,
) -> Result<Json<ClinicSignupResponse>, StatusCode> {
    let slug_regex = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
    if !slug_regex.is_match(&payload.slug) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let id = create_clinic(&ctx.db, CreateClinicData {
        clinic_name: payload.clinic_name,
        slug: payload.slug,
        email: payload.email,
        password_hash,
    })
    .await
    .map_err(|e| {
        match e {
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
                StatusCode::CONFLICT  // slug or email already taken
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    })?;

    Ok(Json(ClinicSignupResponse { id: id.to_string() }))
}