use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;

use crate::auth::clinic_auth::AuthenticatedClinic;
use crate::db::doctors::{create_doctor, CreateDoctorData};
use crate::ApiContext;

#[derive(serde::Deserialize)]
pub struct AddDoctorRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub specialization: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AddDoctorResponse {
    pub id: String,
}

pub async fn add_doctor(
    State(ctx): State<Arc<ApiContext>>,
    clinic: AuthenticatedClinic,
    Json(payload): Json<AddDoctorRequest>,
) -> Result<Json<AddDoctorResponse>, StatusCode> {
    let id = create_doctor(&ctx.db, CreateDoctorData {
        clinic_id: clinic.clinic_id,
        first_name: payload.first_name.trim().to_string(),
        last_name: payload.last_name.trim().to_string(),
        email: payload.email.trim().to_lowercase(),
        specialization: payload.specialization,
        phone_number: payload.phone_number,
    })
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(Json(AddDoctorResponse { id: id.to_string() }))
}