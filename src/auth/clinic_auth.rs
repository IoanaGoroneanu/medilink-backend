use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode, header},
};
use std::sync::Arc;
use uuid::Uuid;

use crate::ApiContext;
use super::verify_clinic_token;

pub struct AuthenticatedClinic {
    pub clinic_id: Uuid,
    pub slug: String,
}

impl FromRequestParts<Arc<ApiContext>> for AuthenticatedClinic {
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

        let claims = verify_clinic_token(token, &state.jwt_secret)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthenticatedClinic {
            clinic_id: claims.sub,
            slug: claims.slug,
        })
    }
}