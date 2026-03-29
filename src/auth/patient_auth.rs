use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
};
use std::sync::Arc;
use uuid::Uuid;

use crate::ApiContext;
use super::verify_patient_token;

pub struct AuthenticatedPatient {
    pub patient_id: Uuid,
    pub clinic_id: Uuid,
}

impl FromRequestParts<Arc<ApiContext>> for AuthenticatedPatient {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ApiContext>,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let claims = verify_patient_token(token, &state.jwt_secret)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthenticatedPatient {
            patient_id: claims.sub,
            clinic_id: claims.clinic_id,
        })
    }
}