use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::clinics::get_clinic_by_slug;
use crate::db::doctors::get_doctors_by_clinic;
use crate::ApiContext;

#[derive(serde::Serialize)]
pub struct DoctorResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub specialization: Option<String>,
}

pub async fn list_doctors(
    State(ctx): State<Arc<ApiContext>>,
    Path(slug): Path<String>,
) -> Result<Json<Vec<DoctorResponse>>, StatusCode> {
    let clinic = get_clinic_by_slug(&ctx.db, &slug)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let doctors = get_doctors_by_clinic(&ctx.db, clinic.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = doctors
        .into_iter()
        .map(|d| DoctorResponse {
            id: d.id,
            first_name: d.first_name,
            last_name: d.last_name,
            specialization: d.specialization,
        })
        .collect();

    Ok(Json(response))
}