use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use axum::{
    Json, extract::{self, Path, State}, http::{HeaderMap, StatusCode, header}
};
use chrono::NaiveDate;
use rand_core::OsRng;
use std::{collections::HashSet, sync::Arc};

use crate::{ApiContext, db::{clinics::get_clinic_by_slug, patients::{CreatePatientData, create_patient}}};

#[derive(serde::Deserialize, Debug)]
pub struct PatientSignupRequest {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub gender: String,
    pub street_address: String,
    pub city: String,
    pub country: String,
    pub postcode: String,
}

#[derive(serde::Serialize, Debug)]
pub struct Response {
    status: String,
}

#[derive(Debug)]
struct SanitizedPatientSignupRequest {
    first_name: String,
    last_name: String,
    birth_date: String,
    email: String,
    password: String,
    phone_number: String,
    gender: String,
    street_address: String,
    city: String,
    country: String,
    postcode: String,
}

fn normalize_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn sanitize_and_validate_payload(
    payload: PatientSignupRequest,
) -> Result<SanitizedPatientSignupRequest, String> {
    let sanitized = SanitizedPatientSignupRequest {
        first_name: normalize_whitespace(payload.first_name.trim()),
        last_name: normalize_whitespace(payload.last_name.trim()),
        birth_date: payload.birth_date.trim().to_string(),
        email: payload.email.trim().to_lowercase(),
        password: payload.password,
        phone_number: payload
            .phone_number
            .trim()
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '+')
            .collect(),
        gender: payload.gender.trim().to_lowercase(),
        street_address: normalize_whitespace(payload.street_address.trim()),
        city: normalize_whitespace(payload.city.trim()),
        country: normalize_whitespace(payload.country.trim()),
        postcode: normalize_whitespace(payload.postcode.trim()).to_uppercase(),
    };

    if sanitized.first_name.is_empty() || sanitized.last_name.is_empty() {
        return Err("First name and last name are required".to_string());
    }
    if sanitized.first_name.len() > 64 || sanitized.last_name.len() > 64 {
        return Err("First name and last name are too long".to_string());
    }
    if sanitized.street_address.is_empty()
        || sanitized.city.is_empty()
        || sanitized.country.is_empty()
        || sanitized.postcode.is_empty()
    {
        return Err("Address fields are required".to_string());
    }
    if sanitized.street_address.len() > 120
        || sanitized.city.len() > 80
        || sanitized.country.len() > 80
        || sanitized.postcode.len() > 16
    {
        return Err("Address fields exceed max length".to_string());
    }
    if !is_valid_birth_date(&sanitized.birth_date) {
        return Err("Birth date must be a valid date in YYYY-MM-DD format".to_string());
    }
    if !is_valid_email(&sanitized.email) {
        return Err("Invalid email format".to_string());
    }
    if !is_valid_password(&sanitized.password) {
        return Err("Password must be at least 8 characters and include upper, lower, digit, and symbol".to_string());
    }
    if !is_valid_phone_number(&sanitized.phone_number) {
        return Err("Phone number must contain 10-15 digits and may start with +".to_string());
    }

    let valid_genders: HashSet<&str> = ["male", "female", "other", "prefer_not_to_say"]
        .into_iter()
        .collect();
    if !valid_genders.contains(sanitized.gender.as_str()) {
        return Err("Gender must be one of: male, female, other, prefer_not_to_say".to_string());
    }

    if !sanitized
        .postcode
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == ' ' || c == '-')
    {
        return Err("Postcode may only contain letters, digits, spaces, and -".to_string());
    }

    Ok(sanitized)
}

fn is_valid_email(email: &str) -> bool {
    if email.len() > 254 || email.starts_with('@') || email.ends_with('@') {
        return false;
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let local = parts[0];
    let domain = parts[1];
    !local.is_empty()
        && !domain.is_empty()
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
}

fn is_valid_password(password: &str) -> bool {
    if password.len() < 8 || password.len() > 128 {
        return false;
    }

    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| !c.is_ascii_alphanumeric());

    has_lower && has_upper && has_digit && has_symbol
}

fn is_valid_phone_number(phone_number: &str) -> bool {
    if phone_number.is_empty() {
        return false;
    }

    let normalized = phone_number.strip_prefix('+').unwrap_or(phone_number);
    normalized.len() >= 10
        && normalized.len() <= 15
        && normalized.chars().all(|c| c.is_ascii_digit())
}

fn is_valid_birth_date(value: &str) -> bool {
    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() != 3
        || parts[0].len() != 4
        || parts[1].len() != 2
        || parts[2].len() != 2
    {
        return false;
    }

    let year: i32 = match parts[0].parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let month: u32 = match parts[1].parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let day: u32 = match parts[2].parse() {
        Ok(v) => v,
        Err(_) => return false,
    };

    if !(1900..=2100).contains(&year) || !(1..=12).contains(&month) {
        return false;
    }

    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => return false,
    };

    (1..=max_day).contains(&day)
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub async fn patient_signup(
    State(ctx): State<Arc<ApiContext>>,
    Path(slug): Path<String>,
    headers: HeaderMap,
    extract::Json(payload): extract::Json<PatientSignupRequest>,
) -> Result<Json<Response>, (StatusCode, Json<Response>)> {
    let clinic = get_clinic_by_slug(&ctx.db, &slug) 
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(Response {status: "Couldn't get clinic".to_string()})))?
        .ok_or((StatusCode::NOT_FOUND, Json(Response { status: "Clinic not found".to_string()})))?;
    
    let sanitized = match sanitize_and_validate_payload(payload) {
        Ok(value) => value,
        Err(error) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(Response { status: error }),
            ));
        }
    };

    log::info!(
        "Signup request accepted for validation: email={}",
        sanitized.email,
    );

    let auth = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    let token = match auth.and_then(|h| h.strip_prefix("Bearer ")) {
        Some(t) if !t.is_empty() => t,
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(Response {
                    status: "Missing or invalid Bearer token".to_string(),
                }),
            ))
        }
    };

    log::info!("Token: {}", token);

    //password hashing
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let _password_hash = match argon2.hash_password(sanitized.password.as_bytes(), &salt) {
        Ok(hash) => hash,
        Err(e) => {
            log::error!("Error hashing password: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response {
                    status: "Couldn't hash password".to_string(),
                }),
            ));
        }
    }.to_string();

    log::info!("Password hash generated successfully");

    let birth_date = NaiveDate::parse_from_str(&sanitized.birth_date, "%Y-%m-%d")
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(Response { status: "Invalid birth date".to_string() })))?;

    let _id = create_patient(&ctx.db, CreatePatientData {
        clinic_id: clinic.id,
        first_name: sanitized.first_name,
        last_name: sanitized.last_name,
        birth_date,
        email: sanitized.email,
        password_hash: _password_hash,
        phone_number: Some(sanitized.phone_number),
        gender: Some(sanitized.gender),
        street_address: Some(sanitized.street_address),
        city: Some(sanitized.city),
        country: Some(sanitized.country),
        postcode: Some(sanitized.postcode), 
    })
    .await
    .map_err(|e| {
        match e {
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505")  => {
                (StatusCode::CONFLICT, Json(Response {status: "Email already used".to_string()}))
            }
            _ => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(Response {status: "Couldn't create patient".to_string()}))
            }
        }
    })?;
    log::info!("Patient created successfully");

    Ok(Json(Response {
        status: "ok".to_string(),
    }))
}
