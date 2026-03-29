use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod clinic_auth;
pub mod patient_auth;

#[derive(Serialize, Deserialize)]
pub struct ClinicClaims {
    // clinic id
    pub sub: Uuid,
    // clinic slug
    pub slug: String,
    // expiry timestamp
    pub exp: usize,     
}

pub fn generate_clinic_token(
    clinic_id: Uuid,
    slug: &str,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiry = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("expected valid timestamp for token expiry time")
        .timestamp() as usize;

    let claims = ClinicClaims {
        sub: clinic_id,
        slug: slug.to_string(),
        exp: expiry,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_clinic_token(
    token: &str,
    secret: &str,
) -> Result<ClinicClaims, jsonwebtoken::errors::Error> {
    let token_data = decode::<ClinicClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

#[derive(Serialize, Deserialize)]
pub struct PatientClaims {
    // patient id
    pub sub: Uuid,      
    pub clinic_id: Uuid,
    pub exp: usize,
}

pub fn generate_patient_token(
    patient_id: Uuid,
    clinic_id: Uuid,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiry = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = PatientClaims {
        sub: patient_id,
        clinic_id,
        exp: expiry,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_patient_token(
    token: &str,
    secret: &str,
) -> Result<PatientClaims, jsonwebtoken::errors::Error> {
    let token_data = decode::<PatientClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}