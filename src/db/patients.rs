use sqlx::PgPool;
use uuid::Uuid;
use chrono::NaiveDate;

pub struct CreatePatientData {
    pub clinic_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: NaiveDate,
    pub email: String,
    pub password_hash: String,
    pub phone_number: Option<String>,
    pub gender: Option<String>,
    pub street_address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postcode: Option<String>,
}

pub struct Patient {
    pub id: Uuid,
    pub clinic_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: NaiveDate,
    pub email: String,
    pub password_hash: String,
    pub phone_number: Option<String>,
    pub gender: Option<String>,
    pub street_address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postcode: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_patient(pool: &PgPool, data: CreatePatientData) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar!(
        r#"
        INSERT INTO patients (
            clinic_id,
            first_name,
            last_name,
            birth_date,
            email,
            password_hash,
            phone_number,
            gender,
            street_address,
            city,
            country,
            postcode
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING id
        "#,
        data.clinic_id,
        data.first_name,
        data.last_name,
        data.birth_date,
        data.email,
        data.password_hash,
        data.phone_number,
        data.gender,
        data.street_address,
        data.city,
        data.country,
        data.postcode
    )
    .fetch_one(pool)
    .await?;

    Ok(id)

}

pub async fn get_patient_by_id(pool: &PgPool, id: Uuid, clinic_id: Uuid) -> Result<Option<Patient>, sqlx::Error> {
    let patient = sqlx::query_as!(
        Patient,
        r#"
        SELECT * FROM patients
        WHERE id = $1 AND clinic_id = $2
        "#,
        id,
        clinic_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(patient)
}

pub async fn get_patient_by_email(
    pool: &PgPool,
    email: &str,
    clinic_id: Uuid,
) -> Result<Option<Patient>, sqlx::Error> {
    let patient = sqlx::query_as!(
        Patient,
        r#"SELECT * FROM patients WHERE email = $1 AND clinic_id = $2"#,
        email,
        clinic_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(patient)
}