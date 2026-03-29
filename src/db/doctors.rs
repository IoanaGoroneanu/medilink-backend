use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct Doctor {
    pub id: Uuid,
    pub clinic_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub specialization: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub struct CreateDoctorData {
    pub clinic_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub specialization: Option<String>,
    pub phone_number: Option<String>,
}

pub async fn create_doctor(pool: &PgPool, data: CreateDoctorData) -> Result<Uuid, sqlx::Error> {
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO doctors (clinic_id, first_name, last_name, email, specialization, phone_number)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        data.clinic_id,
        data.first_name,
        data.last_name,
        data.email,
        data.specialization,
        data.phone_number
    )
    .fetch_one(pool)
    .await?;

    Ok(id)
}

pub async fn get_doctors_by_clinic(
    pool: &PgPool,
    clinic_id: Uuid,
) -> Result<Vec<Doctor>, sqlx::Error> {
    let doctors = sqlx::query_as!(
        Doctor,
        r#"SELECT * FROM doctors WHERE clinic_id = $1"#,
        clinic_id
    )
    .fetch_all(pool)
    .await?;

    Ok(doctors)
}