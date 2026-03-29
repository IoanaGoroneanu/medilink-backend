use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

use crate::auth::patient_auth::AuthenticatedPatient;
use crate::db::appointments::{CreateAppointmentData, create_appointment};
use crate::ApiContext;

#[derive(serde::Deserialize)]
pub struct BookAppointmentRequest {
    pub doctor_id: Uuid,
    pub appointment_time: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(serde::Serialize)]
pub struct BookAppointmentResponse {
    pub id: String,
}

pub async fn book_appointment(
    State(ctx): State<Arc<ApiContext>>,
    patient: AuthenticatedPatient,
    Json(payload): Json<BookAppointmentRequest>,
) -> Result<Json<BookAppointmentResponse>, StatusCode> {
    // Verify the doctor belongs to the same clinic as the patient
    let doctor = sqlx::query!(
        r#"SELECT clinic_id FROM doctors WHERE id = $1"#,
        payload.doctor_id
    )
    .fetch_optional(&ctx.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    if doctor.clinic_id != patient.clinic_id {
        return Err(StatusCode::FORBIDDEN);
    }

    if payload.appointment_time < Utc::now() {
        return Err(StatusCode::BAD_REQUEST);
    }

let id = create_appointment(&ctx.db, CreateAppointmentData {
    clinic_id: patient.clinic_id,
    doctor_id: payload.doctor_id,
    patient_id: patient.patient_id,
    appointment_time: payload.appointment_time,
    notes: payload.notes,
})
.await
.map_err(|e| match e {
    sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
        StatusCode::CONFLICT
    }
    _ => StatusCode::INTERNAL_SERVER_ERROR,
})?;

    Ok(Json(BookAppointmentResponse { id: id.to_string() }))
}

