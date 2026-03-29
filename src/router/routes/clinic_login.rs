use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;

use crate::auth::generate_clinic_token;
use crate::db::clinics::get_clinic_by_email;
use crate::ApiContext;

#[derive(serde::Deserialize)]
pub struct ClinicLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct ClinicLoginResponse {
    pub token: String,
}

pub async fn clinic_login(
    State(ctx): State<Arc<ApiContext>>,
    Json(payload): Json<ClinicLoginRequest>,
) -> Result<Json<ClinicLoginResponse>, StatusCode> {
    let clinic = get_clinic_by_email(&ctx.db, &payload.email.trim().to_lowercase())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let parsed_hash = PasswordHash::new(&clinic.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token = generate_clinic_token(clinic.id, &clinic.slug, &ctx.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ClinicLoginResponse { token }))
}