use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

use crate::auth::patient_auth::AuthenticatedPatient;
use crate::db::appointments::get_appointments_by_patient;
use crate::ApiContext;

#[derive(serde::Serialize)]
pub struct AppointmentResponse {
    pub id: Uuid,
    pub doctor_id: Uuid,
    pub appointment_time: DateTime<Utc>,
    pub status: String,
    pub notes: Option<String>,
}

pub async fn get_appointments(
    State(ctx): State<Arc<ApiContext>>,
    patient: AuthenticatedPatient,
) -> Result<Json<Vec<AppointmentResponse>>, StatusCode> {
    let appointments = get_appointments_by_patient(&ctx.db, patient.patient_id, patient.clinic_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = appointments
        .into_iter()
        .map(|a| AppointmentResponse {
            id: a.id,
            doctor_id: a.doctor_id,
            appointment_time: a.appointment_time,
            status: a.status,
            notes: a.notes,
        })
        .collect();

    Ok(Json(response))
}