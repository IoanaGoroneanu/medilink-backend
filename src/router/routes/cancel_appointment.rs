use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::auth::patient_auth::AuthenticatedPatient;
use crate::db::appointments::cancel_appointment;
use crate::ApiContext;

#[derive(serde::Serialize)]
pub struct CancelAppointmentResponse {
    pub status: String,
}

pub async fn cancel_appointment_handler(
    State(ctx): State<Arc<ApiContext>>,
    patient: AuthenticatedPatient,
    Path(appointment_id): Path<Uuid>,
) -> Result<Json<CancelAppointmentResponse>, StatusCode> {
    let cancelled = cancel_appointment(&ctx.db, appointment_id, patient.patient_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !cancelled {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(CancelAppointmentResponse {
        status: "cancelled".to_string(),
    }))
}