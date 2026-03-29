use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::sync::Arc;

use crate::auth::generate_patient_token;
use crate::db::clinics::get_clinic_by_slug;
use crate::db::patients::get_patient_by_email;
use crate::ApiContext;

#[derive(serde::Deserialize)]
pub struct PatientLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct PatientLoginResponse {
    pub token: String,
}

pub async fn patient_login(
    State(ctx): State<Arc<ApiContext>>,
    Path(slug): Path<String>,
    Json(payload): Json<PatientLoginRequest>,
) -> Result<Json<PatientLoginResponse>, StatusCode> {
    let clinic = get_clinic_by_slug(&ctx.db, &slug)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let patient = get_patient_by_email(
        &ctx.db,
        &payload.email.trim().to_lowercase(),
        clinic.id,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    let parsed_hash = PasswordHash::new(&patient.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token = generate_patient_token(patient.id, clinic.id, &ctx.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(PatientLoginResponse { token }))
}